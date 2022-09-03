use std::process::{Command, Output};

pub enum DeviceCommand {
    List,
    Select(usize),
    Current,
}

impl DeviceCommand {
    pub fn execute(self) -> Option<Output> {
        match self {
            DeviceCommand::List => Some(list_devices()),
            DeviceCommand::Select(id) => Some(select_device(id)),
            DeviceCommand::Current => Some(current_device())
        }
    }
}

fn list_devices() -> Output {
    let cmd_output = Command::new("pactl")
        .arg("-f")
        .arg("json")
        .arg("list")
        .arg("sinks")
        .output()
        .expect("failed to execute pactl -f json list sinks");

    cmd_output
}

fn select_device(id: usize) -> Output {
    let cmd_output = Command::new("pactl")
        .arg("set-default-sink")
        .arg(id.to_string())
        .output()
        .expect("failed to execute pactl set-default-sink");

    cmd_output
}


fn current_device() -> Output {
    let cmd_output = Command::new("pactl")
        .arg("get-default-sink")
        .output()
        .expect("failed to execute pactl set-default-sink");

    cmd_output
}
