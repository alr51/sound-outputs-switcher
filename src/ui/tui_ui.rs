use crate::{device::SoundOutputs, ui::SoundOutputsUi};

pub struct TuiUi {}

impl SoundOutputsUi for TuiUi {
    fn ui_display(&self, _outputs: &SoundOutputs) {
        println!("Tui Ui here ...");
    }
}
