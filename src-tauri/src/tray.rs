use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTray, AppHandle, SystemTrayEvent, Manager};

pub fn create_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "Show the app");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_event(handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "show"=> show_the_main_app(&handle),
                "quit"=> {
                    handle.exit(0);
                },
                _=>{}
            }
        },
        SystemTrayEvent::DoubleClick { position: _, size: _, .. }=> show_the_main_app(&handle),
        _=>{}
    }
}

fn show_the_main_app(handle: &AppHandle) {
    let main = handle.get_window("main").unwrap();
                    main.show().unwrap();
                    main.set_focus().unwrap();
}