use rdev::{EventType, Key};
use std::collections::HashSet;
use tauri::{DeviceEventFilter, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        // don't let tauri consume the events when focused
        .device_event_filter(DeviceEventFilter::Always)
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut pressed = HashSet::new();
                let callback = move |event: rdev::Event| match event.event_type {
                    EventType::KeyPress(key) if !pressed.contains(&key) => {
                        // we don't want to process the left control if we have already processed AltGr
                        if pressed.contains(&Key::AltGr) && key == Key::ControlLeft {
                            return;
                        }
                        let key_name = format!("{:?}", key);
                        let _ = handle.emit_to("main", "KeyPress", key_name);
                        pressed.insert(key);
                    }
                    EventType::KeyRelease(key) => {
                        let key_name = format!("{:?}", key);
                        let _ = handle.emit_to("main", "KeyRelease", key_name);
                        pressed.remove(&key);
                    }
                    _ => {}
                };
                rdev::listen(callback)
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
