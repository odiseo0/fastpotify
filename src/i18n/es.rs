use super::{Message, TextKey};

pub(super) fn text(key: TextKey) -> &'static str {
    match key {
        TextKey::SettingsTitle => "Configuración",
        TextKey::SettingsAccountHeading => "Cuenta",
        TextKey::SettingsSpotifyPremium => "Spotify Premium",
        TextKey::SettingsSpotifyFreeNeedsPremium => {
            "Spotify Free, la reproducción local requiere Premium"
        }
        TextKey::SettingsSignOut => "Cerrar sesión",
        TextKey::SettingsPersonalAppLabel => "Aplicación personal de Spotify",
        TextKey::SettingsPersonalAppDetail => {
            "Usa una aplicación personal en modo de desarrollo para tener una cuota de API aparte. La aplicación compartida seguirá activa."
        }
        TextKey::SettingsClientIdHint => "Client ID",
        TextKey::SettingsCreateAppLabel => "Crear una aplicación",
        TextKey::SettingsCreateAppDetail => {
            "Créala gratis en el panel para desarrolladores de Spotify."
        }
        TextKey::SettingsSetupGuide => "Guía de configuración",
        TextKey::SettingsPersonalAppReadyLabel => "Aplicación personal lista",
        TextKey::SettingsPersonalAppReadyDetail => {
            "Las solicitudes compatibles usan tu aplicación. Las demás usan la aplicación compartida."
        }
        TextKey::SettingsRemove => "Quitar",
        TextKey::SettingsAuthorizePersonalAppLabel => "Autorizar tu aplicación personal",
        TextKey::SettingsAuthorizePersonalAppDetail => {
            "Spotify se abre en el navegador para verificar la cuenta."
        }
        TextKey::SettingsAuthorize => "Autorizar",
        TextKey::SettingsRemovePersonalAppLabel => "Quitar aplicación personal",
        TextKey::SettingsRemovePersonalAppDetail => "El acceso compartido seguirá conectado.",
        TextKey::SettingsPlaybackHeading => "Reproducción en este equipo",
        TextKey::SettingsPlaybackReady => "Listo",
        TextKey::SettingsPlaybackReadyDetail => "Este equipo es un dispositivo Spotify Connect.",
        TextKey::SettingsPlaybackSettingUp => "Configurando",
        TextKey::SettingsPlaybackSettingUpDetail => "Termina la autorización en el navegador.",
        TextKey::SettingsPlaybackConnecting => "Conectando",
        TextKey::SettingsPlaybackConnectingDetail => "Conectando con Spotify…",
        TextKey::SettingsPlaybackUnavailable => "No disponible",
        TextKey::SettingsPlaybackNotSetUp => "Sin configurar",
        TextKey::SettingsPlaybackNotSetUpDetail => {
            "Requiere Spotify Premium e iniciar sesión una vez en el navegador."
        }
        TextKey::SettingsTryAgain => "Intentar de nuevo",
        TextKey::SettingsEnablePlayback => "Activar la reproducción aquí",
        TextKey::SettingsReconnect => "Volver a conectar",
        TextKey::SettingsDeviceNameLabel => "Nombre del dispositivo",
        TextKey::SettingsDeviceNameDetail => "Cómo aparece este equipo en Spotify Connect.",
        TextKey::SettingsAudioQualityLabel => "Calidad de audio",
        TextKey::SettingsAudioQualityDetail => {
            "Las tasas de bits más altas usan más datos y espacio de caché."
        }
        TextKey::SettingsQualityVeryHigh => "Muy alta · 320 kbps",
        TextKey::SettingsQualityHigh => "Alta · 160 kbps",
        TextKey::SettingsQualityNormal => "Normal · 96 kbps",
        TextKey::SettingsNormalizeVolumeLabel => "Normalizar volumen",
        TextKey::SettingsNormalizeVolumeDetail => {
            "Mantiene las canciones fuertes y suaves a un nivel similar."
        }
        TextKey::SettingsAutoplayLabel => "Reproducción automática",
        TextKey::SettingsAutoplayDetail => {
            "Sigue reproduciendo canciones similares cuando termine tu música."
        }
        TextKey::SettingsGaplessLabel => "Reproducción sin pausas",
        TextKey::SettingsGaplessDetail => "Reproduce las canciones sin silencio entre ellas.",
        TextKey::SettingsKeepPlayingLabel => "Seguir reproduciendo música al cerrar la ventana",
        TextKey::SettingsKeepPlayingDetailControl => {
            "Fastpotify se oculta en la bandeja del sistema. Sal desde el menú de la bandeja o con Ctrl+Q."
        }
        TextKey::SettingsKeepPlayingDetailCommand => {
            "Fastpotify se oculta en la bandeja del sistema. Sal desde el menú de la bandeja o con Cmd+Q."
        }
        TextKey::SettingsUpdateChecksLabel => "Comprobaciones automáticas de actualizaciones",
        TextKey::SettingsUpdateChecksDetail => {
            "Consulta GitHub una vez al día. No envía datos personales."
        }
        TextKey::SettingsAudioOutputLabel => "Salida de audio",
        TextKey::SettingsAudioOutputDetail => {
            "PulseAudio también funciona con PipeWire. Rodio se comunica directamente con ALSA."
        }
        TextKey::SettingsOutputBufferLabel => "Búfer de salida",
        TextKey::SettingsOutputBufferDetail => {
            "Un búfer mayor puede evitar chasquidos en equipos ocupados. Un búfer menor hace que los controles respondan antes."
        }
        TextKey::SettingsAudioCacheLabel => "Caché de audio",
        TextKey::SettingsAudioCacheDetail => {
            "Guarda el audio descargado para reproducirlo después."
        }
        TextKey::SettingsApplyRestartPlayback => "Aplicar y reiniciar la reproducción",
        TextKey::SettingsRestartPlaybackDetail => {
            "Reinicia la reproducción local para aplicar esta configuración."
        }
        TextKey::SettingsPlaybackApplied => "Se aplicó la configuración de reproducción.",
        TextKey::SettingsAppearanceHeading => "Apariencia",
        TextKey::SettingsLanguageLabel => "Idioma",
        TextKey::SettingsThemeLabel => "Tema",
        TextKey::SettingsThemeDark => "Oscuro",
        TextKey::SettingsThemeLight => "Claro",
        TextKey::SettingsThemeSystem => "Usar el del sistema",
        TextKey::SettingsColourFromArtLabel => "Color de la portada",
        TextKey::SettingsColourFromArtDetail => {
            "Usa el color de la portada actual en las páginas y la barra del reproductor."
        }
        TextKey::SettingsCompactSidebarLabel => "Barra lateral compacta",
        TextKey::SettingsCompactSidebarDetail => {
            "Muestra los nombres sin portadas en la barra lateral."
        }
        TextKey::SettingsCompactTrackListLabel => "Lista de canciones compacta",
        TextKey::SettingsCompactTrackListDetail => {
            "Muestra cada canción en una línea y sin portada."
        }
        TextKey::SettingsZoomLabel => "Zoom de la interfaz",
        TextKey::SettingsZoomDetailControl => {
            "Ctrl+Más y Ctrl+Menos funcionan en cualquier lugar; Ctrl+0 restablece el zoom."
        }
        TextKey::SettingsZoomDetailCommand => {
            "Cmd+Más y Cmd+Menos funcionan en cualquier lugar; Cmd+0 restablece el zoom."
        }
        TextKey::SettingsWinampHeading => "Skins de Winamp",
        TextKey::SettingsMiniPlayerLabel => "Minirreproductor",
        TextKey::SettingsMiniPlayerDetailControl => {
            "Usa skins clásicos de Winamp en formato .wsz. Pulsa Ctrl+M o haz clic en el logotipo del skin para volver. Suelta un skin en cualquiera de las ventanas para añadirlo."
        }
        TextKey::SettingsMiniPlayerDetailCommand => {
            "Usa skins clásicos de Winamp en formato .wsz. Pulsa Cmd+Shift+M o haz clic en el logotipo del skin para volver. Suelta un skin en cualquiera de las ventanas para añadirlo."
        }
        TextKey::SettingsSwitchToIt => "Cambiar a él",
        TextKey::SettingsSkinLabel => "Skin",
        TextKey::SettingsSkinMuseum => "Skin Museum",
        TextKey::SettingsOpenFolder => "Abrir carpeta",
        TextKey::SettingsSkinSizeLabel => "Tamaño",
        TextKey::SettingsSkinSizeDetail => {
            "La escala con números enteros mantiene nítidos los píxeles del skin."
        }
        TextKey::SettingsAlwaysOnTopLabel => "Siempre visible",
        TextKey::SettingsAlwaysOnTopDetail => {
            "Mantiene la ventana de Winamp por encima de las demás."
        }
        TextKey::SettingsMilkdropHeading => "MilkDrop",
        TextKey::SettingsMilkdropWindowLabel => "Ventana de MilkDrop",
        TextKey::SettingsMilkdropWindowDetailControl => {
            "Un visualizador projectM para la reproducción local. Ábrelo aquí, desde la barra superior, con Ctrl+Shift+K o desde el menú V del minirreproductor. Pulsa ? o F1 para ver sus atajos."
        }
        TextKey::SettingsMilkdropWindowDetailCommand => {
            "Un visualizador projectM para la reproducción local. Ábrelo aquí, desde la barra superior, con Cmd+Shift+K o desde el menú V del minirreproductor. Pulsa ? o F1 para ver sus atajos."
        }
        TextKey::SettingsPresetsLabel => "Preajustes",
        TextKey::SettingsFetching => "Obteniendo...",
        TextKey::SettingsMilkdropOriginalPackNote => {
            "Los 550 preajustes incluidos con MilkDrop 2; cerca de 1 MB."
        }
        TextKey::SettingsMilkdropCropPackNote => {
            "La selección de Jason Fletcher de 9.800 preajustes creados por la comunidad; cerca de 25 MB."
        }
        TextKey::SettingsTimePerPresetLabel => "Tiempo por preajuste",
        TextKey::SettingsTimePerPresetDetail => {
            "Cuánto dura cada preajuste antes de pasar de forma gradual al siguiente."
        }
        TextKey::SettingsFrameRateLabel => "Velocidad de fotogramas",
        TextKey::SettingsFrameRateDetail => {
            "Las velocidades menores usan menos recursos. Sin límite dibuja tan rápido como sea posible."
        }
        TextKey::SettingsResolutionLabel => "Resolución",
        TextKey::SettingsResolutionDetail => {
            "Media y Cuarto usan menos recursos y vuelven a ampliar la imagen."
        }
        TextKey::SettingsResolutionFull => "Completa",
        TextKey::SettingsResolutionHalf => "Media",
        TextKey::SettingsResolutionQuarter => "Cuarto",
        TextKey::SettingsEqualizerHeading => "Ecualizador",
        TextKey::SettingsEqualizerLabel => "Ecualizador",
        TextKey::SettingsEqualizerDetail => {
            "Un ecualizador de diez bandas para la reproducción en este equipo. No afecta a otros dispositivos."
        }
        TextKey::SettingsPreampLabel => "Preamplificador",
        TextKey::SettingsStorageHeading => "Almacenamiento",
        TextKey::SettingsArtworkCacheLabel => "Caché de portadas",
        TextKey::SettingsClearArtwork => "Borrar portadas",
        TextKey::SettingsPlayHistoryLabel => "Historial de reproducción",
        TextKey::SettingsClearHistory => "Borrar historial",
        TextKey::SettingsSignInLabel => "Inicio de sesión",
        TextKey::SettingsAboutHeading => "Acerca de",
        TextKey::SettingsAboutDetail => {
            "Creado con Rust, egui y librespot. No está afiliado a Spotify."
        }
        TextKey::SettingsCheckingUpdates => "Comprobando…",
        TextKey::SettingsCheckForUpdates => "Buscar actualizaciones",
        TextKey::SettingsKeyboardShortcuts => "Atajos de teclado",
        TextKey::SettingsSourceCode => "Código fuente",
        TextKey::SettingsFpsUncapped => "Sin límite",
        TextKey::CommonPlay => "Reproducir",
        TextKey::CommonPause => "Pausar",
        TextKey::CommonMore => "Más",
        TextKey::CommonStarting => "Iniciando…",
        TextKey::CommonFollow => "Seguir",
        TextKey::CommonFollowing => "Siguiendo",
        TextKey::CommonShowLess => "Mostrar menos",
        TextKey::CommonSeeMore => "Ver más",
        TextKey::CommonLoadMore => "Cargar más",
        TextKey::CommonLoading => "Cargando…",
        TextKey::CommonRetry => "Reintentar",
        TextKey::CommonClose => "Cerrar",
        TextKey::CommonCancel => "Cancelar",
        TextKey::CommonTryAgain => "Intentar de nuevo",
        TextKey::CommonSettings => "Configuración",
        TextKey::WindowMinimize => "Minimizar",
        TextKey::WindowRestore => "Restaurar",
        TextKey::WindowMaximize => "Maximizar",
        TextKey::WindowClose => "Cerrar",
        TextKey::CommonKeyboardShortcuts => "Atajos de teclado",
        TextKey::CommonSignOut => "Cerrar sesión",
        TextKey::CommonHome => "Inicio",
        TextKey::CommonBack => "Atrás",
        TextKey::CommonForward => "Adelante",
        TextKey::CommonQueue => "Cola",
        TextKey::CommonLyrics => "Letra",
        TextKey::CommonArtist => "Artista",
        TextKey::CommonAlbum => "Álbum",
        TextKey::CommonPlaylist => "Playlist",
        TextKey::CommonPodcast => "Pódcast",
        TextKey::CommonEpisode => "Episodio",
        TextKey::CommonAll => "Todo",
        TextKey::LoginTagline => "Un cliente nativo de Spotify.",
        TextKey::LoginWaitingBrowser => "Esperando a Spotify en el navegador…",
        TextKey::LoginOpenAgain => "¿No se abrió? Abre de nuevo la página de inicio de sesión",
        TextKey::LoginConnecting => "Conectando con Spotify…",
        TextKey::LoginUseSharedApp => "Usar la aplicación compartida de Spotify",
        TextKey::LoginSignIn => "Iniciar sesión con Spotify",
        TextKey::LoginPrivacyDetail => {
            "Inicia sesión desde el navegador. Fastpotify nunca ve tu contraseña. La reproducción local requiere Spotify Premium."
        }
        TextKey::LyricsFollow => "Seguir",
        TextKey::LyricsNothingPlaying => "No hay nada en reproducción",
        TextKey::LyricsPlaySong => "Reproduce una canción para ver la letra.",
        TextKey::LyricsNoLyrics => "Sin letra",
        TextKey::LyricsNoLyricsDetail => "No se encontró la letra de esta canción.",
        TextKey::LyricsInstrumental => "Instrumental",
        TextKey::LyricsInstrumentalDetail => "Esta canción no tiene letra sincronizada.",
        TextKey::DevicesSettingUp => "Configurando…",
        TextKey::DevicesSetUpPlayback => "Configurar la reproducción aquí",
        TextKey::DevicesConnecting => "Conectando…",
        TextKey::DevicesNetworkReceiver => "En tu red, haz clic para conectar",
        TextKey::DevicesHeading => "Conectar a un dispositivo",
        TextKey::DevicesRefresh => "Actualizar",
        TextKey::DevicesNoneFound => {
            "No se encontraron dispositivos. Abre Spotify en otro dispositivo y actualiza."
        }
        TextKey::DevicesListeningHere => "Escuchando en este dispositivo",
        TextKey::DevicesRestricted => "Restringido",
        TextKey::DevicesPlayHere => "Reproducir aquí",
        TextKey::QueueRecentTab => "Recientes",
        TextKey::QueueSavePlaylist => "Guardar como Playlist",
        TextKey::QueueClear => "Vaciar cola",
        TextKey::QueueNowPlaying => "En reproducción",
        TextKey::QueueNothingQueued => "La cola está vacía",
        TextKey::QueueNothingQueuedDetail => "Las canciones en cola aparecerán aquí.",
        TextKey::QueuePlayingNext => "A continuación",
        TextKey::QueueNextUp => "Después",
        TextKey::QueueNoRecentPlays => "No hay reproducciones recientes",
        TextKey::QueueNoRecentPlaysDetail => "Las canciones reproducidas aparecerán aquí.",
        TextKey::PlayerNothingPlaying => "No hay nada en reproducción",
        TextKey::PlayerPickSomething => "Elige una canción, un álbum o una Playlist",
        TextKey::PlayerRemoveLiked => "Quitar de Canciones favoritas",
        TextKey::PlayerSaveLiked => "Guardar en Canciones favoritas",
        TextKey::PlayerShuffle => "Orden aleatorio",
        TextKey::PlayerShuffleOn => "Orden aleatorio",
        TextKey::PlayerPrevious => "Anterior",
        TextKey::PlayerNext => "Siguiente",
        TextKey::PlayerRepeat => "Repetir",
        TextKey::PlayerRepeatOne => "Repetir una",
        TextKey::PlayerRepeatOff => "Desactivar repetición",
        TextKey::PlayerPositionAccessibility => "Posición de reproducción (%)",
        TextKey::PlayerVolumeAccessibility => "Volumen (%)",
        TextKey::PlayerUnmute => "Activar sonido",
        TextKey::PlayerMute => "Silenciar",
        TextKey::PlayerConnectDevice => "Conectar a un dispositivo",
        TextKey::TopbarShowSidebarControl => "Mostrar barra lateral (Ctrl+B)",
        TextKey::TopbarShowSidebarCommand => "Mostrar barra lateral (Cmd+B)",
        TextKey::TopbarSearchHint => "¿Qué quieres reproducir?",
        TextKey::SearchClear => "Borrar",
        TextKey::PlaylistSongsUnavailableThirdParty => {
            "Spotify no permite que las aplicaciones de terceros accedan a las canciones de esta Playlist."
        }
        TextKey::TopbarMilkdropControl => "Visualizador MilkDrop (Ctrl+Shift+K)",
        TextKey::TopbarMilkdropCommand => "Visualizador MilkDrop (Cmd+Shift+K)",
        TextKey::TopbarWinampControl => "Minirreproductor Winamp (Ctrl+M)",
        TextKey::TopbarWinampCommand => "Minirreproductor Winamp (Cmd+Shift+M)",
        TextKey::TopbarWaitingSpotify => "Esperando a Spotify…",
        TextKey::TopbarAnotherDevice => "otro dispositivo",
        TextKey::CommonSongs => "Canciones",
        TextKey::CommonArtists => "Artistas",
        TextKey::CommonAlbums => "Álbumes",
        TextKey::CommonPlaylists => "Playlists",
        TextKey::CommonPodcasts => "Pódcasts",
        TextKey::CommonEpisodes => "Episodios",
        TextKey::CommonLikedSongs => "Canciones favoritas",
        TextKey::CommonRemoveLibrary => "Quitar de tu biblioteca",
        TextKey::CommonAddLibrary => "Añadir a tu biblioteca",
        TextKey::AlbumKindSingle => "Sencillo",
        TextKey::AlbumKindCompilation => "Recopilación",
        TextKey::AlbumKindAppearsOn => "Aparece en",
        TextKey::AlbumKindAlbum => "Álbum",
        TextKey::DiscographyFilterAll => "Todo",
        TextKey::DiscographyFilterAlbums => "Álbumes",
        TextKey::DiscographyFilterSingles => "Sencillos y EP",
        TextKey::DiscographyFilterAppearsOn => "Aparece en",
        TextKey::ArtistPopular => "Popular",
        TextKey::ArtistNoPopularSongs => "No hay canciones populares para mostrar.",
        TextKey::ArtistDiscography => "Discografía",
        TextKey::ArtistNothingCategory => "No hay nada en esta categoría.",
        TextKey::ArtistFansAlsoLike => "A los fans también les gusta",
        TextKey::ShowPlayLatest => "Reproducir el episodio más reciente",
        TextKey::ShowFollowPodcast => "Seguir pódcast",
        TextKey::ShowAbout => "Acerca de",
        TextKey::ShowAllEpisodes => "Todos los episodios",
        TextKey::ShowPlayed => "Reproducido",
        TextKey::HomeMadeForYou => "Hecho para ti",
        TextKey::HomeShelfLoadError => "No se pudo cargar esta sección",
        TextKey::HomeRecentlyPlayed => "Reproducido recientemente",
        TextKey::HomeTopArtists => "Tus artistas más escuchados",
        TextKey::HomeTopSongs => "Tus canciones más escuchadas",
        TextKey::HomeShowMoreTopSongs => "Mostrar más canciones populares",
        TextKey::HomeRecommended => "Recomendado para ti",
        TextKey::SearchNoResultsDetail => "Revisa la ortografía o usa menos palabras.",
        TextKey::SearchSpotify => "Buscar en Spotify",
        TextKey::SearchSpotifyDetail => {
            "Encuentra canciones, artistas, álbumes, Playlists y pódcasts."
        }
        TextKey::SearchRecent => "Búsquedas recientes",
        TextKey::SearchTopResult => "Resultado principal",
        TextKey::LibraryNoSavedAlbums => "No hay álbumes guardados",
        TextKey::LibrarySavedAlbumsDetail => "Los álbumes guardados aparecerán aquí.",
        TextKey::LibraryNoFollowedArtists => "No sigues a ningún artista",
        TextKey::LibraryFollowedArtistsDetail => "Los artistas que sigues aparecerán aquí.",
        TextKey::LibraryNoPodcasts => "Aún no hay pódcasts",
        TextKey::LibraryFollowedPodcastsDetail => "Los pódcasts que sigues aparecerán aquí.",
        TextKey::LibraryNoSavedEpisodes => "No hay episodios guardados",
        TextKey::LibrarySavedEpisodesDetail => "Los episodios guardados aparecerán aquí.",
        TextKey::GreetingMorning => "Buenos días",
        TextKey::GreetingAfternoon => "Buenas tardes",
        TextKey::GreetingEvening => "Buenas noches",
        TextKey::DialogDeletePlaylist => "¿Eliminar Playlist?",
        TextKey::DialogRemoveLibrary => "¿Quitar de tu biblioteca?",
        TextKey::DialogDelete => "Eliminar",
        TextKey::DialogRemove => "Quitar",
        TextKey::DialogSongsAlreadyPlaylist => "Canciones que ya están en esta Playlist",
        TextKey::DialogSongAlreadyPlaylist => "La canción ya está en esta Playlist",
        TextKey::DialogAddAnyway => "Añadir de todos modos",
        TextKey::DialogDone => "Listo",
        TextKey::DialogPremiumTitle => "Esta cuenta no puede reproducir música aquí",
        TextKey::DialogPremiumDetail => {
            "La reproducción requiere Spotify Premium. Las cuentas gratuitas pueden explorar y buscar, pero no reproducir música con Fastpotify."
        }
        TextKey::DialogOk => "Aceptar",
        TextKey::DialogNewPlaylist => "Nueva Playlist",
        TextKey::DialogName => "Nombre",
        TextKey::DialogPlaylistNameHint => "Mi Playlist",
        TextKey::DialogPublicPlaylist => "Playlist pública",
        TextKey::DialogCreate => "Crear",
        TextKey::DialogEditDetails => "Editar detalles",
        TextKey::DialogDescription => "Descripción",
        TextKey::DialogOptionalDescription => "Descripción opcional",
        TextKey::DialogSave => "Guardar",
        TextKey::CommonSearch => "Buscar",
        TextKey::SidebarLibrary => "Biblioteca",
        TextKey::SidebarHideControl => "Ocultar barra lateral (Ctrl+B)",
        TextKey::SidebarHideCommand => "Ocultar barra lateral (Cmd+B)",
        TextKey::SidebarCreatePlaylist => "Crear una Playlist",
        TextKey::SidebarSearchLibrary => "Buscar en Tu biblioteca",
        TextKey::SidebarNothingHere => "Aún no hay nada aquí.",
        TextKey::SidebarNoMatches => "No hay coincidencias.",
        TextKey::SidebarUnpin => "Desfijar",
        TextKey::SidebarPinTop => "Fijar arriba",
        TextKey::SidebarSortRecent => "Ordenar por reproducción reciente",
        TextKey::SidebarHome => "Inicio",
        TextKey::SidebarSearch => "Buscar",
        TextKey::SidebarDefaultFolder => "Carpeta",
        TextKey::SidebarFilterPlaylists => "Playlists",
        TextKey::SidebarFilterAlbums => "Álbumes",
        TextKey::SidebarFilterArtists => "Artistas",
        TextKey::SidebarFilterPodcasts => "Pódcasts",
        TextKey::SidebarLikedSongs => "Canciones favoritas",
        TextKey::SidebarPlaylistKind => "Playlist",
        TextKey::SidebarArtistKind => "Artista",
        TextKey::SidebarPlay => "Reproducir",
        TextKey::MenuPlayNext => "Reproducir a continuación",
        TextKey::MenuAddPlaylist => "Añadir a una Playlist",
        TextKey::MenuNewPlaylist => "Nueva Playlist",
        TextKey::MenuSaveEpisode => "Guardar episodio",
        TextKey::MenuMoveUp => "Mover arriba",
        TextKey::MenuMoveDown => "Mover abajo",
        TextKey::MenuRemovePlaylist => "Quitar de esta Playlist",
        TextKey::MenuSongRadio => "Ir a la radio de la canción",
        TextKey::MenuGoArtist => "Ir al artista",
        TextKey::MenuGoAlbum => "Ir al álbum",
        TextKey::MenuGoPodcast => "Ir al pódcast",
        TextKey::MenuCopyLink => "Copiar enlace",
        TextKey::MenuOpenSpotify => "Abrir en Spotify",
        TextKey::MenuShufflePlay => "Reproducir en orden aleatorio",
        TextKey::MenuUnfollow => "Dejar de seguir",
        TextKey::TableSortPlaylistOrder => "Ordenar según la Playlist",
        TextKey::TableOriginalOrderReversed => "Orden original, invertido",
        TextKey::TableAddedBy => "AÑADIDA POR",
        TextKey::TableDateAdded => "FECHA DE ADICIÓN",
        TextKey::TableSortDuration => "Ordenar por duración",
        TextKey::TableTitle => "TÍTULO",
        TextKey::TableAlbum => "ÁLBUM",
        TextKey::CollectionShuffleOff => "Orden aleatorio desactivado",
        TextKey::CollectionFilter => "Filtrar",
        TextKey::CollectionGoSong => "Ir a la canción",
        TextKey::CollectionGo => "Ir",
        TextKey::CollectionNothingHere => "Aún no hay nada aquí",
        TextKey::CollectionAddedSongs => "Las canciones añadidas aparecerán aquí.",
        TextKey::CollectionTopSongsDetail => {
            "Tus canciones más reproducidas de las últimas cuatro semanas."
        }
        TextKey::CollectionCollaborativePlaylist => "Playlist colaborativa",
        TextKey::CollectionPublicPlaylist => "Playlist pública",
        TextKey::ShortcutPlayPause => "Reproducir o pausar",
        TextKey::ShortcutPreviousNext => "Anterior o siguiente",
        TextKey::ShortcutSeek => "Avanzar o retroceder 10 segundos",
        TextKey::ShortcutVolume => "Subir o bajar el volumen",
        TextKey::ShortcutMute => "Silenciar o activar el sonido",
        TextKey::ShortcutLikePlaying => "Añadir o quitar la canción de Canciones favoritas",
        TextKey::ShortcutShuffle => "Activar o desactivar el orden aleatorio",
        TextKey::ShortcutRepeat => "Cambiar el modo de repetición",
        TextKey::ShortcutQueue => "Mostrar la cola",
        TextKey::ShortcutLyrics => "Mostrar la letra",
        TextKey::ShortcutSearch => "Buscar",
        TextKey::ShortcutSidebar => "Mostrar u ocultar la barra lateral",
        TextKey::ShortcutBackForward => "Atrás o adelante",
        TextKey::ShortcutHome => "Inicio",
        TextKey::ShortcutLikedSongs => "Canciones favoritas",
        TextKey::ShortcutPlayingArtist => "Ir al artista en reproducción",
        TextKey::ShortcutPlayingAlbum => "Ir al álbum en reproducción",
        TextKey::ShortcutWinamp => "Minirreproductor Winamp",
        TextKey::ShortcutMilkdrop => "MilkDrop, bajo el minirreproductor",
        TextKey::ShortcutMilkdropFullscreen => "MilkDrop: llenar la pantalla",
        TextKey::ShortcutMilkdropNext => "MilkDrop: siguiente preajuste",
        TextKey::ShortcutMilkdropPrevious => "MilkDrop: preajuste anterior",
        TextKey::ShortcutMilkdropKeep => "MilkDrop: mantener este preajuste",
        TextKey::ShortcutMilkdropClose => "MilkDrop: salir de pantalla completa o cerrar",
        TextKey::ShortcutSettings => "Configuración",
        TextKey::ShortcutHelp => "Atajos de teclado",
        TextKey::ShortcutCloseWindow => "Cerrar la ventana",
        TextKey::ShortcutQuit => "Salir",
        TextKey::MenuSelectionRemoveLiked => "Quitar de Canciones favoritas",
        TextKey::MenuSelectionSaveLiked => "Guardar en Canciones favoritas",
        TextKey::MenuTrackRemoveLiked => "Quitar de Canciones favoritas",
        TextKey::MenuTrackSaveLiked => "Guardar en Canciones favoritas",
        TextKey::TrackRowRemoveLiked => "Quitar de Canciones favoritas",
        TextKey::TrackRowSaveLiked => "Guardar en Canciones favoritas",
        TextKey::TrackRowMore => "Más",
        TextKey::CollectionStarting => "Iniciando…",
        TextKey::CollectionPlay => "Reproducir",
        TextKey::CollectionPause => "Pausar",
        TextKey::CollectionPlaylistKind => "Playlist",
        TextKey::CollectionLikedSongs => "Canciones favoritas",
        TextKey::CollectionPlaylistAddLibrary => "Añadir a tu biblioteca",
        TextKey::CollectionPlaylistRemoveLibrary => "Quitar de tu biblioteca",
        TextKey::CollectionAlbumSaveLibrary => "Guardar en tu biblioteca",
        TextKey::CollectionAlbumRemoveLibrary => "Quitar de tu biblioteca",
        TextKey::DateMonthJan => "ene",
        TextKey::DateMonthFeb => "feb",
        TextKey::DateMonthMar => "mar",
        TextKey::DateMonthApr => "abr",
        TextKey::DateMonthMay => "may",
        TextKey::DateMonthJun => "jun",
        TextKey::DateMonthJul => "jul",
        TextKey::DateMonthAug => "ago",
        TextKey::DateMonthSep => "sep",
        TextKey::DateMonthOct => "oct",
        TextKey::DateMonthNov => "nov",
        TextKey::DateMonthDec => "dic",
        TextKey::NoticeUpToDate => "Fastpotify está actualizado",
        TextKey::NoticeQueueCleared => "Se vació la cola",
        TextKey::NoticePlaylistUpdated => "Playlist actualizada",
        TextKey::NoticeAddedLibrary => "Se añadió a tu biblioteca",
        TextKey::NoticeRemovedLibrary => "Se quitó de tu biblioteca",
        TextKey::NoticeAddedLikedSongs => "Se añadió a Canciones favoritas",
        TextKey::NoticeRemovedLikedSongs => "Se quitó de Canciones favoritas",
        TextKey::NoticeFollowingArtist => "Ahora sigues al artista",
        TextKey::NoticeUnfollowedArtist => "Dejaste de seguir al artista",
        TextKey::NoticeSavedLibrary => "Se guardó en tu biblioteca",
        TextKey::NoticeEpisodePodcastUnavailable => {
            "El pódcast de este episodio no está en Spotify"
        }
        TextKey::NoticeSongAlbumUnavailable => "El álbum de esta canción no está en Spotify",
        TextKey::NoticeUnsupportedSpotifyLink => {
            "Fastpotify no puede abrir este tipo de enlace de Spotify"
        }
        TextKey::NoticeNothingPlaying => "No hay nada en reproducción. Elige algo primero",
        TextKey::NoticeChooseDevice => {
            "Elige un dispositivo o activa la reproducción en este equipo"
        }
        TextKey::NoticePickSomething => "Elige algo para reproducir",
        TextKey::NoticePickContext => "Elige una canción, un álbum o una Playlist",
        TextKey::NoticeLinkCopied => "Enlace copiado",
        TextKey::NoticeRestartingPlayback => "Reiniciando la reproducción local",
        TextKey::NoticeAudioDisconnected => {
            "El audio de Spotify se desconectó. Volviendo a conectar la reproducción local"
        }
        TextKey::NoticePremiumRequired => "La reproducción local requiere Spotify Premium",
        TextKey::NoticeOpeningPlaybackSetup => {
            "Abriendo el navegador para configurar la reproducción local"
        }
        TextKey::NoticeHistoryCleared => "Se borró el historial de reproducción",
        TextKey::NoticePersonalAppNudge => {
            "Spotify está tardando. Configura una aplicación personal en Configuración para tener una cuota de API aparte"
        }
        TextKey::NoticeSignInExpired => "Tu sesión de Spotify venció. Inicia sesión de nuevo.",
        TextKey::NoticeUpdateCheckFailedPrefix => "No se pudieron buscar actualizaciones",
        TextKey::NoticeLocalPlaybackPrefix => "Reproducción local",
        TextKey::NoticeProfileLoadFailedPrefix => "No se pudo cargar tu perfil",
        TextKey::NoticeListDevicesFailedPrefix => "No se pudieron obtener los dispositivos",
        TextKey::NoticeLoadMorePlaylistsFailedPrefix => "No se pudieron cargar más Playlists",
        TextKey::NoticeCreatePlaylistFailedPrefix => "No se pudo crear la Playlist",
        TextKey::NoticeUpdatePlaylistFailedPrefix => "No se pudo actualizar la Playlist",
        TextKey::NoticePlaylistChangeFailedPrefix => "No se pudo cambiar la Playlist",
        TextKey::NoticeLibraryUpdateFailedPrefix => "No se pudo actualizar tu biblioteca",
        TextKey::NoticeCannotOpenSongPrefix => "No se puede abrir esta canción",
        TextKey::NoticeCannotOpenEpisodePrefix => "No se puede abrir este episodio",
        TextKey::NoticeSwitchDeviceFailedPrefix => "No se pudo cambiar de dispositivo",
        TextKey::NoticeAddQueueFailedPrefix => "No se pudo añadir a la cola",
        TextKey::NoticeClearArtworkFailedPrefix => "No se pudieron borrar las portadas",
        TextKey::NoticeChooseDeviceHint => {
            "Elige primero un dispositivo en el menú de dispositivos."
        }
        TextKey::NoticeRemoteStartFailed => "No se pudo iniciar la reproducción",
        TextKey::NoticeRemotePauseFailed => "No se pudo pausar",
        TextKey::NoticeRemoteNextFailed => "No se pudo saltar",
        TextKey::NoticeRemotePreviousFailed => "No se pudo volver atrás",
        TextKey::NoticeRemoteSeekFailed => "No se pudo cambiar la posición",
        TextKey::NoticeRemoteVolumeFailed => "No se pudo cambiar el volumen",
        TextKey::NoticeRemoteShuffleFailed => "No se pudo cambiar el orden aleatorio",
        TextKey::NoticeRemoteRepeatFailed => "No se pudo cambiar la repetición",
        TextKey::NoticeLocalPlaybackNotSetUp => {
            "La reproducción local aún no está configurada en este equipo"
        }
        TextKey::NoticeSpotifyPermissionsChanged => {
            "Los permisos de Spotify cambiaron. Inicia sesión de nuevo."
        }
        TextKey::NoticeSpotifyAccountsDiffer => {
            "Las autorizaciones de Spotify pertenecen a cuentas distintas"
        }
        TextKey::NoticePersonalClientIdRequired => "se requiere un Client ID personal de Spotify",
        TextKey::NoticeRemovedFromPlaylist => "Se quitó de la Playlist",
        TextKey::NoticePlaybackFailedPrefix => "Error de reproducción",
        TextKey::NoticeSharedSignInFailedPrefix => {
            "Falló el inicio de sesión compartido de Spotify"
        }
        TextKey::NoticePersonalAuthorizationFailedPrefix => {
            "Falló la autorización de la aplicación personal"
        }
        TextKey::NoticeSignInFailedPrefix => "Falló el inicio de sesión",
        TextKey::TrayShowHide => "Mostrar u ocultar Fastpotify",
        TextKey::TrayPlay => "Reproducir",
        TextKey::TrayPause => "Pausar",
        TextKey::TrayNext => "Siguiente",
        TextKey::TrayPrevious => "Anterior",
        TextKey::TrayQuit => "Salir",
        TextKey::AuthSuccessTitle => "Sesión iniciada en Fastpotify",
        TextKey::AuthSuccessHeading => "Iniciaste sesión",
        TextKey::AuthSuccessBody => "Puedes cerrar esta pestaña y volver a Fastpotify.",
        TextKey::AuthFailureTitle => "Falló el inicio de sesión",
        TextKey::AuthFailureHeading => "No se completó el inicio de sesión",
        TextKey::AuthFailureReturn => "Vuelve a Fastpotify e inténtalo de nuevo.",
        TextKey::MilkdropPresetKept => "Preajuste fijado",
        TextKey::MilkdropPresetFree => "Preajuste liberado",
        TextKey::MilkdropRandomOrder => "Orden aleatorio",
        TextKey::MilkdropFolderOrder => "Orden de la carpeta",
        TextKey::MilkdropSongWhenChanged => "Título de la canción: al cambiar",
        TextKey::MilkdropSongAlways => "Título de la canción: siempre",
        TextKey::MilkdropSongOff => "Título de la canción: desactivado",
        TextKey::MilkdropNothingPlaying => "No hay nada en reproducción",
        TextKey::MilkdropNoPreset => "Sin preajuste",
        TextKey::MilkdropHelpPresets => "PREAJUSTES",
        TextKey::MilkdropHelpPlayback => "REPRODUCCIÓN",
        TextKey::MilkdropHelpWindow => "VENTANA",
        TextKey::MilkdropHelpShow => "MOSTRAR",
        TextKey::MilkdropHelpNextPreset => "Siguiente preajuste",
        TextKey::MilkdropHelpPreviousPreset => "Preajuste anterior",
        TextKey::MilkdropHelpBeatCut => "Siguiente preajuste, cambio con el ritmo",
        TextKey::MilkdropHelpKeepPreset => "Mantener este preajuste",
        TextKey::MilkdropHelpOrder => "Orden aleatorio o de carpeta",
        TextKey::MilkdropHelpRightClick => "Siguiente preajuste",
        TextKey::MilkdropHelpPlayPause => "Reproducir o pausar",
        TextKey::MilkdropHelpPreviousNextSong => "Canción anterior o siguiente",
        TextKey::MilkdropHelpVolume => "Subir o bajar el volumen",
        TextKey::MilkdropHelpMute => "Silenciar o activar el sonido",
        TextKey::MilkdropHelpLike => "Añadir o quitar la canción de Canciones favoritas",
        TextKey::MilkdropHelpShuffle => "Orden aleatorio",
        TextKey::MilkdropHelpFullscreen => "Pantalla completa",
        TextKey::MilkdropHelpLeaveFullscreen => "Salir de pantalla completa o cerrar",
        TextKey::MilkdropHelpMoveResize => "Mover; arrastra una esquina para cambiar el tamaño",
        TextKey::MilkdropHelpTheseKeys => "Mostrar estas teclas",
        TextKey::MilkdropHelpSongTitle => "Título de la canción: al cambiar, siempre o desactivado",
        TextKey::MilkdropHelpPresetName => "Nombre del preajuste: activado o desactivado",
        TextKey::MilkdropHelpFps => "FPS: activados o desactivados",
        TextKey::MacMenuCheckUpdates => "Buscar actualizaciones…",
        TextKey::MacMenuSettings => "Configuración…",
        TextKey::MacMenuFile => "Archivo",
        TextKey::MacMenuCloseWindow => "Cerrar ventana",
        TextKey::MacMenuEdit => "Editar",
        TextKey::MacMenuCut => "Cortar",
        TextKey::MacMenuCopy => "Copiar",
        TextKey::MacMenuPaste => "Pegar",
        TextKey::MacMenuSelectAll => "Seleccionar todo",
        TextKey::MacMenuPlayback => "Reproducción",
        TextKey::MacMenuPlayPause => "Reproducir / Pausar",
        TextKey::MacMenuNextTrack => "Siguiente canción",
        TextKey::MacMenuPreviousTrack => "Canción anterior",
        TextKey::MacMenuSeekForward => "Avanzar (10s)",
        TextKey::MacMenuSeekBackward => "Retroceder (10s)",
        TextKey::MacMenuShuffle => "Orden aleatorio",
        TextKey::MacMenuRepeat => "Repetir",
        TextKey::MacMenuIncreaseVolume => "Subir volumen",
        TextKey::MacMenuDecreaseVolume => "Bajar volumen",
        TextKey::MacMenuMute => "Silenciar",
        TextKey::MacMenuView => "Ver",
        TextKey::MacMenuBack => "Atrás",
        TextKey::MacMenuForward => "Adelante",
        TextKey::MacMenuHome => "Inicio",
        TextKey::MacMenuSearch => "Buscar",
        TextKey::MacMenuLikedSongs => "Canciones favoritas",
        TextKey::MacMenuToggleSidebar => "Mostrar u ocultar la barra lateral",
        TextKey::MacMenuQueue => "Cola",
        TextKey::MacMenuToggleFullscreen => "Activar o desactivar pantalla completa",
        TextKey::MacMenuWindow => "Ventana",
        TextKey::MacMenuMinimize => "Minimizar",
        TextKey::MacMenuZoom => "Zoom",
        TextKey::MacMenuBringAllToFront => "Traer todo al frente",
        TextKey::MacMenuHelp => "Ayuda",
        TextKey::MacMenuKeyboardShortcuts => "Atajos de teclado",
        TextKey::MacMenuGithub => "Fastpotify en GitHub",
        TextKey::NoticeDownloadingMilkdropPacks => "Descargando paquetes de preajustes de MilkDrop",
        TextKey::NoticeFetchPresetsFailedPrefix => "No se pudieron obtener los preajustes",
    }
}

pub(super) fn message(message: &Message) -> String {
    match message {
        Message::SettingsConnectedAs { username } => format!("Sesión iniciada como {username}"),
        Message::SettingsPlaybackStatus { status } => format!("Estado: {status}"),
        Message::SettingsSkinFolder { path } => {
            format!("Los skins instalados están en {path}. Encuentra más en Winamp Skin Museum.")
        }
        Message::SettingsPresetFolder { count, path } => {
            let count = match count {
                0 => "Ninguno por ahora".to_string(),
                1 => "Un preajuste".to_string(),
                count => format!("{count} preajustes"),
            };
            format!(
                "{count} en {path}. Añade aquí archivos .milk. Fastpotify descarga preajustes cuando MilkDrop se abre por primera vez con una carpeta vacía."
            )
        }
        Message::SettingsGetPresetPack { name } => format!("Obtener {name}"),
        Message::SettingsScreenRefreshRate { hz } => format!(
            "Tu pantalla se actualiza a {hz} Hz. Las tasas más altas no añaden fotogramas visibles. Sin límite dibuja tan rápido como sea posible."
        ),
        Message::SettingsStoredIn { path } => format!("Guardado en {path}"),
        Message::SettingsHistoryStoredIn { path } => {
            format!(
                "Las canciones reproducidas aquí se guardan en {path}. Este archivo nunca se sube."
            )
        }
        Message::SettingsCredentialsStoredIn { path } => {
            format!("Las credenciales se guardan en {path}")
        }
        Message::SettingsVersion { version } => format!("Fastpotify {version}"),
        Message::SettingsFps { rate } => format!("{rate} fps"),
        Message::SettingsFpsYourScreen { rate } => format!("{rate} fps, tu pantalla"),
        Message::LoginFooter { version } => {
            format!("Fastpotify {version} • no está afiliado con Spotify")
        }
        Message::LyricsFetchFailed { detail } => {
            format!("No se pudo obtener la letra: {detail}")
        }
        Message::DeviceThisComputer { name } => format!("{name} (este equipo)"),
        Message::PlayingOnDevice { name } => format!("Reproduciendo en {name}"),
        Message::UpdateToVersion { version } => format!("Actualizar a {version}"),
        Message::UpdateAvailableDetail { version } => {
            format!("La versión {version} está disponible. Abre la página de descargas.")
        }
        Message::FollowerCount { count } => match count {
            1 => "1 seguidor".to_string(),
            count => format!("{} seguidores", crate::util::format_count(*count)),
        },
        Message::EpisodeCount { count } => match count {
            1 => "1 episodio".to_string(),
            count => format!("{count} episodios"),
        },
        Message::EpisodeTimeLeft { time } => format!("Quedan {time}"),
        Message::SearchNoResults { query } => format!("No hay resultados para “{query}”"),
        Message::SearchSongBy { artist } => format!("Canción • {artist}"),
        Message::SearchAlbumBy { artist } => format!("Álbum • {artist}"),
        Message::SearchPlaylistBy { owner } => format!("Playlist • {owner}"),
        Message::SearchPodcastBy { publisher } => format!("Pódcast • {publisher}"),
        Message::ByName { name } => format!("De {name}"),
        Message::AlbumYearKind { year, kind } => format!("{year} • {kind}"),
        Message::DeletePlaylistDetail { name } => {
            format!("¿Eliminar “{name}”? Puedes recuperarla desde Spotify durante 90 días.")
        }
        Message::RemovePlaylistDetail { name } => {
            format!("“{name}” dejará de aparecer en tu biblioteca.")
        }
        Message::PlaylistSongsAdded { count } => match count {
            1 => "Se añadirá 1 canción.".to_string(),
            count => format!("Se añadirán {count} canciones."),
        },
        Message::DuplicateSongs {
            playlist_name,
            names,
            selected_count,
        } => {
            let named = match names.as_slice() {
                [] => "Esta canción".to_string(),
                [name] => format!("“{name}”"),
                [first, second] => format!("“{first}” y “{second}”"),
                [first, second, rest @ ..] => {
                    format!("“{first}”, “{second}” y {} más", rest.len())
                }
            };
            let verb = if names.len() <= 1 {
                "ya está"
            } else {
                "ya están"
            };
            let question = if *selected_count == 1 {
                "¿Quieres añadirla de nuevo?"
            } else {
                "¿Quieres añadir todas las canciones seleccionadas de todos modos?"
            };
            format!("{named} {verb} en “{playlist_name}”. {question}")
        }
        Message::SidebarFolderPlaylistCount { count } => match count {
            1 => "Carpeta • 1 Playlist".to_string(),
            count => format!("Carpeta • {count} Playlists"),
        },
        Message::SidebarPlaylistSongCount { count } => match count {
            1 => "Playlist • 1 canción".to_string(),
            count => format!("Playlist • {count} canciones"),
        },
        Message::SidebarPlaylistBy { owner } => format!("Playlist • {owner}"),
        Message::SidebarPodcastBy { publisher } => format!("Pódcast • {publisher}"),
        Message::SidebarAlbumBy { kind, artists } => format!("{kind} • {artists}"),
        Message::SidebarFolderState { name, collapsed } => format!(
            "{name}, carpeta, {}",
            if *collapsed {
                "contraída"
            } else {
                "expandida"
            }
        ),
        Message::SidebarPlayItem { name } => format!("Reproducir {name}"),
        Message::MenuSelectionCount { count } => match count {
            1 => "1 canción".to_string(),
            count => format!("{count} canciones"),
        },
        Message::SortBy { label } => format!("Ordenar por {label}"),
        Message::CollectionNamedContributors { names } => {
            format!("con {}", names.join(" y "))
        }
        Message::CollectionOtherContributors { count } => match count {
            1 => "y 1 más".to_string(),
            count => format!("y {count} más"),
        },
        Message::CollectionSongCount { count } => match count {
            1 => "1 canción".to_string(),
            count => format!("{} canciones", crate::util::format_count(*count)),
        },
        Message::CollectionSongCountDuration { count, duration } => match count {
            1 => format!("1 canción, {duration}"),
            count => format!(
                "{} canciones, {duration}",
                crate::util::format_count(*count)
            ),
        },
        Message::DurationHoursMinutes { hours, minutes } => {
            format!("{hours} h {minutes} min")
        }
        Message::DurationMinutesSeconds { minutes, seconds } => {
            format!("{minutes} min {seconds} s")
        }
        Message::DurationSeconds { seconds } => format!("{seconds} s"),
        Message::EpisodeHoursMinutes { hours, minutes } => {
            format!("{hours} h {minutes} min")
        }
        Message::EpisodeMinutes { minutes } => format!("{minutes} min"),
        Message::DateDay { month, day, year } => format!("{day} de {month} de {year}"),
        Message::DateMonthYear { month, year } => format!("{month} de {year}"),
        Message::RelativeSeconds { count } => relative(*count, "segundo"),
        Message::RelativeMinutes { count } => relative(*count, "minuto"),
        Message::RelativeHours { count } => relative(*count, "hora"),
        Message::RelativeDays { count } => relative(*count, "día"),
        Message::RelativeWeeks { count } => relative(*count, "semana"),
        Message::NoticeDetail { prefix, detail } => format!("{}: {detail}", text(*prefix)),
        Message::NoticeReceiverReady { name } => format!("{name} está listo"),
        Message::NoticeReceiverFailed { name, detail } => format!("{name}: {detail}"),
        Message::NoticeUpdateAvailable { version } => {
            format!("Fastpotify {version} está disponible")
        }
        Message::NoticePlaylistCreated { name } => format!("Se creó {name}"),
        Message::NoticeItemPlayNext { name } => format!("{name} se reproducirá a continuación"),
        Message::NoticeSongsPlayNext { count } => match count {
            1 => "1 canción se reproducirá a continuación".to_string(),
            count => format!("{count} canciones se reproducirán a continuación"),
        },
        Message::NoticeArtworkCleared { megabytes } => {
            format!("Se borraron {megabytes} MB de portadas")
        }
        Message::NoticeRemoteActionFailed {
            action,
            detail,
            choose_device,
        } => {
            if *choose_device {
                format!(
                    "{action}: {detail}. {}",
                    text(TextKey::NoticeChooseDeviceHint)
                )
            } else {
                format!("{action}: {detail}.")
            }
        }
        Message::NoticeText { key } => text(*key).to_string(),
        Message::NoticeAddedToPlaylist { name } => format!("Se añadió a {name}"),
        Message::TrackPlayAccessibility { name, subtitle } => {
            format!("Reproducir {name}, {subtitle}")
        }
        Message::QueueRadioPlaylistName { track } => format!("Radio de {track}"),
        Message::QueuePlaylistName { date } => format!("Cola {date}"),
        Message::NoticeMilkdropPresetsAdded { count } => match count {
            1 => "Se añadió 1 preajuste de MilkDrop".to_string(),
            count => format!("Se añadieron {count} preajustes de MilkDrop"),
        },
        Message::NoticeDownloadingPresetPack { name } => {
            format!("Descargando los preajustes de {name}")
        }
        Message::NoticeSkinAdded { name } => format!("Se añadió el skin {name}"),
    }
}

fn relative(count: i64, unit: &str) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("hace {count} {unit}{plural}")
}
