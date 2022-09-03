use std::collections::HashSet;

use crate::commands::DeviceCommand;
use serde::Deserialize;

#[derive(Debug)]
pub struct SoundOutputs {
    pub devices: Option<Vec<Device>>,
    pub current_device_name: Option<String>,
}

impl SoundOutputs {
    pub fn list() -> Self {
        
        // Search devices
        let cmd = DeviceCommand::List;
        let cmd_output = DeviceCommand::execute(cmd).expect("no command result");
        let sound_outputs = serde_json::from_slice(&cmd_output.stdout);
        let devices = match sound_outputs {
            Ok(outputs) => Some(outputs),
            Err(err) => {
                println!("Parsing Error !!!!! {} ", err);
                None
            }
        };

        // Search current device
        let mut current_device_name: Option<String> = None;
        let cmd = DeviceCommand::Current;
        let cmd_output = DeviceCommand::execute(cmd);

        if let Some(output) = cmd_output {
            current_device_name = match String::from_utf8(output.stdout) {
                Ok(val) => Some(val),
                Err(_) => None,
            };
        }

        Self {
            devices,
            current_device_name,
        }
    }

    pub fn is_current_device(&self, device: &Device) -> bool {
        match &self.current_device_name {
            Some(ref val) => val.trim().to_lowercase() == device.name.trim().to_lowercase(),
            None => false,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Device {
    pub index: usize,
    pub name: String,
    pub ports: Option<Vec<DevicePort>>,
    pub properties: DeviceProperties,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DevicePort {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeviceProperties {
    #[serde(rename(deserialize = "alsa.id"))]
    pub alsa_id: Option<String>,
    #[serde(rename(deserialize = "node.nick"))]
    pub node_nick: Option<String>,
    #[serde(rename(deserialize = "device.description"))]
    pub description: Option<String>,
}

impl Device {
    pub fn display_name(&self) -> String {
        let ports_name = match self.ports {
            Some(ref ports) => ports
                .iter()
                .map(|p| match &p.description {
                    Some(ref desc) => desc.into(),
                    None => String::from(""),
                })
                .collect::<Vec<String>>()
                .join(" / "),
            None => String::from(""),
        };

        format!("{} ({})", self.properties.get_hardware_name(), ports_name)
    }

    pub fn select(&self) {
        let cmd = DeviceCommand::Select(self.index);
        DeviceCommand::execute(cmd);
    }
}

impl DeviceProperties {
    fn get_hardware_name(&self) -> String {
        let mut hardware_names = HashSet::new();

        if let Some(ref id) = self.alsa_id {
            hardware_names.insert(id);
        }

        if let Some(ref nick) = self.node_nick {
            hardware_names.insert(nick);
        }

        if let Some(ref description) = self.description {
            hardware_names.insert(description);
        }

        hardware_names
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
