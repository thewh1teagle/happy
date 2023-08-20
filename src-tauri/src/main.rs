// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::Peripheral;
use btleplug::api::CharPropFlags;

use tauri::async_runtime::Mutex;
use tauri::State;
mod controller;
mod scanner;

struct Scanner(Mutex<scanner::Scanner>);
struct Controller(Mutex<controller::Controller>);

#[tauri::command(async)]
async fn scan(scanner: State<'_, Scanner>) -> Result<Vec<serde_json::Value>, String> {
    let scanner = &scanner.0;
    let scanner = scanner.lock().await;
    let devices = scanner.scan().await;
    Ok(devices)
}

#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(async)]
async fn connect(address: &str , controller: State<'_, Controller>, scanner: State<'_, Scanner>) -> Result<i8, String> {
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

#[tokio::main]
async fn main() {
    let scanner = scanner::Scanner::new().await;
    let controller = controller::Controller::new();
    tauri::Builder::default()
        .manage(Scanner(Mutex::new(scanner)))
        .manage(Controller(Mutex::new(controller)))
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            scan,
            connect, 
            disconnect,
            set_power, 
            set_rgb,
            get_modes,
            set_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
