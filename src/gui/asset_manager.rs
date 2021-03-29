use crate::config;
use iced::{window::Icon, Length, Svg};
use image::{io::Reader as ImageReader, ImageResult, RgbaImage};
use ionic_deckhandler::{Card, Rank, Suit};

// TODO: these assets can take time to load. Perhaps look at way to cache them.
// Card display.
fn card_img(card_name: &str) -> Svg {
    Svg::from_path(format!("{}/assets/cards/{}", config::get_assetsdir(), card_name))
        .width(Length::Units(50))
        .height(Length::Units(50))
}

trait CardToImg {
    fn get_card_img(&self) -> Svg;
}

impl CardToImg for Card {
    fn get_card_img(&self) -> Svg {
        match *self {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            } => card_img("AS.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
            } => card_img("KS.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            } => card_img("QS.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
            } => card_img("JS.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
            } => card_img("TS.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
            } => card_img("9S.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            } => card_img("8S.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
            } => card_img("7S.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
            } => card_img("6S.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            } => card_img("5S.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            } => card_img("4S.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
            } => card_img("3S.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
            } => card_img("2S.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            } => card_img("AH.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            } => card_img("KH.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Hearts,
            } => card_img("QH.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
            } => card_img("JH.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Hearts,
            } => card_img("TH.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
            } => card_img("9H.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            } => card_img("8H.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Hearts,
            } => card_img("7H.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            } => card_img("6H.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            } => card_img("5H.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            } => card_img("4H.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            } => card_img("3H.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
            } => card_img("2H.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            } => card_img("AC.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            } => card_img("KC.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Clubs,
            } => card_img("QC.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            } => card_img("JC.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            } => card_img("TC.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            } => card_img("9C.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            } => card_img("8C.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Clubs,
            } => card_img("7C.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            } => card_img("6C.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Clubs,
            } => card_img("5C.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            } => card_img("4C.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            } => card_img("3C.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            } => card_img("2C.svg"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            } => card_img("AD.svg"),
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            } => card_img("KD.svg"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            } => card_img("QD.svg"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            } => card_img("JD.svg"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            } => card_img("TD.svg"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamonds,
            } => card_img("9D.svg"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            } => card_img("8D.svg"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamonds,
            } => card_img("7D.svg"),
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            } => card_img("6D.svg"),
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            } => card_img("5D.svg"),
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            } => card_img("4D.svg"),
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            } => card_img("3D.svg"),
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            } => card_img("2D.svg"),
        }
    }
}

pub(in crate::gui) trait HandToImgs {
    fn get_hand_imgs(&self) -> Vec<Svg>;
}

impl HandToImgs for [Card] {
    fn get_hand_imgs(&self) -> Vec<Svg> {
        self.iter().map(|c| c.get_card_img()).collect()
    }
}

fn get_rgba8img(filename: &str) -> ImageResult<RgbaImage> {
    Ok(ImageReader::open(filename)?.decode()?.to_rgba8())
}

pub fn get_icon() -> Option<Icon> {
    get_rgba8img(&format!("{}/telluricdeckay.png", config::get_pixmapsdir())).map_or_else(
        |e| {
            eprintln!("Could not load image: {:?}", e);
            None
        },
        |rgba8_img| {
            Icon::from_rgba(rgba8_img.to_vec(), rgba8_img.width(), rgba8_img.height()).map_or_else(
                |e| {
                    eprintln!("Could not load image: {:?}", e);
                    None
                },
                |icon| Some(icon),
            )
        },
    )
}
