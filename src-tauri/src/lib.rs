use rdev::{EventType, Key};
use std::{collections::HashSet, thread, time::Duration};
use tauri::{AppHandle, DeviceEventFilter, Emitter};

/// Simulate a key press and release event
#[tauri::command]
fn simulate_key(key: String) -> Result<(), &'static str> {
    let key = serde_json::from_str::<Key>(&format!("{:?}", key)).map_err(|_| "Invalid key")?;
    thread::spawn(move || {
        rdev::simulate(&EventType::KeyPress(key))
            .and_then(|_| {
                thread::sleep(Duration::from_millis(150));
                rdev::simulate(&EventType::KeyRelease(key))
            })
            .map_err(|_| "Failed to simulate key press")?;

        Ok::<_, String>(())
    });
    Ok(())
}

/// Emit a key event to the frontend
fn emit_key(handle: &AppHandle, event: &str, key: Key) -> tauri::Result<()> {
    let payload = format!("{:?}", key);
    handle.emit_to("main", event, payload)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        // don't let tauri consume the events when focused
        .device_event_filter(DeviceEventFilter::Always)
        .setup(|app| {
            let handle = app.handle().clone();
            thread::spawn(move || {
                let mut pressed = HashSet::new();
                let callback = move |event: rdev::Event| match event.event_type {
                    EventType::KeyPress(key) if !pressed.contains(&key) => {
                        // we don't want to process the left control if we have already processed AltGr
                        if pressed.contains(&Key::AltGr) && key == Key::ControlLeft {
                            return;
                        }
                        let _ = emit_key(&handle, "KeyPress", key);
                        pressed.insert(key);
                    }
                    EventType::KeyRelease(key) => {
                        let _ = emit_key(&handle, "KeyRelease", key);
                        pressed.remove(&key);
                    }
                    _ => {}
                };
                rdev::listen(callback)
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![simulate_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
