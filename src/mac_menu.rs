//! Native macOS application menu bar (File, Edit, View, Playback, Window, Help).

use crate::settings::LanguageChoice;

/// All Fastpotify-owned application menu titles, in menu order.
pub fn titles(language: LanguageChoice) -> Vec<&'static str> {
    use crate::i18n::TextKey::*;
    let translator = crate::i18n::Translator::new(language);
    [
        MacMenuCheckUpdates,
        MacMenuSettings,
        MacMenuFile,
        MacMenuCloseWindow,
        MacMenuEdit,
        MacMenuCut,
        MacMenuCopy,
        MacMenuPaste,
        MacMenuSelectAll,
        MacMenuPlayback,
        MacMenuPlayPause,
        MacMenuNextTrack,
        MacMenuPreviousTrack,
        MacMenuSeekForward,
        MacMenuSeekBackward,
        MacMenuShuffle,
        MacMenuRepeat,
        MacMenuIncreaseVolume,
        MacMenuDecreaseVolume,
        MacMenuMute,
        MacMenuView,
        MacMenuBack,
        MacMenuForward,
        MacMenuHome,
        MacMenuSearch,
        MacMenuLikedSongs,
        MacMenuToggleSidebar,
        MacMenuQueue,
        MacMenuToggleFullscreen,
        MacMenuWindow,
        MacMenuMinimize,
        MacMenuZoom,
        MacMenuBringAllToFront,
        MacMenuHelp,
        MacMenuKeyboardShortcuts,
        MacMenuGithub,
    ]
    .into_iter()
    .map(|key| translator.text(key))
    .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuCommand {
    PlayPause,
    Next,
    Previous,
    SeekForward,
    SeekBackward,
    ToggleShuffle,
    CycleRepeat,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    Home,
    Search,
    LikedSongs,
    Sidebar,
    Queue,
    Settings,
    CheckForUpdates,
    Shortcuts,
    Back,
    Forward,
    OpenRepo,
    Cut,
    Copy,
    Paste,
    SelectAll,
}

#[cfg(not(target_os = "macos"))]
pub fn init(_language: LanguageChoice) {}

#[cfg(not(target_os = "macos"))]
pub fn set_language(_language: LanguageChoice) {}

#[cfg(not(target_os = "macos"))]
pub fn set_waker(_wake: impl Fn() + Send + Sync + 'static) {}

#[cfg(not(target_os = "macos"))]
pub fn drain_commands() -> Vec<MenuCommand> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::titles;
    use crate::settings::LanguageChoice;

    #[test]
    fn every_application_menu_title_follows_language() {
        let english = titles(LanguageChoice::English);
        let spanish = titles(LanguageChoice::Spanish);
        assert_eq!(english.len(), 36);
        assert_eq!(spanish.len(), english.len());
        assert!(english.contains(&"Playback"));
        assert!(english.contains(&"Fastpotify on GitHub"));
        assert!(spanish.iter().all(|title| !title.is_empty()));
    }
}

#[cfg(target_os = "macos")]
pub use mac_impl::*;

#[cfg(target_os = "macos")]
mod mac_impl {
    use objc2::rc::Retained;
    use objc2::runtime::Sel;
    use objc2::{MainThreadOnly, define_class, sel};
    use objc2_app_kit::{NSApplication, NSEventModifierFlags, NSMenu, NSMenuItem};
    use objc2_foundation::{MainThreadMarker, NSObject, NSString, ns_string};
    use std::cell::{Cell, RefCell};
    use std::sync::Mutex;

    use super::{LanguageChoice, MenuCommand};
    use crate::i18n::{TextKey, Translator};

    static COMMANDS: Mutex<Vec<MenuCommand>> = Mutex::new(Vec::new());
    static WAKER: Mutex<Option<Box<dyn Fn() + Send + Sync>>> = Mutex::new(None);

    thread_local! {
        static LANGUAGE: Cell<LanguageChoice> = const { Cell::new(LanguageChoice::English) };
        static ITEMS: RefCell<Vec<(Retained<NSMenuItem>, TextKey)>> = const { RefCell::new(Vec::new()) };
    }

    pub fn set_language(language: LanguageChoice) {
        LANGUAGE.with(|current| current.set(language));
        let translator = Translator::new(language);
        ITEMS.with(|items| {
            for (item, key) in items.borrow().iter() {
                item.setTitle(&NSString::from_str(translator.text(*key)));
            }
        });
    }

    pub fn set_waker(wake: impl Fn() + Send + Sync + 'static) {
        if let Ok(mut w) = WAKER.lock() {
            *w = Some(Box::new(wake));
        }
    }

    fn push_command(cmd: MenuCommand) {
        if let Ok(mut list) = COMMANDS.lock() {
            list.push(cmd);
        }
        if let Ok(w) = WAKER.lock()
            && let Some(wake) = w.as_ref()
        {
            wake();
        }
    }

    pub fn drain_commands() -> Vec<MenuCommand> {
        if let Ok(mut list) = COMMANDS.lock() {
            std::mem::take(&mut *list)
        } else {
            Vec::new()
        }
    }

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "FastpotifyMenuHandler"]
        pub struct FastpotifyMenuHandler;

        impl FastpotifyMenuHandler {
            #[unsafe(method(openSettings:))]
            fn open_settings(&self, _sender: &NSObject) {
                push_command(MenuCommand::Settings);
            }

            #[unsafe(method(checkForUpdates:))]
            fn check_for_updates(&self, _sender: &NSObject) {
                push_command(MenuCommand::CheckForUpdates);
            }

            #[unsafe(method(playPause:))]
            fn play_pause(&self, _sender: &NSObject) {
                push_command(MenuCommand::PlayPause);
            }

            #[unsafe(method(nextTrack:))]
            fn next_track(&self, _sender: &NSObject) {
                push_command(MenuCommand::Next);
            }

            #[unsafe(method(previousTrack:))]
            fn previous_track(&self, _sender: &NSObject) {
                push_command(MenuCommand::Previous);
            }

            #[unsafe(method(seekForward:))]
            fn seek_forward(&self, _sender: &NSObject) {
                push_command(MenuCommand::SeekForward);
            }

            #[unsafe(method(seekBackward:))]
            fn seek_backward(&self, _sender: &NSObject) {
                push_command(MenuCommand::SeekBackward);
            }

            #[unsafe(method(toggleShuffle:))]
            fn toggle_shuffle(&self, _sender: &NSObject) {
                push_command(MenuCommand::ToggleShuffle);
            }

            #[unsafe(method(cycleRepeat:))]
            fn cycle_repeat(&self, _sender: &NSObject) {
                push_command(MenuCommand::CycleRepeat);
            }

            #[unsafe(method(volumeUp:))]
            fn volume_up(&self, _sender: &NSObject) {
                push_command(MenuCommand::VolumeUp);
            }

            #[unsafe(method(volumeDown:))]
            fn volume_down(&self, _sender: &NSObject) {
                push_command(MenuCommand::VolumeDown);
            }

            #[unsafe(method(toggleMute:))]
            fn toggle_mute(&self, _sender: &NSObject) {
                push_command(MenuCommand::ToggleMute);
            }

            #[unsafe(method(openHome:))]
            fn open_home(&self, _sender: &NSObject) {
                push_command(MenuCommand::Home);
            }

            #[unsafe(method(focusSearch:))]
            fn focus_search(&self, _sender: &NSObject) {
                push_command(MenuCommand::Search);
            }

            #[unsafe(method(openLikedSongs:))]
            fn open_liked_songs(&self, _sender: &NSObject) {
                push_command(MenuCommand::LikedSongs);
            }

            #[unsafe(method(toggleSidebar:))]
            fn toggle_sidebar(&self, _sender: &NSObject) {
                push_command(MenuCommand::Sidebar);
            }

            #[unsafe(method(toggleQueue:))]
            fn toggle_queue(&self, _sender: &NSObject) {
                push_command(MenuCommand::Queue);
            }

            #[unsafe(method(goBack:))]
            fn go_back(&self, _sender: &NSObject) {
                push_command(MenuCommand::Back);
            }

            #[unsafe(method(goForward:))]
            fn go_forward(&self, _sender: &NSObject) {
                push_command(MenuCommand::Forward);
            }

            #[unsafe(method(showShortcuts:))]
            fn show_shortcuts(&self, _sender: &NSObject) {
                push_command(MenuCommand::Shortcuts);
            }

            #[unsafe(method(openRepo:))]
            fn open_repo(&self, _sender: &NSObject) {
                push_command(MenuCommand::OpenRepo);
            }

            // The Edit items answer to this handler rather than to the
            // responder chain: winit's view implements none of the standard
            // editing selectors, so a menu item aimed there does nothing,
            // while its key equivalent still takes the chord away from the
            // window. Routed through egui, the same item and chord work.
            #[unsafe(method(editCut:))]
            fn edit_cut(&self, _sender: &NSObject) {
                push_command(MenuCommand::Cut);
            }

            #[unsafe(method(editCopy:))]
            fn edit_copy(&self, _sender: &NSObject) {
                push_command(MenuCommand::Copy);
            }

            #[unsafe(method(editPaste:))]
            fn edit_paste(&self, _sender: &NSObject) {
                push_command(MenuCommand::Paste);
            }

            #[unsafe(method(editSelectAll:))]
            fn edit_select_all(&self, _sender: &NSObject) {
                push_command(MenuCommand::SelectAll);
            }
        }
    );

    fn create_item(
        mtm: MainThreadMarker,
        title_key: TextKey,
        action: Option<Sel>,
        key: &NSString,
        masks: Option<NSEventModifierFlags>,
        target: Option<&NSObject>,
    ) -> Retained<NSMenuItem> {
        let title = LANGUAGE
            .with(|language| NSString::from_str(Translator::new(language.get()).text(title_key)));
        let item = unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(mtm.alloc(), &title, action, key)
        };
        if let Some(masks) = masks {
            item.setKeyEquivalentModifierMask(masks);
        }
        if let Some(target) = target {
            unsafe { item.setTarget(Some(target)) };
        }
        ITEMS.with(|items| items.borrow_mut().push((item.clone(), title_key)));
        item
    }

    fn create_menu(
        mtm: MainThreadMarker,
        title_key: TextKey,
    ) -> (Retained<NSMenuItem>, Retained<NSMenu>) {
        let title = LANGUAGE
            .with(|language| NSString::from_str(Translator::new(language.get()).text(title_key)));
        let container_item = unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(
                mtm.alloc(),
                &title,
                None,
                ns_string!(""),
            )
        };
        let menu = NSMenu::initWithTitle(mtm.alloc(), &title);
        menu.setAutoenablesItems(false);
        container_item.setSubmenu(Some(&menu));
        ITEMS.with(|items| items.borrow_mut().push((container_item.clone(), title_key)));
        (container_item, menu)
    }

    pub fn init(language: LanguageChoice) {
        set_language(language);
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let app = NSApplication::sharedApplication(mtm);
        let Some(menubar) = app.mainMenu() else {
            return;
        };

        static INITIALIZED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);
        if INITIALIZED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        let handler: Retained<FastpotifyMenuHandler> =
            unsafe { objc2::msg_send![mtm.alloc::<FastpotifyMenuHandler>(), init] };
        let target: &NSObject = &handler;

        // 1. Update and Settings items in app menu (first menu)
        if let Some(app_menu_item) = menubar.itemAtIndex(0)
            && let Some(app_menu) = app_menu_item.submenu()
        {
            let update_item = create_item(
                mtm,
                TextKey::MacMenuCheckUpdates,
                Some(sel!(checkForUpdates:)),
                ns_string!(""),
                None,
                Some(target),
            );
            let settings_item = create_item(
                mtm,
                TextKey::MacMenuSettings,
                Some(sel!(openSettings:)),
                ns_string!(","),
                None,
                Some(target),
            );
            let sep = NSMenuItem::separatorItem(mtm);
            app_menu.insertItem_atIndex(&update_item, 1);
            app_menu.insertItem_atIndex(&settings_item, 2);
            app_menu.insertItem_atIndex(&sep, 3);
        }

        // 2. File menu
        let (file_item, file_menu) = create_menu(mtm, TextKey::MacMenuFile);
        file_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuCloseWindow,
            Some(sel!(performClose:)),
            ns_string!("w"),
            None,
            None,
        ));
        menubar.addItem(&file_item);

        // 3. Edit menu. No Undo and Redo: egui's text fields handle Cmd+Z
        // themselves, and a menu item holding that chord would take it
        // from them.
        let (edit_item, edit_menu) = create_menu(mtm, TextKey::MacMenuEdit);
        edit_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuCut,
            Some(sel!(editCut:)),
            ns_string!("x"),
            None,
            Some(target),
        ));
        edit_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuCopy,
            Some(sel!(editCopy:)),
            ns_string!("c"),
            None,
            Some(target),
        ));
        edit_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuPaste,
            Some(sel!(editPaste:)),
            ns_string!("v"),
            None,
            Some(target),
        ));
        edit_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuSelectAll,
            Some(sel!(editSelectAll:)),
            ns_string!("a"),
            None,
            Some(target),
        ));
        menubar.addItem(&edit_item);

        // 4. Playback menu
        let (playback_item, playback_menu) = create_menu(mtm, TextKey::MacMenuPlayback);
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuPlayPause,
            Some(sel!(playPause:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuNextTrack,
            Some(sel!(nextTrack:)),
            &NSString::from_str("\u{F703}"), // Right arrow
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuPreviousTrack,
            Some(sel!(previousTrack:)),
            &NSString::from_str("\u{F702}"), // Left arrow
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        // Shift+arrow has no key equivalent here on purpose: a menu key
        // equivalent fires ahead of the focused view, so binding it would
        // take shift-arrow selection away from every text field. The window
        // handles the same chord itself, and only when nothing has focus.
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuSeekForward,
            Some(sel!(seekForward:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuSeekBackward,
            Some(sel!(seekBackward:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuShuffle,
            Some(sel!(toggleShuffle:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuRepeat,
            Some(sel!(cycleRepeat:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        playback_menu.addItem(&NSMenuItem::separatorItem(mtm));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuIncreaseVolume,
            Some(sel!(volumeUp:)),
            &NSString::from_str("\u{F700}"), // Up arrow
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuDecreaseVolume,
            Some(sel!(volumeDown:)),
            &NSString::from_str("\u{F701}"), // Down arrow
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        playback_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuMute,
            Some(sel!(toggleMute:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        menubar.addItem(&playback_item);

        // 5. View menu
        let (view_item, view_menu) = create_menu(mtm, TextKey::MacMenuView);
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuBack,
            Some(sel!(goBack:)),
            ns_string!("["),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuForward,
            Some(sel!(goForward:)),
            ns_string!("]"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&NSMenuItem::separatorItem(mtm));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuHome,
            Some(sel!(openHome:)),
            ns_string!("H"),
            Some(NSEventModifierFlags::Command | NSEventModifierFlags::Shift),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuSearch,
            Some(sel!(focusSearch:)),
            ns_string!("f"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuLikedSongs,
            Some(sel!(openLikedSongs:)),
            ns_string!("l"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuToggleSidebar,
            Some(sel!(toggleSidebar:)),
            ns_string!("b"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuQueue,
            Some(sel!(toggleQueue:)),
            ns_string!("u"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        view_menu.addItem(&NSMenuItem::separatorItem(mtm));
        view_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuToggleFullscreen,
            Some(sel!(toggleFullScreen:)),
            ns_string!("f"),
            Some(NSEventModifierFlags::Control | NSEventModifierFlags::Command),
            None,
        ));
        menubar.addItem(&view_item);

        // 6. Window menu
        let (window_item, window_menu) = create_menu(mtm, TextKey::MacMenuWindow);
        window_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuMinimize,
            Some(sel!(performMiniaturize:)),
            ns_string!("m"),
            Some(NSEventModifierFlags::Command),
            None,
        ));
        window_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuZoom,
            Some(sel!(performZoom:)),
            ns_string!(""),
            None,
            None,
        ));
        window_menu.addItem(&NSMenuItem::separatorItem(mtm));
        window_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuBringAllToFront,
            Some(sel!(arrangeInFront:)),
            ns_string!(""),
            None,
            None,
        ));
        menubar.addItem(&window_item);

        // 7. Help menu
        let (help_item, help_menu) = create_menu(mtm, TextKey::MacMenuHelp);
        help_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuKeyboardShortcuts,
            Some(sel!(showShortcuts:)),
            ns_string!("/"),
            Some(NSEventModifierFlags::Command),
            Some(target),
        ));
        help_menu.addItem(&create_item(
            mtm,
            TextKey::MacMenuGithub,
            Some(sel!(openRepo:)),
            ns_string!(""),
            None,
            Some(target),
        ));
        menubar.addItem(&help_item);

        // NSMenuItem does not retain its target, and this one has to answer
        // for as long as the menu bar exists. It is a single process-wide
        // object, so leaking it is the whole lifetime story.
        std::mem::forget(handler);
    }
}
