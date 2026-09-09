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
        TextKey::CommonPlay => "Play",
        TextKey::CommonPause => "Pause",
        TextKey::CommonMore => "More",
        TextKey::CommonStarting => "Starting…",
        TextKey::CommonFollow => "Follow",
        TextKey::CommonFollowing => "Following",
        TextKey::CommonShowLess => "Show less",
        TextKey::CommonSeeMore => "See more",
        TextKey::CommonLoadMore => "Load more",
        TextKey::CommonRetry => "Retry",
        TextKey::CommonClose => "Close",
        TextKey::CommonCancel => "Cancel",
        TextKey::CommonTryAgain => "Try again",
        TextKey::CommonSettings => "Settings",
        TextKey::CommonKeyboardShortcuts => "Keyboard shortcuts",
        TextKey::CommonSignOut => "Sign out",
        TextKey::CommonHome => "Home",
        TextKey::CommonBack => "Back",
        TextKey::CommonForward => "Forward",
        TextKey::CommonQueue => "Queue",
        TextKey::CommonLyrics => "Lyrics",
        TextKey::CommonArtist => "Artist",
        TextKey::CommonAlbum => "Album",
        TextKey::CommonPlaylist => "Playlist",
        TextKey::CommonPodcast => "Podcast",
        TextKey::CommonEpisode => "Episode",
        TextKey::CommonAll => "All",
        TextKey::LoginTagline => "A native Spotify client.",
        TextKey::LoginWaitingBrowser => "Waiting for Spotify in your browser…",
        TextKey::LoginOpenAgain => "Didn't open? Open the sign-in page again",
        TextKey::LoginConnecting => "Connecting to Spotify…",
        TextKey::LoginUseSharedApp => "Use the shared Spotify app instead",
        TextKey::LoginSignIn => "Sign in with Spotify",
        TextKey::LoginPrivacyDetail => {
            "Sign in through your browser. Fastpotify never sees your password. Local playback needs Spotify Premium."
        }
        TextKey::LyricsFollow => "Follow",
        TextKey::LyricsNothingPlaying => "Nothing playing",
        TextKey::LyricsPlaySong => "Play a song to see its lyrics.",
        TextKey::LyricsNoLyrics => "No lyrics",
        TextKey::LyricsNoLyricsDetail => "No lyrics found for this track.",
        TextKey::LyricsInstrumental => "Instrumental",
        TextKey::LyricsInstrumentalDetail => "No timed lyrics for this track.",
        TextKey::DevicesSettingUp => "Setting up…",
        TextKey::DevicesSetUpPlayback => "Set up playback here",
        TextKey::DevicesConnecting => "Connecting…",
        TextKey::DevicesNetworkReceiver => "On your network, click to connect",
        TextKey::DevicesHeading => "Connect to a device",
        TextKey::DevicesRefresh => "Refresh",
        TextKey::DevicesNoneFound => {
            "No devices found. Open Spotify on another device, then refresh."
        }
        TextKey::DevicesListeningHere => "Listening on this device",
        TextKey::DevicesRestricted => "Restricted",
        TextKey::DevicesPlayHere => "Play here",
        TextKey::QueueRecentTab => "Recent",
        TextKey::QueueSavePlaylist => "Save as a playlist",
        TextKey::QueueClear => "Clear queue",
        TextKey::QueueNowPlaying => "Now playing",
        TextKey::QueueNothingQueued => "Nothing queued",
        TextKey::QueueNothingQueuedDetail => "Queued songs appear here.",
        TextKey::QueuePlayingNext => "Playing next",
        TextKey::QueueNextUp => "Next up",
        TextKey::QueueNoRecentPlays => "No recent plays",
        TextKey::QueueNoRecentPlaysDetail => "Played songs appear here.",
        TextKey::PlayerNothingPlaying => "Nothing playing",
        TextKey::PlayerPickSomething => "Pick a song, album, or playlist",
        TextKey::PlayerRemoveLiked => "Remove from Liked Songs",
        TextKey::PlayerSaveLiked => "Save to Liked Songs",
        TextKey::PlayerShuffle => "Shuffle",
        TextKey::PlayerShuffleOn => "Shuffle",
        TextKey::PlayerPrevious => "Previous",
        TextKey::PlayerNext => "Next",
        TextKey::PlayerRepeat => "Repeat",
        TextKey::PlayerRepeatOne => "Repeat one",
        TextKey::PlayerRepeatOff => "Repeat off",
        TextKey::PlayerPositionAccessibility => "Playback position (%)",
        TextKey::PlayerVolumeAccessibility => "Volume (%)",
        TextKey::PlayerUnmute => "Unmute",
        TextKey::PlayerMute => "Mute",
        TextKey::PlayerConnectDevice => "Connect to a device",
        TextKey::TopbarShowSidebarControl => "Show sidebar (Ctrl+B)",
        TextKey::TopbarShowSidebarCommand => "Show sidebar (Cmd+B)",
        TextKey::TopbarSearchHint => "What do you want to play?",
        TextKey::TopbarMilkdropControl => "MilkDrop visualiser (Ctrl+Shift+K)",
        TextKey::TopbarMilkdropCommand => "MilkDrop visualiser (Cmd+Shift+K)",
        TextKey::TopbarWinampControl => "Winamp mini player (Ctrl+M)",
        TextKey::TopbarWinampCommand => "Winamp mini player (Cmd+Shift+M)",
        TextKey::TopbarWaitingSpotify => "Waiting for Spotify…",
        TextKey::TopbarAnotherDevice => "another device",
        TextKey::CommonSongs => "Songs",
        TextKey::CommonArtists => "Artists",
        TextKey::CommonAlbums => "Albums",
        TextKey::CommonPlaylists => "Playlists",
        TextKey::CommonPodcasts => "Podcasts",
        TextKey::CommonEpisodes => "Episodes",
        TextKey::CommonLikedSongs => "Liked Songs",
        TextKey::CommonRemoveLibrary => "Remove from Your Library",
        TextKey::CommonAddLibrary => "Add to Your Library",
        TextKey::AlbumKindSingle => "Single",
        TextKey::AlbumKindCompilation => "Compilation",
        TextKey::AlbumKindAppearsOn => "Appears On",
        TextKey::AlbumKindAlbum => "Album",
        TextKey::DiscographyFilterAll => "All",
        TextKey::DiscographyFilterAlbums => "Albums",
        TextKey::DiscographyFilterSingles => "Singles & EPs",
        TextKey::DiscographyFilterAppearsOn => "Appears On",
        TextKey::ArtistPopular => "Popular",
        TextKey::ArtistNoPopularSongs => "No popular songs to show.",
        TextKey::ArtistDiscography => "Discography",
        TextKey::ArtistNothingCategory => "Nothing in this category.",
        TextKey::ArtistFansAlsoLike => "Fans also like",
        TextKey::ShowPlayLatest => "Play latest episode",
        TextKey::ShowFollowPodcast => "Follow podcast",
        TextKey::ShowAbout => "About",
        TextKey::ShowAllEpisodes => "All episodes",
        TextKey::ShowPlayed => "Played",
        TextKey::HomeMadeForYou => "Made for you",
        TextKey::HomeShelfLoadError => "Couldn't load this shelf",
        TextKey::HomeRecentlyPlayed => "Recently played",
        TextKey::HomeTopArtists => "Your top artists",
        TextKey::HomeTopSongs => "Your top songs",
        TextKey::HomeShowMoreTopSongs => "Show more top songs",
        TextKey::HomeRecommended => "Recommended for you",
        TextKey::SearchNoResultsDetail => "Check the spelling, or try fewer words.",
        TextKey::SearchSpotify => "Search Spotify",
        TextKey::SearchSpotifyDetail => "Find songs, artists, albums, playlists, and podcasts.",
        TextKey::SearchRecent => "Recent searches",
        TextKey::SearchTopResult => "Top result",
        TextKey::LibraryNoSavedAlbums => "No saved albums",
        TextKey::LibrarySavedAlbumsDetail => "Saved albums appear here.",
        TextKey::LibraryNoFollowedArtists => "No followed artists",
        TextKey::LibraryFollowedArtistsDetail => "Followed artists appear here.",
        TextKey::LibraryNoPodcasts => "No podcasts yet",
        TextKey::LibraryFollowedPodcastsDetail => "Followed podcasts appear here.",
        TextKey::LibraryNoSavedEpisodes => "No saved episodes",
        TextKey::LibrarySavedEpisodesDetail => "Saved episodes appear here.",
        TextKey::GreetingMorning => "Good morning",
        TextKey::GreetingAfternoon => "Good afternoon",
        TextKey::GreetingEvening => "Good evening",
        TextKey::DialogDeletePlaylist => "Delete playlist?",
        TextKey::DialogRemoveLibrary => "Remove from Your Library?",
        TextKey::DialogDelete => "Delete",
        TextKey::DialogRemove => "Remove",
        TextKey::DialogSongsAlreadyPlaylist => "Songs already in this playlist",
        TextKey::DialogSongAlreadyPlaylist => "Song already in this playlist",
        TextKey::DialogAddAnyway => "Add anyway",
        TextKey::DialogDone => "Done",
        TextKey::DialogPremiumTitle => "This account cannot play music here",
        TextKey::DialogPremiumDetail => {
            "Playback needs Spotify Premium. Free accounts can browse and search, but cannot play music through Fastpotify."
        }
        TextKey::DialogOk => "OK",
        TextKey::DialogNewPlaylist => "New playlist",
        TextKey::DialogName => "Name",
        TextKey::DialogPlaylistNameHint => "My playlist",
        TextKey::DialogPublicPlaylist => "Public playlist",
        TextKey::DialogCreate => "Create",
        TextKey::DialogEditDetails => "Edit details",
        TextKey::DialogDescription => "Description",
        TextKey::DialogOptionalDescription => "Optional description",
        TextKey::DialogSave => "Save",
        TextKey::CommonSearch => "Search",
        TextKey::SidebarLibrary => "Library",
        TextKey::SidebarHideControl => "Hide sidebar (Ctrl+B)",
        TextKey::SidebarHideCommand => "Hide sidebar (Cmd+B)",
        TextKey::SidebarCreatePlaylist => "Create a playlist",
        TextKey::SidebarSearchLibrary => "Search Your Library",
        TextKey::SidebarNothingHere => "Nothing here yet.",
        TextKey::SidebarNoMatches => "No matches.",
        TextKey::SidebarUnpin => "Unpin",
        TextKey::SidebarPinTop => "Pin to top",
        TextKey::SidebarSortRecent => "Sort by recently played",
        TextKey::MenuPlayNext => "Play next",
        TextKey::MenuAddPlaylist => "Add to playlist",
        TextKey::MenuNewPlaylist => "New playlist",
        TextKey::MenuSaveEpisode => "Save episode",
        TextKey::MenuMoveUp => "Move up",
        TextKey::MenuMoveDown => "Move down",
        TextKey::MenuRemovePlaylist => "Remove from this playlist",
        TextKey::MenuSongRadio => "Go to song radio",
        TextKey::MenuGoArtist => "Go to artist",
        TextKey::MenuGoAlbum => "Go to album",
        TextKey::MenuGoPodcast => "Go to podcast",
        TextKey::MenuCopyLink => "Copy link",
        TextKey::MenuOpenSpotify => "Open in Spotify",
        TextKey::MenuShufflePlay => "Shuffle play",
        TextKey::MenuUnfollow => "Unfollow",
        TextKey::TableSortPlaylistOrder => "Sort by playlist order",
        TextKey::TableOriginalOrderReversed => "Original order, reversed",
        TextKey::TableAddedBy => "ADDED BY",
        TextKey::TableDateAdded => "DATE ADDED",
        TextKey::TableSortDuration => "Sort by duration",
        TextKey::TableTitle => "TITLE",
        TextKey::TableAlbum => "ALBUM",
        TextKey::CollectionShuffleOff => "Shuffle off",
        TextKey::CollectionFilter => "Filter",
        TextKey::CollectionGoSong => "Go to song",
        TextKey::CollectionGo => "Go",
        TextKey::CollectionNothingHere => "Nothing here yet",
        TextKey::CollectionAddedSongs => "Added songs appear here.",
        TextKey::CollectionTopSongsDetail => "Your most-played tracks from the last four weeks.",
        TextKey::CollectionCollaborativePlaylist => "Collaborative Playlist",
        TextKey::CollectionPublicPlaylist => "Public Playlist",
        TextKey::CollectionSaveLibrary => "Save to Your Library",
        TextKey::ShortcutPlayPause => "Play or pause",
        TextKey::ShortcutPreviousNext => "Previous or next",
        TextKey::ShortcutSeek => "Seek 10 seconds",
        TextKey::ShortcutVolume => "Volume up or down",
        TextKey::ShortcutMute => "Mute or unmute",
        TextKey::ShortcutLikePlaying => "Like or unlike the playing song",
        TextKey::ShortcutShuffle => "Toggle shuffle",
        TextKey::ShortcutRepeat => "Cycle repeat",
        TextKey::ShortcutQueue => "Show the queue",
        TextKey::ShortcutLyrics => "Show the lyrics",
        TextKey::ShortcutSearch => "Search",
        TextKey::ShortcutSidebar => "Show or hide the sidebar",
        TextKey::ShortcutBackForward => "Back or forward",
        TextKey::ShortcutHome => "Home",
        TextKey::ShortcutLikedSongs => "Liked Songs",
        TextKey::ShortcutPlayingArtist => "Go to the playing artist",
        TextKey::ShortcutPlayingAlbum => "Go to the playing album",
        TextKey::ShortcutWinamp => "Winamp mini player",
        TextKey::ShortcutMilkdrop => "MilkDrop, under the mini player",
        TextKey::ShortcutMilkdropFullscreen => "MilkDrop: fill the screen",
        TextKey::ShortcutMilkdropNext => "MilkDrop: next preset",
        TextKey::ShortcutMilkdropPrevious => "MilkDrop: previous preset",
        TextKey::ShortcutMilkdropKeep => "MilkDrop: keep this preset",
        TextKey::ShortcutMilkdropClose => "MilkDrop: leave full screen, or close",
        TextKey::ShortcutSettings => "Settings",
        TextKey::ShortcutHelp => "Keyboard shortcuts",
        TextKey::ShortcutCloseWindow => "Close the window",
        TextKey::ShortcutQuit => "Quit",
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
        Message::LoginFooter { version } => {
            format!("Fastpotify {version} • not affiliated with Spotify")
        }
        Message::LyricsFetchFailed { detail } => {
            format!("Couldn't fetch the lyrics: {detail}")
        }
        Message::DeviceThisComputer { name } => format!("{name} (this computer)"),
        Message::PlayingOnDevice { name } => format!("Playing on {name}"),
        Message::UpdateToVersion { version } => format!("Update to {version}"),
        Message::UpdateAvailableDetail { version } => {
            format!("Version {version} is available. Open the download page.")
        }
        Message::FollowerCount { count } => match count {
            1 => "1 follower".to_string(),
            count => format!("{} followers", crate::util::format_count(*count)),
        },
        Message::EpisodeCount { count } => match count {
            1 => "1 episode".to_string(),
            count => format!("{count} episodes"),
        },
        Message::EpisodeTimeLeft { time } => format!("{time} left"),
        Message::SearchNoResults { query } => format!("No results for “{query}”"),
        Message::SearchSongBy { artist } => format!("Song • {artist}"),
        Message::SearchAlbumBy { artist } => format!("Album • {artist}"),
        Message::SearchPlaylistBy { owner } => format!("Playlist • {owner}"),
        Message::SearchPodcastBy { publisher } => format!("Podcast • {publisher}"),
        Message::ByName { name } => format!("By {name}"),
        Message::AlbumYearKind { year, kind } => format!("{year} • {kind}"),
        Message::DeletePlaylistDetail { name } => {
            format!("Delete “{name}”? You can recover it from Spotify for 90 days.")
        }
        Message::RemovePlaylistDetail { name } => {
            format!("“{name}” will no longer appear in Your Library.")
        }
        Message::PlaylistSongsAdded { count } => match count {
            1 => "1 song will be added.".to_string(),
            count => format!("{count} songs will be added."),
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
            format!("{named} {verb} already in “{playlist_name}”. {question}")
        }
        Message::FolderPlaylistCount { count } => match count {
            1 => "Folder • 1 playlist".to_string(),
            count => format!("Folder • {count} playlists"),
        },
        Message::PlaylistSongCount { count } => match count {
            1 => "Playlist • 1 song".to_string(),
            count => format!("Playlist • {count} songs"),
        },
        Message::SortBy { label } => format!("Sort by {label}"),
    }
}
