mod simple_ui;
mod tui_ui;
mod gui_ui;

use crate::device::SoundOutputs;
use clap::ValueEnum;

use simple_ui::SimpleUi;
use tui_ui::TuiUi;
use gui_ui::GuiUi;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum UserInterface {
    Simple,
    Tui,
    Gui
}

pub trait SoundOutputsUi {
    fn ui_display(&self, outputs: &SoundOutputs);
}

pub fn select_user_interface(mode: UserInterface) -> Box<dyn SoundOutputsUi> {
    match mode {
        UserInterface::Simple => Box::new(SimpleUi{}),
        UserInterface::Tui => Box::new(TuiUi{}),
        UserInterface::Gui => Box::new(GuiUi{})
    }
}
