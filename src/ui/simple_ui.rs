use crate::{device::SoundOutputs, ui::SoundOutputsUi};
use std::io::{stdin, BufRead};

pub struct SimpleUi {}

impl SoundOutputsUi for SimpleUi {
    fn ui_display(&self, outputs: &SoundOutputs) {
        if let Some(ref devices) = outputs.devices {
            println!("Device list:");
            for (i, device) in devices.iter().enumerate() {
                let current = match outputs.is_current_device(device) {
                    true => "x",
                    false => " ",
                };
                println!("    [{}] {}. {}", current, i + 1, device.display_name());
            }

            let options = (1..=devices.len())
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            eprint!("Switch to [{}]: ", options.join("/"));

            let stdin = stdin();
            let line1 = stdin.lock().lines().next().unwrap().unwrap();

            if options.contains(&line1) {
                if let Ok(device_idx) = line1.parse::<usize>() {
                    let device_idx = device_idx - 1;
                    if device_idx < devices.len() {
                        let device = &devices[device_idx];
                        device.select();
                        println!("{} [SELECTED]", device.display_name());
                    }
                }
            } else {
                println!("Unknown device");
            }
        } else {
            println!("No device detected");
        }
    }
}
