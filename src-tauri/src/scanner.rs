use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::time::Duration;

use tokio::time;
use serde_json::{Value, json};


pub struct Scanner {
    adapter: Adapter
}

impl Scanner {
    pub async fn new() -> Self {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let central = manager
            .adapters()
            .await
            .expect("Unable to fetch adapter list.")
            .into_iter()
            .nth(0)
            .expect("Unable to find adapters.");
        Self { adapter: central }
    }

    pub async fn scan(&self) -> Vec<Value> {
        self.adapter.start_scan(ScanFilter::default()).await.unwrap();
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = self.adapter.peripherals().await.unwrap();
        let mut peripherals_json = Vec::new();
        for peripheral in peripherals {
            let address = peripheral.address().to_string();
            let properties = peripheral.properties().await.unwrap().unwrap();
            let name = properties.local_name.unwrap_or("unknown".to_string());
            peripherals_json.push(json!({"address": address, "name": name}));
        }
        peripherals_json
    }


    pub async fn connect(&self, address: &str) -> Result<Peripheral, Box<dyn std::error::Error>> {
        self.adapter.start_scan(ScanFilter::default()).await.unwrap();
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = self.adapter.peripherals().await?;
    
        if let Some(peripheral) = peripherals.into_iter().find(|p| p.address().to_string() == address) {
            peripheral.connect().await?;
            Ok(peripheral)
        } else {
            Err(format!("Peripheral with address {} not found", address).into())
        }
    }
}