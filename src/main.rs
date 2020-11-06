extern crate btleplug;
extern crate rand;

use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::ConnectedAdapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
use btleplug::api::{UUID, Central, Peripheral};

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

struct Commands {
    power_on: [u8;9],
    power_off: [u8;9]
}

const COMMANDS: Commands = Commands {
    power_on: [0x7e,0x04,0x04,0xf0,0x00,0x01,0xff,0x00,0xef],
    power_off: [0x7e,0x04,0x04,0x00,0x00,0x00,0xff,0x00,0xef],
};

pub fn main() {

    let manager = Manager::new().unwrap();
    let central = get_central(&manager);

    central.start_scan().unwrap();
    thread::sleep(Duration::from_secs(1));

    let address = btleplug::api::BDAddr{address: [0x3B,0x56,0x01,0x50,0x89,0xBE]};
    loop {
        match central.peripheral(address) {
            Some(device) => {
                ////////// CONNECTION AND FETCHING OF CHARACTERISTICS
                while device.connect().is_err(){
                    thread::sleep(Duration::from_millis(100));
                };
                central.stop_scan().unwrap();
                let characteristics = device.discover_characteristics().unwrap();
                let char = characteristics.iter().find(|c| {
                    c.uuid.to_string() == "00:00:FF:F3:00:00:10:00:80:00:00:80:5F:9B:34:FB"
                }).unwrap();
                //////////////////////////////////////////////////////

                for _ in 0..10 {

                    device.command(char, &COMMANDS.power_off).unwrap();
                    thread::sleep(Duration::from_millis(250));
            
                    device.command(char, &COMMANDS.power_on).unwrap();
                    thread::sleep(Duration::from_millis(250));
                }

                break;
            },
            _ => {}
        }
    }
    
}