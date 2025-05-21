use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    DeviceEventFilter, Manager, WindowEvent,
};

mod keyboard;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // plugins
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_os::init());

    // only allow one instance of the app to run
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // focus the existing instance
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
        // don't let tauri consume the events when focused
        .device_event_filter(DeviceEventFilter::Always)
        .setup(|app| {
            // create tray icon
            #[cfg(desktop)]
            {
                let _tray = TrayIconBuilder::new()
                    // unwrap is safe here because we know the icon is present
                    .icon(app.default_window_icon().unwrap().clone())
                    .tooltip("animalese")
                    .menu({
                        &Menu::with_items(
                            app,
                            &[&MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?],
                        )?
                    })
                    .build(app)?;
            }

            // handle keypresses
            let handle = app.handle().clone();
            std::thread::spawn(move || keyboard::listen(handle));

            Ok(())
        })
        // prevent the window from closing when the user clicks the close button
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
            }
            _ => {}
        })
        .on_tray_icon_event(|app, event| {
            // show the window again when the tray icon is left clicked
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    if let Some(window) = app.get_webview_window("main") {
                        if !window.is_visible().unwrap_or(false) {
                            let _ = window
                                .center()
                                .and_then(|_| window.show())
                                .and_then(|_| window.set_focus());
                        }
                    }
                }
                _ => {}
            }
        })
        // handle tray icon menu events
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![keyboard::simulate_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
