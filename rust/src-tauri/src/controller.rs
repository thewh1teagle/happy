use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use std::error::Error;
use serde::{Serialize, Deserialize}; // 1.0.124

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Mode {
    name: &'static str,
    value: u8,
}

pub static MODES: &[Mode] = &[
    Mode { name: "Normal", value: 36 },
    Mode { name: "PulsatingRainbow", value: 37 },
    Mode { name: "PulsatingRed", value: 38 },
    Mode { name: "PulsatingGreen", value: 39 },
    Mode { name: "PulsatingBlue", value: 40 },
    Mode { name: "PulsatingYellow", value: 41 },
    Mode { name: "PulsatingCyan", value: 42 },
    Mode { name: "PulsatingPurple", value: 43 },
    Mode { name: "PulsatingWhite", value: 44 },
    Mode { name: "PulsatingRedGreen", value: 45 },
    Mode { name: "PulsatingRedBlue", value: 46 },
    Mode { name: "PulsatingGreenBlue", value: 47 },
    Mode { name: "RainbowStrobe", value: 48 },
    Mode { name: "RedStrobe", value: 49 },
    Mode { name: "GreenStrobe", value: 50 },
    Mode { name: "BlueStrobe", value: 51 },
    Mode { name: "YellowStrobe", value: 52 },
    Mode { name: "CyanStrobe", value: 53 },
    Mode { name: "PurpleStrobe", value: 54 },
    Mode { name: "WhiteStrobe", value: 55 },
    Mode { name: "RainbowJumpingChange", value: 56 },
    Mode { name: "PulsatingRgb", value: 57 },
    Mode { name: "RgbJumpingChange", value: 58 },
];

pub struct Controller {
    peripheral: Option<Peripheral>,
    char: Option<Characteristic>,
}

impl Controller {

    pub fn new() -> Self {
        Self {
            peripheral: None,
            char: None
        }
    }

    pub fn set_peripheral(&mut self, p: &Peripheral) {
        self.peripheral = Some(p.clone());
    }

    pub async fn set_char(&mut self, c: &Characteristic) {
        self.char = Some(c.clone());
    }

    pub async fn disconnect(&self) {
        self.peripheral.as_ref().unwrap().disconnect().await.unwrap();
    }

    pub async fn set_power(&self, state: bool) -> Result<(), Box<dyn Error>> {
        let data: [u8; 3] = if state { [204, 35, 51] } else { [204, 36, 51] };
        self.peripheral
            .as_ref()
            .unwrap()
            .write(
                &self.char.as_ref().unwrap(),
                &data,
                WriteType::WithoutResponse,
            )
            .await?;
        Ok(())
    }

    pub async fn set_rgb(&self, r: u8, g: u8, b: u8, q: u8) -> Result<(), Box<dyn Error>> {
        let args: [u8; 7] = [
            86,
            r,
            g,
            b,
            q,
            255 - 15,
            255 - 85,
        ];
        let values: Vec<u8> = args.to_vec();

        self.peripheral
            .as_ref()
            .unwrap()
            .write(&self.char.as_ref().unwrap(), &values, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    pub async fn set_mode(&self, mode: u8, speed: u8) -> Result<(), Box<dyn Error>> {
        let data: [u8; 4] = [255 - 68, mode, speed & 0xFF, 68];
        self.peripheral
        .as_ref().unwrap()
            .write(&self.char.as_ref().unwrap(), &data, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }
}
