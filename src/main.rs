mod config;
mod game;
mod gui;
mod player;
use i18n_embed::{DesktopLanguageRequester,
    gettext::gettext_language_loader};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/mo"] // path to the compiled localization resources
struct Translations;
use iced::{window::Settings, Sandbox, Error};

// import macros
use tr::tr_init;

fn main() -> Result<(), Error> {
    tr_init!("/usr/share/locale/");
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
