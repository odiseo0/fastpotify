//! Compile-time catalogues for Fastpotify-owned interface text.

use crate::settings::LanguageChoice;

mod en;
mod es;

macro_rules! text_keys {
    ($macro:ident) => {
        $macro! {
            SettingsTitle,
            SettingsAccountHeading,
            SettingsSpotifyPremium,
            SettingsSpotifyFreeNeedsPremium,
            SettingsSignOut,
            SettingsPersonalAppLabel,
            SettingsPersonalAppDetail,
            SettingsClientIdHint,
            SettingsCreateAppLabel,
            SettingsCreateAppDetail,
            SettingsSetupGuide,
            SettingsPersonalAppReadyLabel,
            SettingsPersonalAppReadyDetail,
            SettingsRemove,
            SettingsAuthorizePersonalAppLabel,
            SettingsAuthorizePersonalAppDetail,
            SettingsAuthorize,
            SettingsRemovePersonalAppLabel,
            SettingsRemovePersonalAppDetail,
            SettingsPlaybackHeading,
            SettingsPlaybackReady,
            SettingsPlaybackReadyDetail,
            SettingsPlaybackSettingUp,
            SettingsPlaybackSettingUpDetail,
            SettingsPlaybackConnecting,
            SettingsPlaybackConnectingDetail,
            SettingsPlaybackUnavailable,
            SettingsPlaybackNotSetUp,
            SettingsPlaybackNotSetUpDetail,
            SettingsTryAgain,
            SettingsEnablePlayback,
            SettingsReconnect,
            SettingsDeviceNameLabel,
            SettingsDeviceNameDetail,
            SettingsAudioQualityLabel,
            SettingsAudioQualityDetail,
            SettingsQualityVeryHigh,
            SettingsQualityHigh,
            SettingsQualityNormal,
            SettingsNormalizeVolumeLabel,
            SettingsNormalizeVolumeDetail,
            SettingsAutoplayLabel,
            SettingsAutoplayDetail,
            SettingsGaplessLabel,
            SettingsGaplessDetail,
            SettingsKeepPlayingLabel,
            SettingsKeepPlayingDetailControl,
            SettingsKeepPlayingDetailCommand,
            SettingsUpdateChecksLabel,
            SettingsUpdateChecksDetail,
            SettingsAudioOutputLabel,
            SettingsAudioOutputDetail,
            SettingsOutputBufferLabel,
            SettingsOutputBufferDetail,
            SettingsAudioCacheLabel,
            SettingsAudioCacheDetail,
            SettingsApplyRestartPlayback,
            SettingsRestartPlaybackDetail,
            SettingsPlaybackApplied,
            SettingsAppearanceHeading,
            SettingsLanguageLabel,
            SettingsThemeLabel,
            SettingsThemeDark,
            SettingsThemeLight,
            SettingsThemeSystem,
            SettingsColourFromArtLabel,
            SettingsColourFromArtDetail,
            SettingsCompactSidebarLabel,
            SettingsCompactSidebarDetail,
            SettingsCompactTrackListLabel,
            SettingsCompactTrackListDetail,
            SettingsZoomLabel,
            SettingsZoomDetailControl,
            SettingsZoomDetailCommand,
            SettingsWinampHeading,
            SettingsMiniPlayerLabel,
            SettingsMiniPlayerDetailControl,
            SettingsMiniPlayerDetailCommand,
            SettingsSwitchToIt,
            SettingsSkinLabel,
            SettingsSkinMuseum,
            SettingsOpenFolder,
            SettingsSkinSizeLabel,
            SettingsSkinSizeDetail,
            SettingsAlwaysOnTopLabel,
            SettingsAlwaysOnTopDetail,
            SettingsMilkdropHeading,
            SettingsMilkdropWindowLabel,
            SettingsMilkdropWindowDetailControl,
            SettingsMilkdropWindowDetailCommand,
            SettingsPresetsLabel,
            SettingsFetching,
            SettingsMilkdropOriginalPackNote,
            SettingsMilkdropCropPackNote,
            SettingsTimePerPresetLabel,
            SettingsTimePerPresetDetail,
            SettingsFrameRateLabel,
            SettingsFrameRateDetail,
            SettingsResolutionLabel,
            SettingsResolutionDetail,
            SettingsResolutionFull,
            SettingsResolutionHalf,
            SettingsResolutionQuarter,
            SettingsEqualizerHeading,
            SettingsEqualizerLabel,
            SettingsEqualizerDetail,
            SettingsPreampLabel,
            SettingsStorageHeading,
            SettingsArtworkCacheLabel,
            SettingsClearArtwork,
            SettingsPlayHistoryLabel,
            SettingsClearHistory,
            SettingsSignInLabel,
            SettingsAboutHeading,
            SettingsAboutDetail,
            SettingsCheckingUpdates,
            SettingsCheckForUpdates,
            SettingsKeyboardShortcuts,
            SettingsSourceCode,
            SettingsFpsUncapped,
            CommonPlay,
            CommonPause,
            CommonMore,
            CommonStarting,
            CommonFollow,
            CommonFollowing,
            CommonShowLess,
            CommonSeeMore,
            CommonLoadMore,
            CommonRetry,
            CommonClose,
            CommonCancel,
            CommonTryAgain,
            CommonSettings,
            CommonKeyboardShortcuts,
            CommonSignOut,
            CommonHome,
            CommonBack,
            CommonForward,
            CommonQueue,
            CommonLyrics,
            CommonArtist,
            CommonAlbum,
            CommonPlaylist,
            CommonPodcast,
            CommonEpisode,
            CommonAll,
            LoginTagline,
            LoginWaitingBrowser,
            LoginOpenAgain,
            LoginConnecting,
            LoginUseSharedApp,
            LoginSignIn,
            LoginPrivacyDetail,
            LyricsFollow,
            LyricsNothingPlaying,
            LyricsPlaySong,
            LyricsNoLyrics,
            LyricsNoLyricsDetail,
            LyricsInstrumental,
            LyricsInstrumentalDetail,
            DevicesSettingUp,
            DevicesSetUpPlayback,
            DevicesConnecting,
            DevicesNetworkReceiver,
            DevicesHeading,
            DevicesRefresh,
            DevicesNoneFound,
            DevicesListeningHere,
            DevicesRestricted,
            DevicesPlayHere,
            QueueRecentTab,
            QueueSavePlaylist,
            QueueClear,
            QueueNowPlaying,
            QueueNothingQueued,
            QueueNothingQueuedDetail,
            QueuePlayingNext,
            QueueNextUp,
            QueueNoRecentPlays,
            QueueNoRecentPlaysDetail,
            PlayerNothingPlaying,
            PlayerPickSomething,
            PlayerRemoveLiked,
            PlayerSaveLiked,
            PlayerShuffle,
            PlayerShuffleOn,
            PlayerPrevious,
            PlayerNext,
            PlayerRepeat,
            PlayerRepeatOne,
            PlayerRepeatOff,
            PlayerPositionAccessibility,
            PlayerVolumeAccessibility,
            PlayerUnmute,
            PlayerMute,
            PlayerConnectDevice,
            TopbarShowSidebarControl,
            TopbarShowSidebarCommand,
            TopbarSearchHint,
            TopbarMilkdropControl,
            TopbarMilkdropCommand,
            TopbarWinampControl,
            TopbarWinampCommand,
            TopbarWaitingSpotify,
            TopbarAnotherDevice,
            CommonSongs,
            CommonArtists,
            CommonAlbums,
            CommonPlaylists,
            CommonPodcasts,
            CommonEpisodes,
            CommonLikedSongs,
            CommonRemoveLibrary,
            CommonAddLibrary,
            AlbumKindSingle,
            AlbumKindCompilation,
            AlbumKindAppearsOn,
            AlbumKindAlbum,
            DiscographyFilterAll,
            DiscographyFilterAlbums,
            DiscographyFilterSingles,
            DiscographyFilterAppearsOn,
            ArtistPopular,
            ArtistNoPopularSongs,
            ArtistDiscography,
            ArtistNothingCategory,
            ArtistFansAlsoLike,
            ShowPlayLatest,
            ShowFollowPodcast,
            ShowAbout,
            ShowAllEpisodes,
            ShowPlayed,
            HomeMadeForYou,
            HomeShelfLoadError,
            HomeRecentlyPlayed,
            HomeTopArtists,
            HomeTopSongs,
            HomeShowMoreTopSongs,
            HomeRecommended,
            SearchNoResultsDetail,
            SearchSpotify,
            SearchSpotifyDetail,
            SearchRecent,
            SearchTopResult,
            LibraryNoSavedAlbums,
            LibrarySavedAlbumsDetail,
            LibraryNoFollowedArtists,
            LibraryFollowedArtistsDetail,
            LibraryNoPodcasts,
            LibraryFollowedPodcastsDetail,
            LibraryNoSavedEpisodes,
            LibrarySavedEpisodesDetail,
            GreetingMorning,
            GreetingAfternoon,
            GreetingEvening,
            DialogDeletePlaylist,
            DialogRemoveLibrary,
            DialogDelete,
            DialogRemove,
            DialogSongsAlreadyPlaylist,
            DialogSongAlreadyPlaylist,
            DialogAddAnyway,
            DialogDone,
            DialogPremiumTitle,
            DialogPremiumDetail,
            DialogOk,
            DialogNewPlaylist,
            DialogName,
            DialogPlaylistNameHint,
            DialogPublicPlaylist,
            DialogCreate,
            DialogEditDetails,
            DialogDescription,
            DialogOptionalDescription,
            DialogSave,
            CommonSearch,
            SidebarLibrary,
            SidebarHideControl,
            SidebarHideCommand,
            SidebarCreatePlaylist,
            SidebarSearchLibrary,
            SidebarNothingHere,
            SidebarNoMatches,
            SidebarUnpin,
            SidebarPinTop,
            SidebarSortRecent,
            SidebarHome,
            SidebarSearch,
            SidebarDefaultFolder,
            SidebarFilterPlaylists,
            SidebarFilterAlbums,
            SidebarFilterArtists,
            SidebarFilterPodcasts,
            SidebarLikedSongs,
            SidebarPlaylistKind,
            SidebarArtistKind,
            SidebarPlay,
            MenuPlayNext,
            MenuAddPlaylist,
            MenuNewPlaylist,
            MenuSaveEpisode,
            MenuMoveUp,
            MenuMoveDown,
            MenuRemovePlaylist,
            MenuSongRadio,
            MenuGoArtist,
            MenuGoAlbum,
            MenuGoPodcast,
            MenuCopyLink,
            MenuOpenSpotify,
            MenuShufflePlay,
            MenuUnfollow,
            TableSortPlaylistOrder,
            TableOriginalOrderReversed,
            TableAddedBy,
            TableDateAdded,
            TableSortDuration,
            TableTitle,
            TableAlbum,
            CollectionShuffleOff,
            CollectionFilter,
            CollectionGoSong,
            CollectionGo,
            CollectionNothingHere,
            CollectionAddedSongs,
            CollectionTopSongsDetail,
            CollectionCollaborativePlaylist,
            CollectionPublicPlaylist,
            ShortcutPlayPause,
            ShortcutPreviousNext,
            ShortcutSeek,
            ShortcutVolume,
            ShortcutMute,
            ShortcutLikePlaying,
            ShortcutShuffle,
            ShortcutRepeat,
            ShortcutQueue,
            ShortcutLyrics,
            ShortcutSearch,
            ShortcutSidebar,
            ShortcutBackForward,
            ShortcutHome,
            ShortcutLikedSongs,
            ShortcutPlayingArtist,
            ShortcutPlayingAlbum,
            ShortcutWinamp,
            ShortcutMilkdrop,
            ShortcutMilkdropFullscreen,
            ShortcutMilkdropNext,
            ShortcutMilkdropPrevious,
            ShortcutMilkdropKeep,
            ShortcutMilkdropClose,
            ShortcutSettings,
            ShortcutHelp,
            ShortcutCloseWindow,
            ShortcutQuit,
            MenuSelectionRemoveLiked,
            MenuSelectionSaveLiked,
            MenuTrackRemoveLiked,
            MenuTrackSaveLiked,
            TrackRowRemoveLiked,
            TrackRowSaveLiked,
            TrackRowMore,
            CollectionStarting,
            CollectionPlay,
            CollectionPause,
            CollectionPlaylistKind,
            CollectionLikedSongs,
            CollectionPlaylistAddLibrary,
            CollectionPlaylistRemoveLibrary,
            CollectionAlbumSaveLibrary,
            CollectionAlbumRemoveLibrary
        }
    };
}

macro_rules! define_text_key {
    ($($key:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum TextKey {
            $($key),+
        }

        impl TextKey {
            pub const ALL: &'static [Self] = &[$(Self::$key),+];
        }
    };
}

text_keys!(define_text_key);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    SettingsConnectedAs {
        username: String,
    },
    SettingsPlaybackStatus {
        status: String,
    },
    SettingsSkinFolder {
        path: String,
    },
    SettingsPresetFolder {
        count: usize,
        path: String,
    },
    SettingsGetPresetPack {
        name: String,
    },
    SettingsScreenRefreshRate {
        hz: u32,
    },
    SettingsStoredIn {
        path: String,
    },
    SettingsHistoryStoredIn {
        path: String,
    },
    SettingsCredentialsStoredIn {
        path: String,
    },
    SettingsVersion {
        version: String,
    },
    SettingsFps {
        rate: u32,
    },
    SettingsFpsYourScreen {
        rate: u32,
    },
    LoginFooter {
        version: String,
    },
    LyricsFetchFailed {
        detail: String,
    },
    DeviceThisComputer {
        name: String,
    },
    PlayingOnDevice {
        name: String,
    },
    UpdateToVersion {
        version: String,
    },
    UpdateAvailableDetail {
        version: String,
    },
    FollowerCount {
        count: u64,
    },
    EpisodeCount {
        count: u32,
    },
    EpisodeTimeLeft {
        time: String,
    },
    SearchNoResults {
        query: String,
    },
    SearchSongBy {
        artist: String,
    },
    SearchAlbumBy {
        artist: String,
    },
    SearchPlaylistBy {
        owner: String,
    },
    SearchPodcastBy {
        publisher: String,
    },
    ByName {
        name: String,
    },
    AlbumYearKind {
        year: String,
        kind: String,
    },
    DeletePlaylistDetail {
        name: String,
    },
    RemovePlaylistDetail {
        name: String,
    },
    PlaylistSongsAdded {
        count: usize,
    },
    DuplicateSongs {
        playlist_name: String,
        names: Vec<String>,
        selected_count: usize,
    },
    SidebarFolderPlaylistCount {
        count: usize,
    },
    SidebarPlaylistSongCount {
        count: u32,
    },
    SidebarPlaylistBy {
        owner: String,
    },
    SidebarPodcastBy {
        publisher: String,
    },
    SidebarAlbumBy {
        kind: String,
        artists: String,
    },
    SidebarFolderState {
        name: String,
        collapsed: bool,
    },
    SidebarPlayItem {
        name: String,
    },
    MenuSelectionCount {
        count: usize,
    },
    SortBy {
        label: String,
    },
    CollectionNamedContributors {
        names: Vec<String>,
    },
    CollectionOtherContributors {
        count: usize,
    },
    CollectionSongCount {
        count: u64,
    },
    CollectionSongCountDuration {
        count: u64,
        duration: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Translator {
    language: LanguageChoice,
}

impl Translator {
    pub fn new(language: LanguageChoice) -> Self {
        Self { language }
    }

    pub fn language(self) -> LanguageChoice {
        self.language
    }

    pub fn text(self, key: TextKey) -> &'static str {
        match self.language {
            LanguageChoice::English => en::text(key),
            LanguageChoice::Spanish => es::text(key),
        }
    }

    pub fn message(self, message: &Message) -> String {
        match self.language {
            LanguageChoice::English => en::message(message),
            LanguageChoice::Spanish => es::message(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, TextKey, Translator};
    use crate::settings::LanguageChoice;

    #[test]
    fn every_text_key_resolves_in_both_catalogues() {
        for language in LanguageChoice::ALL {
            let translator = Translator::new(language);
            for key in TextKey::ALL {
                assert!(!translator.text(*key).is_empty(), "{language:?} {key:?}");
            }
        }
    }

    #[test]
    fn every_message_resolves_and_keeps_dynamic_values() {
        let messages = [
            Message::SettingsConnectedAs {
                username: "Pérez".into(),
            },
            Message::SettingsPlaybackStatus {
                status: "Ready".into(),
            },
            Message::SettingsSkinFolder {
                path: "C:/Skins/日本".into(),
            },
            Message::SettingsPresetFolder {
                count: 0,
                path: "/tmp/cero".into(),
            },
            Message::SettingsPresetFolder {
                count: 1,
                path: "/tmp/uno".into(),
            },
            Message::SettingsPresetFolder {
                count: 4,
                path: "/tmp/varios".into(),
            },
            Message::SettingsGetPresetPack {
                name: "Pack 7".into(),
            },
            Message::SettingsScreenRefreshRate { hz: 144 },
            Message::SettingsStoredIn {
                path: "/tmp/art".into(),
            },
            Message::SettingsHistoryStoredIn {
                path: "/tmp/history.json".into(),
            },
            Message::SettingsCredentialsStoredIn {
                path: "/tmp/credentials".into(),
            },
            Message::SettingsVersion {
                version: "9.8.7-test".into(),
            },
            Message::SettingsFps { rate: 75 },
            Message::SettingsFpsYourScreen { rate: 120 },
        ];
        let values = [
            "Pérez",
            "Ready",
            "日本",
            "cero",
            "uno",
            "varios",
            "Pack 7",
            "144",
            "art",
            "history.json",
            "credentials",
            "9.8.7-test",
            "75",
            "120",
        ];
        for language in LanguageChoice::ALL {
            let translator = Translator::new(language);
            for (message, value) in messages.iter().zip(values) {
                assert!(translator.message(message).contains(value));
            }
        }
    }

    #[test]
    fn preset_counts_use_only_one_as_singular() {
        let translator = Translator::new(LanguageChoice::English);
        assert!(
            translator
                .message(&Message::SettingsPresetFolder {
                    count: 0,
                    path: "p".into()
                })
                .starts_with("None yet")
        );
        assert!(
            translator
                .message(&Message::SettingsPresetFolder {
                    count: 1,
                    path: "p".into()
                })
                .starts_with("One preset")
        );
        assert!(
            translator
                .message(&Message::SettingsPresetFolder {
                    count: 2,
                    path: "p".into()
                })
                .starts_with("2 presets")
        );
    }

    #[test]
    fn representative_english_text_stays_exact() {
        let translator = Translator::new(LanguageChoice::English);
        assert_eq!(translator.text(TextKey::SettingsTitle), "Settings");
        assert_eq!(
            translator.text(TextKey::SettingsThemeSystem),
            "Follow system"
        );
        assert_eq!(
            translator.text(TextKey::SettingsCheckForUpdates),
            "Check for updates"
        );
    }

    #[test]
    fn spanish_catalogue_has_no_review_markers() {
        assert!(!include_str!("i18n/es.rs").contains("TODO(es)"));
    }
}
