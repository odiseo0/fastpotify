use super::{Message, TextKey};

pub(super) fn text(key: TextKey) -> &'static str {
    match key {
        TextKey::SettingsTitle => "Settings",
        TextKey::SettingsAccountHeading => "Account",
        TextKey::SettingsSpotifyPremium => "Spotify Premium",
        TextKey::SettingsSpotifyFreeNeedsPremium => "Spotify Free, local playback needs Premium",
        TextKey::SettingsSignOut => "Sign out",
        TextKey::SettingsPersonalAppLabel => "Personal Spotify app",
        TextKey::SettingsPersonalAppDetail => {
            "Use a personal Development Mode app for a separate API quota. The shared app stays active."
        }
        TextKey::SettingsClientIdHint => "Client ID",
        TextKey::SettingsCreateAppLabel => "Create an app",
        TextKey::SettingsCreateAppDetail => "Create one for free in Spotify's developer dashboard.",
        TextKey::SettingsSetupGuide => "Setup guide",
        TextKey::SettingsPersonalAppReadyLabel => "Personal app ready",
        TextKey::SettingsPersonalAppReadyDetail => {
            "Supported requests use your app. Other requests use the shared app."
        }
        TextKey::SettingsRemove => "Remove",
        TextKey::SettingsAuthorizePersonalAppLabel => "Authorize your personal app",
        TextKey::SettingsAuthorizePersonalAppDetail => {
            "Spotify opens in your browser to verify the account."
        }
        TextKey::SettingsAuthorize => "Authorize",
        TextKey::SettingsRemovePersonalAppLabel => "Remove personal app",
        TextKey::SettingsRemovePersonalAppDetail => "Shared access remains signed in.",
        TextKey::SettingsPlaybackHeading => "Playback on this computer",
        TextKey::SettingsPlaybackReady => "Ready",
        TextKey::SettingsPlaybackReadyDetail => "This computer is a Spotify Connect device.",
        TextKey::SettingsPlaybackSettingUp => "Setting up",
        TextKey::SettingsPlaybackSettingUpDetail => "Finish authorizing in your browser.",
        TextKey::SettingsPlaybackConnecting => "Connecting",
        TextKey::SettingsPlaybackConnectingDetail => "Connecting to Spotify…",
        TextKey::SettingsPlaybackUnavailable => "Unavailable",
        TextKey::SettingsPlaybackNotSetUp => "Not set up",
        TextKey::SettingsPlaybackNotSetUpDetail => {
            "Requires Spotify Premium and a one-time browser sign-in."
        }
        TextKey::SettingsTryAgain => "Try again",
        TextKey::SettingsEnablePlayback => "Enable playback here",
        TextKey::SettingsReconnect => "Reconnect",
        TextKey::SettingsDeviceNameLabel => "Device name",
        TextKey::SettingsDeviceNameDetail => "How this computer appears in Spotify Connect.",
        TextKey::SettingsAudioQualityLabel => "Audio quality",
        TextKey::SettingsAudioQualityDetail => "Higher bitrates use more data and cache space.",
        TextKey::SettingsQualityVeryHigh => "Very high · 320 kbps",
        TextKey::SettingsQualityHigh => "High · 160 kbps",
        TextKey::SettingsQualityNormal => "Normal · 96 kbps",
        TextKey::SettingsNormalizeVolumeLabel => "Normalize volume",
        TextKey::SettingsNormalizeVolumeDetail => "Keep loud and quiet tracks at a similar level.",
        TextKey::SettingsAutoplayLabel => "Autoplay",
        TextKey::SettingsAutoplayDetail => "Keep playing similar songs when your music ends.",
        TextKey::SettingsGaplessLabel => "Gapless playback",
        TextKey::SettingsGaplessDetail => "Play tracks without silence between them.",
        TextKey::SettingsKeepPlayingLabel => "Keep music playing when the window closes",
        TextKey::SettingsKeepPlayingDetailControl => {
            "Fastpotify hides to the system tray. Quit from the tray menu or with Ctrl+Q."
        }
        TextKey::SettingsKeepPlayingDetailCommand => {
            "Fastpotify hides to the system tray. Quit from the tray menu or with Cmd+Q."
        }
        TextKey::SettingsUpdateChecksLabel => "Automatic update checks",
        TextKey::SettingsUpdateChecksDetail => {
            "Checks GitHub once a day. No personal data is sent."
        }
        TextKey::SettingsAudioOutputLabel => "Audio output",
        TextKey::SettingsAudioOutputDetail => {
            "PulseAudio also covers PipeWire. Rodio talks to ALSA directly."
        }
        TextKey::SettingsOutputBufferLabel => "Output buffer",
        TextKey::SettingsOutputBufferDetail => {
            "More buffering can prevent clicks on busy computers. Less buffering makes controls respond sooner."
        }
        TextKey::SettingsAudioCacheLabel => "Audio cache",
        TextKey::SettingsAudioCacheDetail => "Save downloaded audio for later playback.",
        TextKey::SettingsApplyRestartPlayback => "Apply and restart playback",
        TextKey::SettingsRestartPlaybackDetail => "Restart local playback to apply these settings.",
        TextKey::SettingsPlaybackApplied => "Playback settings applied.",
        TextKey::SettingsAppearanceHeading => "Appearance",
        TextKey::SettingsLanguageLabel => "Language",
        TextKey::SettingsThemeLabel => "Theme",
        TextKey::SettingsThemeDark => "Dark",
        TextKey::SettingsThemeLight => "Light",
        TextKey::SettingsThemeSystem => "Follow system",
        TextKey::SettingsColourFromArtLabel => "Colour from album art",
        TextKey::SettingsColourFromArtDetail => {
            "Use the current cover's colour on pages and the player bar."
        }
        TextKey::SettingsCompactSidebarLabel => "Compact library sidebar",
        TextKey::SettingsCompactSidebarDetail => "Show names without covers in the sidebar.",
        TextKey::SettingsCompactTrackListLabel => "Compact track list",
        TextKey::SettingsCompactTrackListDetail => "Show each track on one line without a cover.",
        TextKey::SettingsZoomLabel => "Interface zoom",
        TextKey::SettingsZoomDetailControl => {
            "Ctrl+Plus and Ctrl+Minus work anywhere; Ctrl+0 resets."
        }
        TextKey::SettingsZoomDetailCommand => "Cmd+Plus and Cmd+Minus work anywhere; Cmd+0 resets.",
        TextKey::SettingsWinampHeading => "Winamp skins",
        TextKey::SettingsMiniPlayerLabel => "Mini player",
        TextKey::SettingsMiniPlayerDetailControl => {
            "Use classic Winamp .wsz skins. Press Ctrl+M or click the skin logo to return. Drop a skin on either window to add it."
        }
        TextKey::SettingsMiniPlayerDetailCommand => {
            "Use classic Winamp .wsz skins. Press Cmd+Shift+M or click the skin logo to return. Drop a skin on either window to add it."
        }
        TextKey::SettingsSwitchToIt => "Switch to it",
        TextKey::SettingsSkinLabel => "Skin",
        TextKey::SettingsSkinMuseum => "Skin Museum",
        TextKey::SettingsOpenFolder => "Open folder",
        TextKey::SettingsSkinSizeLabel => "Size",
        TextKey::SettingsSkinSizeDetail => "Whole-number scaling keeps skin pixels sharp.",
        TextKey::SettingsAlwaysOnTopLabel => "Always on top",
        TextKey::SettingsAlwaysOnTopDetail => "Keep the Winamp window above everything else.",
        TextKey::SettingsMilkdropHeading => "MilkDrop",
        TextKey::SettingsMilkdropWindowLabel => "MilkDrop window",
        TextKey::SettingsMilkdropWindowDetailControl => {
            "A projectM visualiser for local playback. Open it here, from the top bar, with Ctrl+Shift+K, or from the mini player's V menu. Press ? or F1 for its shortcuts."
        }
        TextKey::SettingsMilkdropWindowDetailCommand => {
            "A projectM visualiser for local playback. Open it here, from the top bar, with Cmd+Shift+K, or from the mini player's V menu. Press ? or F1 for its shortcuts."
        }
        TextKey::SettingsPresetsLabel => "Presets",
        TextKey::SettingsFetching => "Fetching...",
        TextKey::SettingsMilkdropOriginalPackNote => {
            "The 550 presets that shipped with MilkDrop 2; about 1 MB."
        }
        TextKey::SettingsMilkdropCropPackNote => {
            "Jason Fletcher's pick of 9,800 presets the community made; about 25 MB."
        }
        TextKey::SettingsTimePerPresetLabel => "Time per preset",
        TextKey::SettingsTimePerPresetDetail => {
            "How long each preset plays before the next fades in."
        }
        TextKey::SettingsFrameRateLabel => "Frame rate",
        TextKey::SettingsFrameRateDetail => {
            "Lower rates use fewer resources. Uncapped draws as fast as possible."
        }
        TextKey::SettingsResolutionLabel => "Resolution",
        TextKey::SettingsResolutionDetail => {
            "Half and Quarter use fewer resources and scale the image back up."
        }
        TextKey::SettingsResolutionFull => "Full",
        TextKey::SettingsResolutionHalf => "Half",
        TextKey::SettingsResolutionQuarter => "Quarter",
        TextKey::SettingsEqualizerHeading => "Equalizer",
        TextKey::SettingsEqualizerLabel => "Equalizer",
        TextKey::SettingsEqualizerDetail => {
            "A ten-band equalizer for playback on this computer. It does not affect other devices."
        }
        TextKey::SettingsPreampLabel => "Pre",
        TextKey::SettingsStorageHeading => "Storage",
        TextKey::SettingsArtworkCacheLabel => "Artwork cache",
        TextKey::SettingsClearArtwork => "Clear artwork",
        TextKey::SettingsPlayHistoryLabel => "Play history",
        TextKey::SettingsClearHistory => "Clear history",
        TextKey::SettingsSignInLabel => "Sign-in",
        TextKey::SettingsAboutHeading => "About",
        TextKey::SettingsAboutDetail => {
            "Built with Rust, egui, and librespot. Not affiliated with Spotify."
        }
        TextKey::SettingsCheckingUpdates => "Checking…",
        TextKey::SettingsCheckForUpdates => "Check for updates",
        TextKey::SettingsKeyboardShortcuts => "Keyboard shortcuts",
        TextKey::SettingsSourceCode => "Source code",
        TextKey::SettingsFpsUncapped => "Uncapped",
    }
}

pub(super) fn message(message: &Message) -> String {
    match message {
        Message::SettingsConnectedAs { username } => format!("Connected as {username}"),
        Message::SettingsPlaybackStatus { status } => format!("Status: {status}"),
        Message::SettingsSkinFolder { path } => {
            format!("Installed skins are in {path}. Find more at the Winamp Skin Museum.")
        }
        Message::SettingsPresetFolder { count, path } => {
            let count = match count {
                0 => "None yet".to_string(),
                1 => "One preset".to_string(),
                count => format!("{count} presets"),
            };
            format!(
                "{count} in {path}. Add .milk files here. Fastpotify downloads presets when MilkDrop first opens with an empty folder."
            )
        }
        Message::SettingsGetPresetPack { name } => format!("Get {name}"),
        Message::SettingsScreenRefreshRate { hz } => format!(
            "Your screen refreshes at {hz} Hz. Higher rates do not add visible frames. Uncapped draws as fast as possible."
        ),
        Message::SettingsStoredIn { path } => format!("Stored in {path}"),
        Message::SettingsHistoryStoredIn { path } => {
            format!("Tracks played here are stored in {path}. This file is never uploaded.")
        }
        Message::SettingsCredentialsStoredIn { path } => format!("Credentials are kept in {path}"),
        Message::SettingsVersion { version } => format!("Fastpotify {version}"),
        Message::SettingsFps { rate } => format!("{rate} fps"),
        Message::SettingsFpsYourScreen { rate } => format!("{rate} fps, your screen"),
    }
}
