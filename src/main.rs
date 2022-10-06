use std::env;
extern crate ui;
use ui::app::ClementineApp;

fn main() {
    println!("clementine v0.1.0");

    let args = env::args().skip(1).collect::<Vec<String>>();

    let cartridge_name = match args.first() {
        Some(name) => {
            println!("loading {name}");
            name.clone()
        }
        None => {
            println!("no cartridge found :(");
            std::process::exit(1)
        }
    };

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1000.0, 600.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "Clementine - A GBA Emulator",
        options,
        Box::new(|_cc| Box::new(ClementineApp::new(cartridge_name))),
    );
}
