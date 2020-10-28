mod config;
mod game;
mod player;
mod gui;
use iced::{window::Settings, Sandbox};
use std::io;

fn main() -> Result<(), io::Error> {
    let mut settings = iced::Settings::default();
    settings.window = Settings {
        // size: (720, 480),
        ..Settings::default()
    };
    gui::Gui::run(settings);
    Ok(())
}
