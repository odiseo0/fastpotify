use crate::i18n::{TextKey, Translator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TrayLabels {
    pub show_hide: &'static str,
    pub play_pause: &'static str,
    pub next: &'static str,
    pub previous: &'static str,
    pub quit: &'static str,
}

pub(crate) fn labels(translator: Translator, playing: bool) -> TrayLabels {
    TrayLabels {
        show_hide: translator.text(TextKey::TrayShowHide),
        play_pause: translator.text(if playing {
            TextKey::TrayPause
        } else {
            TextKey::TrayPlay
        }),
        next: translator.text(TextKey::TrayNext),
        previous: translator.text(TextKey::TrayPrevious),
        quit: translator.text(TextKey::TrayQuit),
    }
}

#[cfg(test)]
mod tests {
    use super::labels;
    use crate::i18n::Translator;
    use crate::settings::LanguageChoice;

    #[test]
    fn all_labels_follow_language_and_playback() {
        let english = Translator::new(LanguageChoice::English);
        assert_eq!(labels(english, false).play_pause, "Play");
        assert_eq!(labels(english, true).play_pause, "Pause");
        assert_eq!(labels(english, false).show_hide, "Show or hide Fastpotify");

        let spanish = Translator::new(LanguageChoice::Spanish);
        let stopped = labels(spanish, false);
        let playing = labels(spanish, true);
        assert_eq!(stopped.play_pause, "Reproducir");
        assert_eq!(playing.play_pause, "Pausar");
        assert_eq!(stopped.next, "Siguiente");
        assert_eq!(stopped.previous, "Anterior");
        assert_eq!(stopped.quit, "Salir");
    }
}
