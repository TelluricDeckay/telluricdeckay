mod config;
mod game;
mod player;
use iced::{window::Settings, Sandbox};
use std::io;
mod gui;

fn main() -> Result<(), io::Error> {
    let mut settings = iced::Settings::default();
    settings.window = Settings {
        size: (640, 480),
        ..Settings::default()
    };
    gui::Gui::run(settings);
    Ok(())
}
