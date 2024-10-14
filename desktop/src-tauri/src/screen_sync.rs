use scrap::Capturer;
use scrap::Display;
use tauri::State;
use tokio::sync::MutexGuard;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{self, sync::mpsc::{Sender, Receiver}};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;

use crate::controller;


pub fn frames_task(sync_state: &AtomicBool, tx: Sender<(f32, f32, f32)>) {
    let d = Display::primary().unwrap();
    let mut capturer = Capturer::new(d).unwrap();

    loop {
        if sync_state.load(Ordering::Relaxed) == false {
            break;
        }
        match capturer.frame() {
            Ok(frame) => {
                let mut total_r = 0.0;
                let mut total_g = 0.0;
                let mut total_b = 0.0;
                let mut total_a = 0.0;

                // Calculate the mean of r, g, and b for each pixel in the frame.
                for pixel in frame.chunks(4) {
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b = pixel[2] as f32;
                    let a = pixel[3] as f32;

                    // Accumulate the values.
                    total_r += r;
                    total_g += g;
                    total_b += b;
                    total_a += a;
                }
                tracing::debug!("rgba: {:?}", (total_r, total_g ,total_b ,total_a));

                // Calculate the mean values.
                let num_pixels = frame.len() as f32 / 4.0;
                let mean_r = total_r / num_pixels;
                let mean_g = total_g / num_pixels;
                let mean_b = total_b / num_pixels;


                tracing::debug!("tx: {:?}", (mean_r, mean_g, mean_b));
                // windows gives it in wrong order b, r, g
                tx.blocking_send((mean_b, mean_g, mean_r)).unwrap();
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub async fn controller_task(sync_state: State<'_, AtomicBool>, controller: MutexGuard<'_, controller::Controller>, mut rx: Receiver<(f32, f32, f32)>) {
    loop {
        if sync_state.load(Ordering::Relaxed) == false {
            break;
        }
        match rx.try_recv() {
            Ok(data) => {
                controller.set_rgb(data.0 as u8, data.1 as u8, data.2 as u8).await.unwrap();
                
            },
            Err(_) => {},
            
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}