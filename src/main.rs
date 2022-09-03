use clap::Parser;
use sound_outputs_switcher::{
    device::SoundOutputs,
    ui::{self, UserInterface},
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum)]
    interface: Option<UserInterface>,
}

fn main() {
    let args = Args::parse();

    let ui = ui::select_user_interface(args.interface.unwrap_or(UserInterface::Simple));
    let outputs = SoundOutputs::list();

    ui.ui_display(&outputs);

}
