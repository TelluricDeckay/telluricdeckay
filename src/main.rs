mod config;
mod game;
mod gui;
mod player;
use iced::{window::{Settings, Icon}, Sandbox};
use std::io;

fn main() -> Result<(), io::Error> {
    let mut settings = iced::Settings::default();
    settings.window = Settings {
        // size: (720, 480),
        icon: gui::asset_manager::get_icon(),
        ..Settings::default()
    };
    gui::Gui::run(settings);
    Ok(())
}
