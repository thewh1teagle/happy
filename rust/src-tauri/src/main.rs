// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use btleplug::api::Peripheral;
use btleplug::api::CharPropFlags;
use tauri::Manager;
use tauri::async_runtime::Mutex;
use tauri::State;
mod controller;
mod scanner;
mod screen_sync;

struct ScannerState(tauri::async_runtime::Mutex<scanner::Scanner>);
struct Controller(tauri::async_runtime::Mutex<controller::Controller>);


#[tauri::command(async)]
async fn scan(scanner: State<'_, ScannerState>) -> Result<Vec<serde_json::Value>, String> {
    let scanner = &scanner.0;
    let scanner = scanner.lock().await;
    match scanner.scan().await {
        Ok(devices) => Ok(devices), // Return the list of devices if scanning succeeds
        Err(err) => Err(err.to_string()), // Convert the error to a String and return it
    }
}

#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    window.set_decorations(true).unwrap();
    window.maximize().unwrap();
    window.show().unwrap();
}

#[tauri::command(async)]
async fn connect(address: &str , controller: State<'_, Controller>, scanner: State<'_, ScannerState>) -> Result<i8, String> {
    let scanner = (&scanner.0).lock().await;
    let mut controller = (&controller.0).lock().await;
    let peripheral = scanner.connect(address).await.unwrap();
    peripheral.discover_services().await.unwrap();
    let characteristics = peripheral.characteristics();
    controller.set_peripheral(&peripheral);
    let char = characteristics.iter().find(|c| c.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE | CharPropFlags::WRITE)).unwrap();
    controller.set_char(char).await;
    Ok(0)
}


#[tauri::command(async)]
async fn set_power(state: bool ,controller: State<'_, Controller>) -> Result<i8, String> {
    let controller = (&controller.0).lock().await;
    controller.set_power(state).await.unwrap();
    Ok(0)
}


#[tauri::command(async)]
async fn set_rgb(r: u8 , g: u8, b: u8, controller: State<'_, Controller>) -> Result<i8, String> {
    let controller = (&controller.0).lock().await;
    controller.set_rgb(r, g, b).await.unwrap();
    Ok(0)
}

#[tauri::command(async)]
async fn set_mode(mode: u8, controller: State<'_, Controller>) -> Result<i8, String> {
    let controller = (&controller.0).lock().await;
    controller.set_mode(mode, 0).await.unwrap();
    Ok(0)
}

#[tauri::command(async)]
async fn get_modes() -> Vec<controller::Mode> {
    let modes = controller::MODES.to_vec();
    return modes;
}

#[tauri::command(async)]
async fn disconnect(controller: State<'_, Controller>) -> Result<i8, String> {
    let controller = (&controller.0).lock().await;
    controller.disconnect().await;
    Ok(0)
}

#[tauri::command(async)]
async fn toggle_screen_sync(app: tauri::AppHandle, controller: State<'_, Controller>) -> Result<(), String> {

    let app_clone = app.clone();
    let sync_state: State<'_, AtomicBool> = app_clone.state::<AtomicBool>();
    if sync_state.load(Ordering::Relaxed) == true {
        sync_state.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |s| Some(!s)).unwrap();
    } else {
        sync_state.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |s| Some(!s)).unwrap();
        let (tx, rx) = tokio::sync::mpsc::channel::<(f32, f32, f32)>(100);
        tokio::task::spawn_blocking( move || {
            let sync_state: State<'_, AtomicBool> = app.state::<AtomicBool>();
            screen_sync::frames_task(&sync_state, tx);
        });
        let controller = (&controller.0).lock().await;
        screen_sync::controller_task(sync_state, controller, rx).await;
    }
    Ok(())
    
    
}


#[tokio::main]
async fn main() {
    let sync_cancel_token: AtomicBool = AtomicBool::new(false);
    let scanner = scanner::Scanner::new().await;
    let controller = controller::Controller::new();
    tauri::Builder::default()
        .manage(ScannerState(Mutex::new(scanner)))
        .manage(Controller(Mutex::new(controller)))
        .manage(sync_cancel_token)
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            scan,
            connect, 
            disconnect,
            set_power, 
            set_rgb,
            get_modes,
            set_mode,
            toggle_screen_sync
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
