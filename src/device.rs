extern crate btleplug;
extern crate rand;

use btleplug::api::{Central, Peripheral, UUID};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::ConnectedAdapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[cfg(target_os = "linux")]
fn get_central(manager: &Manager) -> ConnectedAdapter {
    let adapters = manager.adapters().unwrap();
    let adapter = adapters.into_iter().nth(0).unwrap();
    adapter.connect().unwrap()
}

pub struct Effects {
    pub jump_red_green_blue: u8,
    pub jump_red_green_blue_yellow_cyan_magenta_white: u8,
    pub crossfade_red: u8,
    pub crossfade_green: u8,
    pub crossfade_blue: u8,
    pub crossfade_yellow: u8,
    pub crossfade_cyan: u8,
    pub crossfade_magenta: u8,
    pub crossfade_white: u8,
    pub crossfade_red_green: u8,
    pub crossfade_red_blue: u8,
    pub crossfade_green_blue: u8,
    pub crossfade_red_green_blue: u8,
    pub crossfade_red_green_blue_yellow_cyan_magenta_white: u8,
    pub blink_red: u8,
    pub blink_green: u8,
    pub blink_blue: u8,
    pub blink_yellow: u8,
    pub blink_cyan: u8,
    pub blink_magenta: u8,
    pub blink_white: u8,
    pub blink_red_green_blue_yellow_cyan_magenta_white: u8,
}

pub const EFFECTS: Effects = Effects {
    jump_red_green_blue: 0x87,
    jump_red_green_blue_yellow_cyan_magenta_white: 0x88,
    crossfade_red: 0x8b,
    crossfade_green: 0x8c,
    crossfade_blue: 0x8d,
    crossfade_yellow: 0x8e,
    crossfade_cyan: 0x8f,
    crossfade_magenta: 0x90,
    crossfade_white: 0x91,
    crossfade_red_green: 0x92,
    crossfade_red_blue: 0x93,
    crossfade_green_blue: 0x94,
    crossfade_red_green_blue: 0x89,
    crossfade_red_green_blue_yellow_cyan_magenta_white: 0x8a,
    blink_red: 0x96,
    blink_green: 0x97,
    blink_blue: 0x98,
    blink_yellow: 0x99,
    blink_cyan: 0x9a,
    blink_magenta: 0x9b,
    blink_white: 0x9c,
    blink_red_green_blue_yellow_cyan_magenta_white: 0x95,
};

pub struct BleLedDevice {
    peripheral: btleplug::winrtble::peripheral::Peripheral,
    characteristics: Vec<btleplug::api::Characteristic>,
}

impl BleLedDevice {
    pub fn new() -> BleLedDevice {
        let manager = Manager::new().unwrap();
        let central = get_central(&manager);
        let mut characteristics;
        let peripheral;

        central.start_scan().unwrap();
        thread::sleep(Duration::from_secs(1));

        let address = btleplug::api::BDAddr {
            address: [0x3B, 0x56, 0x01, 0x50, 0x89, 0xBE],
        };
        loop {
            match central.peripheral(address) {
                Some(p) => {
                    peripheral = p;
                    ////////// CONNECTION AND FETCHING OF CHARACTERISTICS
                    while peripheral.connect().is_err() {
                        thread::sleep(Duration::from_millis(100));
                    }

                    central.stop_scan().unwrap();

                    characteristics = peripheral.discover_characteristics().unwrap();

                    let mut i = 0;
                    for _ in 0..characteristics.len() {
                        if characteristics.get(i).unwrap().uuid
                            != UUID::B128([
                                0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00,
                                0x00, 0xf3, 0xff, 0x00, 0x00,
                            ])
                        {
                            characteristics.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                    //////////////////////////////////////////////////////

                    break;
                }
                None => {}
            }
        }
        let device = BleLedDevice {
            peripheral,
            characteristics,
        };
        device.power_on();
        device
    }

    fn get_characteristic(&self) -> &btleplug::api::Characteristic {
        self.characteristics.get(0).unwrap()
    }

    pub fn power_on(&self) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[0x7e, 0x00, 0x04, 0xf0, 0x00, 0x01, 0xff, 0x00, 0xef],
            )
            .unwrap();
    }

    pub fn power_off(&self) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[0x7e, 0x00, 0x04, 0x00, 0x00, 0x00, 0xff, 0x00, 0xef],
            )
            .unwrap();
    }

    pub fn set_color(&self, red_value: u8, green_value: u8, blue_value: u8) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[
                    0x7e,
                    0x00,
                    0x05,
                    0x03,
                    red_value,
                    green_value,
                    blue_value,
                    0x00,
                    0xef,
                ],
            )
            .unwrap();
    }

    pub fn set_brightness(&self, value: u8) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[
                    0x7e,
                    0x00,
                    0x01,
                    value.min(0x64),
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0xef,
                ],
            )
            .unwrap();
    }

    pub fn set_effect(&self, value: u8) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[0x7e, 0x00, 0x03, value, 0x03, 0x00, 0x00, 0x00, 0xef],
            )
            .unwrap();
    }

    pub fn set_effect_speed(&self, value: u8) {
        self.peripheral
            .command(
                self.get_characteristic(),
                &[
                    0x7e,
                    0x00,
                    0x02,
                    value.min(0x64),
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0xef,
                ],
            )
            .unwrap();
    }
}
