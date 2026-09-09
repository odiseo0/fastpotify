//! Small helpers shared across the application.

use crate::i18n::{Message, TextKey, Translator};
use crate::settings::LanguageChoice;

/// `3:45` for track lengths, `1:02:03` past an hour.
pub fn format_duration_ms(ms: u32) -> String {
    let total = ms / 1000;
    let hours = total / 3600;
    let minutes = (total / 60) % 60;
    let seconds = total % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// `2 hr 13 min` for playlist totals, `45 min 12 sec` under an hour.
pub fn format_total_ms(ms: u64) -> String {
    format_total_ms_with(Translator::new(LanguageChoice::English), ms)
}

pub fn format_total_ms_with(translator: Translator, ms: u64) -> String {
    let total = ms / 1000;
    let hours = total / 3600;
    let minutes = (total / 60) % 60;
    let seconds = total % 60;
    if hours > 0 {
        translator.message(&Message::DurationHoursMinutes { hours, minutes })
    } else if minutes > 0 {
        translator.message(&Message::DurationMinutesSeconds { minutes, seconds })
    } else {
        translator.message(&Message::DurationSeconds { seconds })
    }
}

/// Episode lengths read as `1 hr 12 min` or `38 min`.
pub fn format_episode_ms(ms: u32) -> String {
    format_episode_ms_with(Translator::new(LanguageChoice::English), ms)
}

pub fn format_episode_ms_with(translator: Translator, ms: u32) -> String {
    let minutes = ms / 60_000;
    let hours = minutes / 60;
    if hours > 0 {
        translator.message(&Message::EpisodeHoursMinutes {
            hours,
            minutes: minutes % 60,
        })
    } else {
        translator.message(&Message::EpisodeMinutes {
            minutes: minutes.max(1),
        })
    }
}

pub fn format_count(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(character);
    }
    out
}

/// `Jan 5, 2024` from an ISO-8601 timestamp or a bare date.
pub fn format_date(iso: &str) -> String {
    format_date_with(Translator::new(LanguageChoice::English), iso)
}

pub fn format_date_with(translator: Translator, iso: &str) -> String {
    let date = iso.get(..10).unwrap_or(iso);
    let mut parts = date.split('-');
    let (Some(year), Some(month)) = (parts.next(), parts.next()) else {
        return iso.to_string();
    };
    let day = parts.next();
    let month_key = match month {
        "01" => TextKey::DateMonthJan,
        "02" => TextKey::DateMonthFeb,
        "03" => TextKey::DateMonthMar,
        "04" => TextKey::DateMonthApr,
        "05" => TextKey::DateMonthMay,
        "06" => TextKey::DateMonthJun,
        "07" => TextKey::DateMonthJul,
        "08" => TextKey::DateMonthAug,
        "09" => TextKey::DateMonthSep,
        "10" => TextKey::DateMonthOct,
        "11" => TextKey::DateMonthNov,
        "12" => TextKey::DateMonthDec,
        _ => return iso.to_string(),
    };
    let month = translator.text(month_key).to_string();
    match day.and_then(|day| day.trim_start_matches('0').parse::<u8>().ok()) {
        Some(day) => translator.message(&Message::DateDay {
            month,
            day,
            year: year.to_string(),
        }),
        None => translator.message(&Message::DateMonthYear {
            month,
            year: year.to_string(),
        }),
    }
}

/// `5 minutes ago` for recent ISO-8601 timestamps, otherwise the usual date.
///
/// Dates are shown relatively for their first 30 days, matching the playlist
/// table's compact, time-aware presentation. `now` is an argument so callers
/// can render against one instant and the boundary behaviour stays testable.
pub fn format_relative_date(iso: &str, now: jiff::Timestamp) -> String {
    format_relative_date_with(Translator::new(LanguageChoice::English), iso, now)
}

pub fn format_relative_date_with(
    translator: Translator,
    iso: &str,
    now: jiff::Timestamp,
) -> String {
    let Ok(added) = iso.parse::<jiff::Timestamp>() else {
        return format_date_with(translator, iso);
    };
    let seconds = added.duration_until(now).as_secs_f64().floor() as i64;
    if !(0..30 * 24 * 60 * 60).contains(&seconds) {
        return format_date_with(translator, iso);
    }

    let message = if seconds < 60 {
        Message::RelativeSeconds { count: seconds }
    } else if seconds < 60 * 60 {
        Message::RelativeMinutes {
            count: seconds / 60,
        }
    } else if seconds < 24 * 60 * 60 {
        Message::RelativeHours {
            count: seconds / (60 * 60),
        }
    } else if seconds < 7 * 24 * 60 * 60 {
        Message::RelativeDays {
            count: seconds / (24 * 60 * 60),
        }
    } else {
        Message::RelativeWeeks {
            count: seconds / (7 * 24 * 60 * 60),
        }
    };
    translator.message(&message)
}

pub fn relative_date_is_live(iso: &str, now: jiff::Timestamp) -> bool {
    let Ok(added) = iso.parse::<jiff::Timestamp>() else {
        return false;
    };
    let seconds = added.duration_until(now).as_secs_f64().floor() as i64;
    (0..30 * 24 * 60 * 60).contains(&seconds)
}

/// Tears the id out of `spotify:track:abc` and friends.
pub fn uri_id(uri: &str) -> Option<&str> {
    uri.rsplit(':').next().filter(|id| !id.is_empty())
}

pub fn uri_kind(uri: &str) -> Option<&str> {
    let mut parts = uri.split(':');
    parts.next()?;
    parts.next()
}

pub fn open_spotify_url(uri: &str) -> Option<String> {
    let kind = uri_kind(uri)?;
    let id = uri_id(uri)?;
    Some(format!("https://open.spotify.com/{kind}/{id}"))
}

/// The application icon, drawn at runtime: a green disc with a play mark.
/// Shared by the window icon and the tray pixmap.
/// The menu-bar shape for macOS: the circle with the play triangle punched
/// out. macOS template images use only the alpha channel and paint the
/// shape themselves, black in a light menu bar and white in a dark one.
pub fn tray_template_rgba(size: usize) -> Vec<u8> {
    let mut rgba = app_icon_rgba(size);
    for pixel in rgba.as_chunks_mut::<4>().0 {
        // The triangle is the dark colour; make it a hole instead.
        if pixel[1] < 128 {
            pixel[3] = 0;
        }
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
    }
    rgba
}

/// The mark rasterised to pixels for the window icon and the trays,
/// where no egui painter exists. This is deliberately the one separate
/// implementation of the logo; on-screen drawing goes through
/// `theme::logo` and `theme::play_glyph_offset` instead.
pub fn app_icon_rgba(size: usize) -> Vec<u8> {
    let mut rgba = vec![0u8; size * size * 4];
    let center = size as f32 / 2.0;
    let radius = center - 2.0;
    let scale = size as f32 / 128.0;
    let triangle = [
        (center - 12.0 * scale, center - 22.0 * scale),
        (center - 12.0 * scale, center + 22.0 * scale),
        (center + 26.0 * scale, center),
    ];
    let sign = |a: (f32, f32), b: (f32, f32), c: (f32, f32)| {
        (a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1)
    };
    for y in 0..size {
        for x in 0..size {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let distance = ((px - center).powi(2) + (py - center).powi(2)).sqrt();
            let coverage = (radius - distance + 0.5).clamp(0.0, 1.0);
            if coverage <= 0.0 {
                continue;
            }
            let d1 = sign((px, py), triangle[0], triangle[1]);
            let d2 = sign((px, py), triangle[1], triangle[2]);
            let d3 = sign((px, py), triangle[2], triangle[0]);
            let negative = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
            let positive = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
            let inside = !(negative && positive);
            let (r, g, b) = if inside { (10, 20, 14) } else { (30, 215, 96) };
            let index = (y * size + x) * 4;
            rgba[index] = r;
            rgba[index + 1] = g;
            rgba[index + 2] = b;
            rgba[index + 3] = (coverage * 255.0) as u8;
        }
    }
    rgba
}

pub fn greeting() -> &'static str {
    match local_hour() {
        5..=11 => "Good morning",
        12..=17 => "Good afternoon",
        _ => "Good evening",
    }
}

pub fn greeting_text_key() -> crate::i18n::TextKey {
    match local_hour() {
        5..=11 => crate::i18n::TextKey::GreetingMorning,
        12..=17 => crate::i18n::TextKey::GreetingAfternoon,
        _ => crate::i18n::TextKey::GreetingEvening,
    }
}

fn local_hour() -> u8 {
    jiff::Zoned::now().hour() as u8
}

/// Strips the HTML Spotify embeds in playlist descriptions.
pub fn strip_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for character in text.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(character),
            _ => {}
        }
    }
    out.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&#x2F;", "/")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

/// Atomically replaces `path` with `temporary` on the current platform.
#[cfg(not(windows))]
pub(crate) fn replace_file(
    temporary: &std::path::Path,
    path: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::rename(temporary, path)
}

/// Atomically replaces `path` with `temporary` on Windows.
#[cfg(windows)]
pub(crate) fn replace_file(
    temporary: &std::path::Path,
    path: &std::path::Path,
) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let temporary: Vec<u16> = temporary.as_os_str().encode_wide().chain(Some(0)).collect();
    let path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let moved = unsafe {
        MoveFileExW(
            temporary.as_ptr(),
            path.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if moved == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn translator(language: LanguageChoice) -> Translator {
        Translator::new(language)
    }

    #[test]
    fn durations() {
        assert_eq!(format_duration_ms(225_000), "3:45");
        assert_eq!(format_duration_ms(3_723_000), "1:02:03");
        assert_eq!(format_total_ms(7_980_000), "2 hr 13 min");
        assert_eq!(format_total_ms(2_712_000), "45 min 12 sec");
        assert_eq!(format_episode_ms(4_320_000), "1 hr 12 min");
    }

    #[test]
    fn counts_and_dates() {
        assert_eq!(format_count(1_234_567), "1,234,567");
        assert_eq!(format_count(12), "12");
        assert_eq!(format_date("2024-01-05T10:00:00Z"), "Jan 5, 2024");
        assert_eq!(format_date("2024-03"), "Mar 2024");
        assert_eq!(format_date("2024"), "2024");
    }

    #[test]
    fn recent_dates_are_relative_for_the_first_month() {
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();
        assert_eq!(
            format_relative_date("2026-08-31T11:59:30Z", now),
            "30 seconds ago"
        );
        assert_eq!(
            format_relative_date("2026-08-31T11:59:00Z", now),
            "1 minute ago"
        );
        assert_eq!(
            format_relative_date("2026-08-31T11:00:00Z", now),
            "1 hour ago"
        );
        assert_eq!(
            format_relative_date("2026-08-30T12:00:00Z", now),
            "1 day ago"
        );
        assert_eq!(
            format_relative_date("2026-08-17T12:00:00Z", now),
            "2 weeks ago"
        );
        assert_eq!(
            format_relative_date("2026-08-01T12:00:00Z", now),
            "Aug 1, 2026"
        );
    }

    #[test]
    fn relative_dates_fall_back_for_future_and_invalid_timestamps() {
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();
        assert_eq!(
            format_relative_date("2026-09-01T12:00:00Z", now),
            "Sep 1, 2026"
        );
        assert_eq!(format_relative_date("not-a-date", now), "not-a-date");
    }

    #[test]
    fn translated_duration_helpers_keep_values_and_use_the_catalogue() {
        let spanish = translator(LanguageChoice::Spanish);

        let seconds = format_total_ms_with(spanish, 0);
        assert!(seconds.contains('0'));
        let minute = format_total_ms_with(spanish, 60_000);
        assert!(minute.contains('1'));
        let multiple = format_total_ms_with(spanish, 7_980_000);
        assert!(multiple.contains("2"));
        assert!(multiple.contains("13"));
        let episode = format_episode_ms_with(spanish, 4_320_000);
        assert!(episode.contains("1"));
        assert!(episode.contains("12"));
    }

    #[test]
    fn translated_dates_keep_source_values_and_live_state() {
        let spanish = translator(LanguageChoice::Spanish);
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();

        let date = format_date_with(spanish, "2024-01-05T10:00:00Z");
        assert!(date.contains('5'));
        assert!(date.contains("2024"));
        let relative = format_relative_date_with(spanish, "2026-08-31T11:59:00Z", now);
        assert!(relative.contains('1'));
        assert!(relative_date_is_live("2026-08-31T11:59:00Z", now));
        assert!(!relative_date_is_live("2026-08-01T12:00:00Z", now));
        assert!(!relative_date_is_live("2026-09-01T12:00:00Z", now));
        assert_eq!(format_date_with(spanish, "not-a-date"), "not-a-date");
    }

    #[test]
    fn uris() {
        assert_eq!(uri_id("spotify:track:abc"), Some("abc"));
        assert_eq!(uri_kind("spotify:playlist:x"), Some("playlist"));
        assert_eq!(
            open_spotify_url("spotify:album:z").as_deref(),
            Some("https://open.spotify.com/album/z")
        );
    }

    #[test]
    fn html_is_stripped() {
        assert_eq!(
            strip_html("Hi <a href=\"x\">there</a> &amp; you"),
            "Hi there & you"
        );
        assert_eq!(strip_html("ONE&#x2F;TWO&#x2F;THREE"), "ONE/TWO/THREE");
    }
}
