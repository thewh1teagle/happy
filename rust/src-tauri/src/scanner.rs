use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::time::Duration;

use tokio::time;
use serde_json::{Value, json};

pub struct Scanner {
    adapter: Adapter
}

impl Scanner {
    pub async fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let central = manager
            .adapters()
            .await
            .expect("Unable to fetch adapter list.")
            .into_iter()
            .nth(0)
            .unwrap();
        Ok(Self { adapter: central })
    }

    pub async fn scan(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        self.adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = self.adapter.peripherals().await?;
        let mut peripherals_json = Vec::new();
        for peripheral in peripherals {
            let id = peripheral.id().to_string();
            let properties = peripheral.properties().await?.unwrap_or_default();
            let name = properties.local_name.unwrap_or("unknown".to_string());
            peripherals_json.push(json!({"id": id, "name": name}));
        }
        Ok(peripherals_json)
    }


    pub async fn connect(&self, id: &str) -> Result<Peripheral, Box<dyn std::error::Error>> {
        self.adapter.start_scan(ScanFilter::default()).await.unwrap();
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = self.adapter.peripherals().await?;
    
        if let Some(peripheral) = peripherals.into_iter().find(|p| p.id().to_string() == id) {
            peripheral.connect().await?;
            Ok(peripheral)
        } else {
            Err(format!("Peripheral with address {} not found", id).into())
        }
    }
}