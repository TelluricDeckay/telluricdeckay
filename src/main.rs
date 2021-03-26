mod config;
mod game;
mod gui;
mod player;
mod config_h;
use iced::{window::Settings, Sandbox, Error};

// localization
use i18n_embed::{DesktopLanguageRequester,
    gettext::gettext_language_loader};
use rust_embed::RustEmbed;
// import macros
use tr::tr_init;

#[derive(RustEmbed)]
#[folder = "i18n/mo"] // path to the compiled localization resources
struct Translations;

fn main() -> Result<(), Error> {
    tr_init!(crate::config_h::get_localedir());
    let translations = Translations {};
    let language_loader = gettext_language_loader!();

    // Use the language requester for the desktop platform (linux, windows, mac).
    // There is also a requester available for the web-sys WASM platform called
    // WebLanguageRequester, or you can implement your own.
    let requested_languages = DesktopLanguageRequester::requested_languages();

    i18n_embed::select(&language_loader, &translations, &requested_languages);

    let mut settings = iced::Settings::default();
    settings.window = Settings {
        // size: (720, 480),
        icon: gui::asset_manager::get_icon(),
        ..Settings::default()
    };
    gui::Gui::run(settings)
}
