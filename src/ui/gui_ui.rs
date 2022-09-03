use crate::{device::SoundOutputs, ui::SoundOutputsUi};

pub struct GuiUi {}

impl SoundOutputsUi for GuiUi {
    fn ui_display(&self, _outputs: &SoundOutputs) {
        println!("Gui ui here ...");
    }
}
