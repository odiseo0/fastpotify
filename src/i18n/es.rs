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
        TextKey::CommonPlay => "TODO(es) Play",
        TextKey::CommonPause => "TODO(es) Pause",
        TextKey::CommonMore => "TODO(es) More",
        TextKey::CommonStarting => "TODO(es) Starting…",
        TextKey::CommonFollow => "TODO(es) Follow",
        TextKey::CommonFollowing => "TODO(es) Following",
        TextKey::CommonShowLess => "TODO(es) Show less",
        TextKey::CommonSeeMore => "TODO(es) See more",
        TextKey::CommonLoadMore => "TODO(es) Load more",
        TextKey::CommonRetry => "TODO(es) Retry",
        TextKey::CommonClose => "TODO(es) Close",
        TextKey::CommonCancel => "TODO(es) Cancel",
        TextKey::CommonTryAgain => "TODO(es) Try again",
        TextKey::CommonSettings => "TODO(es) Settings",
        TextKey::CommonKeyboardShortcuts => "TODO(es) Keyboard shortcuts",
        TextKey::CommonSignOut => "TODO(es) Sign out",
        TextKey::CommonHome => "TODO(es) Home",
        TextKey::CommonBack => "TODO(es) Back",
        TextKey::CommonForward => "TODO(es) Forward",
        TextKey::CommonQueue => "TODO(es) Queue",
        TextKey::CommonLyrics => "TODO(es) Lyrics",
        TextKey::CommonArtist => "TODO(es) Artist",
        TextKey::CommonAlbum => "TODO(es) Album",
        TextKey::CommonPlaylist => "TODO(es) Playlist",
        TextKey::CommonPodcast => "TODO(es) Podcast",
        TextKey::CommonEpisode => "TODO(es) Episode",
        TextKey::CommonAll => "TODO(es) All",
        TextKey::LoginTagline => "TODO(es) A native Spotify client.",
        TextKey::LoginWaitingBrowser => "TODO(es) Waiting for Spotify in your browser…",
        TextKey::LoginOpenAgain => "TODO(es) Didn't open? Open the sign-in page again",
        TextKey::LoginConnecting => "TODO(es) Connecting to Spotify…",
        TextKey::LoginUseSharedApp => "TODO(es) Use the shared Spotify app instead",
        TextKey::LoginSignIn => "TODO(es) Sign in with Spotify",
        TextKey::LoginPrivacyDetail => {
            "TODO(es) Sign in through your browser. Fastpotify never sees your password. Local playback needs Spotify Premium."
        }
        TextKey::LyricsFollow => "TODO(es) Follow",
        TextKey::LyricsNothingPlaying => "TODO(es) Nothing playing",
        TextKey::LyricsPlaySong => "TODO(es) Play a song to see its lyrics.",
        TextKey::LyricsNoLyrics => "TODO(es) No lyrics",
        TextKey::LyricsNoLyricsDetail => "TODO(es) No lyrics found for this track.",
        TextKey::LyricsInstrumental => "TODO(es) Instrumental",
        TextKey::LyricsInstrumentalDetail => "TODO(es) No timed lyrics for this track.",
        TextKey::DevicesSettingUp => "TODO(es) Setting up…",
        TextKey::DevicesSetUpPlayback => "TODO(es) Set up playback here",
        TextKey::DevicesConnecting => "TODO(es) Connecting…",
        TextKey::DevicesNetworkReceiver => "TODO(es) On your network, click to connect",
        TextKey::DevicesHeading => "TODO(es) Connect to a device",
        TextKey::DevicesRefresh => "TODO(es) Refresh",
        TextKey::DevicesNoneFound => {
            "TODO(es) No devices found. Open Spotify on another device, then refresh."
        }
        TextKey::DevicesListeningHere => "TODO(es) Listening on this device",
        TextKey::DevicesRestricted => "TODO(es) Restricted",
        TextKey::DevicesPlayHere => "TODO(es) Play here",
        TextKey::QueueRecentTab => "TODO(es) Recent",
        TextKey::QueueSavePlaylist => "TODO(es) Save as a playlist",
        TextKey::QueueClear => "TODO(es) Clear queue",
        TextKey::QueueNowPlaying => "TODO(es) Now playing",
        TextKey::QueueNothingQueued => "TODO(es) Nothing queued",
        TextKey::QueueNothingQueuedDetail => "TODO(es) Queued songs appear here.",
        TextKey::QueuePlayingNext => "TODO(es) Playing next",
        TextKey::QueueNextUp => "TODO(es) Next up",
        TextKey::QueueNoRecentPlays => "TODO(es) No recent plays",
        TextKey::QueueNoRecentPlaysDetail => "TODO(es) Played songs appear here.",
        TextKey::PlayerNothingPlaying => "TODO(es) Nothing playing",
        TextKey::PlayerPickSomething => "TODO(es) Pick a song, album, or playlist",
        TextKey::PlayerRemoveLiked => "TODO(es) Remove from Liked Songs",
        TextKey::PlayerSaveLiked => "TODO(es) Save to Liked Songs",
        TextKey::PlayerShuffle => "TODO(es) Shuffle",
        TextKey::PlayerShuffleOn => "TODO(es) Shuffle",
        TextKey::PlayerPrevious => "TODO(es) Previous",
        TextKey::PlayerNext => "TODO(es) Next",
        TextKey::PlayerRepeat => "TODO(es) Repeat",
        TextKey::PlayerRepeatOne => "TODO(es) Repeat one",
        TextKey::PlayerRepeatOff => "TODO(es) Repeat off",
        TextKey::PlayerPositionAccessibility => "TODO(es) Playback position (%)",
        TextKey::PlayerVolumeAccessibility => "TODO(es) Volume (%)",
        TextKey::PlayerUnmute => "TODO(es) Unmute",
        TextKey::PlayerMute => "TODO(es) Mute",
        TextKey::PlayerConnectDevice => "TODO(es) Connect to a device",
        TextKey::TopbarShowSidebarControl => "TODO(es) Show sidebar (Ctrl+B)",
        TextKey::TopbarShowSidebarCommand => "TODO(es) Show sidebar (Cmd+B)",
        TextKey::TopbarSearchHint => "TODO(es) What do you want to play?",
        TextKey::TopbarMilkdropControl => "TODO(es) MilkDrop visualiser (Ctrl+Shift+K)",
        TextKey::TopbarMilkdropCommand => "TODO(es) MilkDrop visualiser (Cmd+Shift+K)",
        TextKey::TopbarWinampControl => "TODO(es) Winamp mini player (Ctrl+M)",
        TextKey::TopbarWinampCommand => "TODO(es) Winamp mini player (Cmd+Shift+M)",
        TextKey::TopbarWaitingSpotify => "TODO(es) Waiting for Spotify…",
        TextKey::TopbarAnotherDevice => "TODO(es) another device",
        TextKey::CommonSongs => "TODO(es) Songs",
        TextKey::CommonArtists => "TODO(es) Artists",
        TextKey::CommonAlbums => "TODO(es) Albums",
        TextKey::CommonPlaylists => "TODO(es) Playlists",
        TextKey::CommonPodcasts => "TODO(es) Podcasts",
        TextKey::CommonEpisodes => "TODO(es) Episodes",
        TextKey::CommonLikedSongs => "TODO(es) Liked Songs",
        TextKey::CommonRemoveLibrary => "TODO(es) Remove from Your Library",
        TextKey::CommonAddLibrary => "TODO(es) Add to Your Library",
        TextKey::AlbumKindSingle => "TODO(es) Single",
        TextKey::AlbumKindCompilation => "TODO(es) Compilation",
        TextKey::AlbumKindAppearsOn => "TODO(es) Appears On",
        TextKey::AlbumKindAlbum => "TODO(es) Album",
        TextKey::DiscographyFilterAll => "TODO(es) All",
        TextKey::DiscographyFilterAlbums => "TODO(es) Albums",
        TextKey::DiscographyFilterSingles => "TODO(es) Singles & EPs",
        TextKey::DiscographyFilterAppearsOn => "TODO(es) Appears On",
        TextKey::ArtistPopular => "TODO(es) Popular",
        TextKey::ArtistNoPopularSongs => "TODO(es) No popular songs to show.",
        TextKey::ArtistDiscography => "TODO(es) Discography",
        TextKey::ArtistNothingCategory => "TODO(es) Nothing in this category.",
        TextKey::ArtistFansAlsoLike => "TODO(es) Fans also like",
        TextKey::ShowPlayLatest => "TODO(es) Play latest episode",
        TextKey::ShowFollowPodcast => "TODO(es) Follow podcast",
        TextKey::ShowAbout => "TODO(es) About",
        TextKey::ShowAllEpisodes => "TODO(es) All episodes",
        TextKey::ShowPlayed => "TODO(es) Played",
        TextKey::HomeMadeForYou => "TODO(es) Made for you",
        TextKey::HomeShelfLoadError => "TODO(es) Couldn't load this shelf",
        TextKey::HomeRecentlyPlayed => "TODO(es) Recently played",
        TextKey::HomeTopArtists => "TODO(es) Your top artists",
        TextKey::HomeTopSongs => "TODO(es) Your top songs",
        TextKey::HomeShowMoreTopSongs => "TODO(es) Show more top songs",
        TextKey::HomeRecommended => "TODO(es) Recommended for you",
        TextKey::SearchNoResultsDetail => "TODO(es) Check the spelling, or try fewer words.",
        TextKey::SearchSpotify => "TODO(es) Search Spotify",
        TextKey::SearchSpotifyDetail => {
            "TODO(es) Find songs, artists, albums, playlists, and podcasts."
        }
        TextKey::SearchRecent => "TODO(es) Recent searches",
        TextKey::SearchTopResult => "TODO(es) Top result",
        TextKey::LibraryNoSavedAlbums => "TODO(es) No saved albums",
        TextKey::LibrarySavedAlbumsDetail => "TODO(es) Saved albums appear here.",
        TextKey::LibraryNoFollowedArtists => "TODO(es) No followed artists",
        TextKey::LibraryFollowedArtistsDetail => "TODO(es) Followed artists appear here.",
        TextKey::LibraryNoPodcasts => "TODO(es) No podcasts yet",
        TextKey::LibraryFollowedPodcastsDetail => "TODO(es) Followed podcasts appear here.",
        TextKey::LibraryNoSavedEpisodes => "TODO(es) No saved episodes",
        TextKey::LibrarySavedEpisodesDetail => "TODO(es) Saved episodes appear here.",
        TextKey::GreetingMorning => "TODO(es) Good morning",
        TextKey::GreetingAfternoon => "TODO(es) Good afternoon",
        TextKey::GreetingEvening => "TODO(es) Good evening",
        TextKey::DialogDeletePlaylist => "TODO(es) Delete playlist?",
        TextKey::DialogRemoveLibrary => "TODO(es) Remove from Your Library?",
        TextKey::DialogDelete => "TODO(es) Delete",
        TextKey::DialogRemove => "TODO(es) Remove",
        TextKey::DialogSongsAlreadyPlaylist => "TODO(es) Songs already in this playlist",
        TextKey::DialogSongAlreadyPlaylist => "TODO(es) Song already in this playlist",
        TextKey::DialogAddAnyway => "TODO(es) Add anyway",
        TextKey::DialogDone => "TODO(es) Done",
        TextKey::DialogPremiumTitle => "TODO(es) This account cannot play music here",
        TextKey::DialogPremiumDetail => {
            "TODO(es) Playback needs Spotify Premium. Free accounts can browse and search, but cannot play music through Fastpotify."
        }
        TextKey::DialogOk => "TODO(es) OK",
        TextKey::DialogNewPlaylist => "TODO(es) New playlist",
        TextKey::DialogName => "TODO(es) Name",
        TextKey::DialogPlaylistNameHint => "TODO(es) My playlist",
        TextKey::DialogPublicPlaylist => "TODO(es) Public playlist",
        TextKey::DialogCreate => "TODO(es) Create",
        TextKey::DialogEditDetails => "TODO(es) Edit details",
        TextKey::DialogDescription => "TODO(es) Description",
        TextKey::DialogOptionalDescription => "TODO(es) Optional description",
        TextKey::DialogSave => "TODO(es) Save",
        TextKey::CommonSearch => "TODO(es) Search",
        TextKey::SidebarLibrary => "TODO(es) Library",
        TextKey::SidebarHideControl => "TODO(es) Hide sidebar (Ctrl+B)",
        TextKey::SidebarHideCommand => "TODO(es) Hide sidebar (Cmd+B)",
        TextKey::SidebarCreatePlaylist => "TODO(es) Create a playlist",
        TextKey::SidebarSearchLibrary => "TODO(es) Search Your Library",
        TextKey::SidebarNothingHere => "TODO(es) Nothing here yet.",
        TextKey::SidebarNoMatches => "TODO(es) No matches.",
        TextKey::SidebarUnpin => "TODO(es) Unpin",
        TextKey::SidebarPinTop => "TODO(es) Pin to top",
        TextKey::SidebarSortRecent => "TODO(es) Sort by recently played",
        TextKey::MenuPlayNext => "TODO(es) Play next",
        TextKey::MenuAddPlaylist => "TODO(es) Add to playlist",
        TextKey::MenuNewPlaylist => "TODO(es) New playlist",
        TextKey::MenuSaveEpisode => "TODO(es) Save episode",
        TextKey::MenuMoveUp => "TODO(es) Move up",
        TextKey::MenuMoveDown => "TODO(es) Move down",
        TextKey::MenuRemovePlaylist => "TODO(es) Remove from this playlist",
        TextKey::MenuSongRadio => "TODO(es) Go to song radio",
        TextKey::MenuGoArtist => "TODO(es) Go to artist",
        TextKey::MenuGoAlbum => "TODO(es) Go to album",
        TextKey::MenuGoPodcast => "TODO(es) Go to podcast",
        TextKey::MenuCopyLink => "TODO(es) Copy link",
        TextKey::MenuOpenSpotify => "TODO(es) Open in Spotify",
        TextKey::MenuShufflePlay => "TODO(es) Shuffle play",
        TextKey::MenuUnfollow => "TODO(es) Unfollow",
        TextKey::TableSortPlaylistOrder => "TODO(es) Sort by playlist order",
        TextKey::TableOriginalOrderReversed => "TODO(es) Original order, reversed",
        TextKey::TableAddedBy => "TODO(es) ADDED BY",
        TextKey::TableDateAdded => "TODO(es) DATE ADDED",
        TextKey::TableSortDuration => "TODO(es) Sort by duration",
        TextKey::TableTitle => "TODO(es) TITLE",
        TextKey::TableAlbum => "TODO(es) ALBUM",
        TextKey::CollectionShuffleOff => "TODO(es) Shuffle off",
        TextKey::CollectionFilter => "TODO(es) Filter",
        TextKey::CollectionGoSong => "TODO(es) Go to song",
        TextKey::CollectionGo => "TODO(es) Go",
        TextKey::CollectionNothingHere => "TODO(es) Nothing here yet",
        TextKey::CollectionAddedSongs => "TODO(es) Added songs appear here.",
        TextKey::CollectionTopSongsDetail => {
            "TODO(es) Your most-played tracks from the last four weeks."
        }
        TextKey::CollectionCollaborativePlaylist => "TODO(es) Collaborative Playlist",
        TextKey::CollectionPublicPlaylist => "TODO(es) Public Playlist",
        TextKey::CollectionSaveLibrary => "TODO(es) Save to Your Library",
        TextKey::ShortcutPlayPause => "TODO(es) Play or pause",
        TextKey::ShortcutPreviousNext => "TODO(es) Previous or next",
        TextKey::ShortcutSeek => "TODO(es) Seek 10 seconds",
        TextKey::ShortcutVolume => "TODO(es) Volume up or down",
        TextKey::ShortcutMute => "TODO(es) Mute or unmute",
        TextKey::ShortcutLikePlaying => "TODO(es) Like or unlike the playing song",
        TextKey::ShortcutShuffle => "TODO(es) Toggle shuffle",
        TextKey::ShortcutRepeat => "TODO(es) Cycle repeat",
        TextKey::ShortcutQueue => "TODO(es) Show the queue",
        TextKey::ShortcutLyrics => "TODO(es) Show the lyrics",
        TextKey::ShortcutSearch => "TODO(es) Search",
        TextKey::ShortcutSidebar => "TODO(es) Show or hide the sidebar",
        TextKey::ShortcutBackForward => "TODO(es) Back or forward",
        TextKey::ShortcutHome => "TODO(es) Home",
        TextKey::ShortcutLikedSongs => "TODO(es) Liked Songs",
        TextKey::ShortcutPlayingArtist => "TODO(es) Go to the playing artist",
        TextKey::ShortcutPlayingAlbum => "TODO(es) Go to the playing album",
        TextKey::ShortcutWinamp => "TODO(es) Winamp mini player",
        TextKey::ShortcutMilkdrop => "TODO(es) MilkDrop, under the mini player",
        TextKey::ShortcutMilkdropFullscreen => "TODO(es) MilkDrop: fill the screen",
        TextKey::ShortcutMilkdropNext => "TODO(es) MilkDrop: next preset",
        TextKey::ShortcutMilkdropPrevious => "TODO(es) MilkDrop: previous preset",
        TextKey::ShortcutMilkdropKeep => "TODO(es) MilkDrop: keep this preset",
        TextKey::ShortcutMilkdropClose => "TODO(es) MilkDrop: leave full screen, or close",
        TextKey::ShortcutSettings => "TODO(es) Settings",
        TextKey::ShortcutHelp => "TODO(es) Keyboard shortcuts",
        TextKey::ShortcutCloseWindow => "TODO(es) Close the window",
        TextKey::ShortcutQuit => "TODO(es) Quit",
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
        Message::SettingsCredentialsStoredIn { path } => {
            format!("Credentials are kept in {path}")
        }
        Message::SettingsVersion { version } => format!("Fastpotify {version}"),
        Message::SettingsFps { rate } => format!("{rate} fps"),
        Message::SettingsFpsYourScreen { rate } => format!("{rate} fps, your screen"),
        Message::LoginFooter { version } => {
            format!("TODO(es) Fastpotify {version} • not affiliated with Spotify")
        }
        Message::LyricsFetchFailed { detail } => {
            format!("TODO(es) Couldn't fetch the lyrics: {detail}")
        }
        Message::DeviceThisComputer { name } => format!("{name} TODO(es) (this computer)"),
        Message::PlayingOnDevice { name } => format!("TODO(es) Playing on {name}"),
        Message::UpdateToVersion { version } => format!("TODO(es) Update to {version}"),
        Message::UpdateAvailableDetail { version } => {
            format!("TODO(es) Version {version} is available. Open the download page.")
        }
        Message::FollowerCount { count } => match count {
            1 => "TODO(es) 1 follower".to_string(),
            count => format!("TODO(es) {} followers", crate::util::format_count(*count)),
        },
        Message::EpisodeCount { count } => match count {
            1 => "TODO(es) 1 episode".to_string(),
            count => format!("TODO(es) {count} episodes"),
        },
        Message::EpisodeTimeLeft { time } => format!("TODO(es) {time} left"),
        Message::SearchNoResults { query } => format!("TODO(es) No results for “{query}”"),
        Message::SearchSongBy { artist } => format!("TODO(es) Song • {artist}"),
        Message::SearchAlbumBy { artist } => format!("TODO(es) Album • {artist}"),
        Message::SearchPlaylistBy { owner } => format!("TODO(es) Playlist • {owner}"),
        Message::SearchPodcastBy { publisher } => format!("TODO(es) Podcast • {publisher}"),
        Message::ByName { name } => format!("TODO(es) By {name}"),
        Message::AlbumYearKind { year, kind } => format!("TODO(es) {year} • {kind}"),
        Message::DeletePlaylistDetail { name } => {
            format!("TODO(es) Delete “{name}”? You can recover it from Spotify for 90 days.")
        }
        Message::RemovePlaylistDetail { name } => {
            format!("TODO(es) “{name}” will no longer appear in Your Library.")
        }
        Message::PlaylistSongsAdded { count } => match count {
            1 => "TODO(es) 1 song will be added.".to_string(),
            count => format!("TODO(es) {count} songs will be added."),
        },
        Message::DuplicateSongs {
            playlist_name,
            names,
            selected_count,
        } => {
            let named = match names.as_slice() {
                [] => "This song".to_string(),
                [name] => format!("“{name}”"),
                [first, second] => format!("“{first}” and “{second}”"),
                [first, second, rest @ ..] => {
                    format!("“{first}”, “{second}”, and {} more", rest.len())
                }
            };
            let verb = if names.len() <= 1 { "is" } else { "are" };
            let question = if *selected_count == 1 {
                "Add it again?"
            } else {
                "Add all selected songs anyway?"
            };
            format!("TODO(es) {named} {verb} already in “{playlist_name}”. {question}")
        }
        Message::FolderPlaylistCount { count } => match count {
            1 => "TODO(es) Folder • 1 playlist".to_string(),
            count => format!("TODO(es) Folder • {count} playlists"),
        },
        Message::PlaylistSongCount { count } => match count {
            1 => "TODO(es) Playlist • 1 song".to_string(),
            count => format!("TODO(es) Playlist • {count} songs"),
        },
        Message::SortBy { label } => format!("TODO(es) Sort by {label}"),
    }
}
