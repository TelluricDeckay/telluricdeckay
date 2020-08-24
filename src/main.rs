mod config;
mod game;
mod player;
use iced::Sandbox;
use std::io;
mod gui;

fn main() -> Result<(), io::Error> {
    gui::Gui::run(iced::Settings::default());

    Ok(())
}
