use crate::config_h;
use iced::{window::Icon, Length, Image};
use image::{io::Reader as ImageReader, ImageResult, RgbaImage};
use ionic_deckhandler::{Card, Rank, Suit};

// TODO: these assets can take time to load. Perhaps look at way to cache them.
// Card display.
fn card_img(card_name: &str) -> Image {
    Image::new(format!("{}/assets/cards/{}", config_h::get_assetsdir(), card_name))
        .width(Length::Units(60))
}

trait CardToImg {
    fn get_card_img(&self) -> Image;
}

impl CardToImg for Card {
    fn get_card_img(&self) -> Image {
        match *self {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            } => card_img("AS.png"),
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
            } => card_img("KS.png"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            } => card_img("QS.png"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
            } => card_img("JS.png"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
            } => card_img("TS.png"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
            } => card_img("9S.png"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            } => card_img("8S.png"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
            } => card_img("7S.png"),
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
            } => card_img("6S.png"),
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            } => card_img("5S.png"),
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            } => card_img("4S.png"),
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
            } => card_img("3S.png"),
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
            } => card_img("2S.png"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            } => card_img("AH.png"),
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            } => card_img("KH.png"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Hearts,
            } => card_img("QH.png"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
            } => card_img("JH.png"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Hearts,
            } => card_img("TH.png"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
            } => card_img("9H.png"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            } => card_img("8H.png"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Hearts,
            } => card_img("7H.png"),
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            } => card_img("6H.png"),
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            } => card_img("5H.png"),
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            } => card_img("4H.png"),
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            } => card_img("3H.png"),
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
            } => card_img("2H.png"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            } => card_img("AC.png"),
            Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            } => card_img("KC.png"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Clubs,
            } => card_img("QC.png"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            } => card_img("JC.png"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            } => card_img("TC.png"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            } => card_img("9C.png"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            } => card_img("8C.png"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Clubs,
            } => card_img("7C.png"),
            Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            } => card_img("6C.png"),
            Card {
                rank: Rank::Five,
                suit: Suit::Clubs,
            } => card_img("5C.png"),
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            } => card_img("4C.png"),
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            } => card_img("3C.png"),
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            } => card_img("2C.png"),
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            } => card_img("AD.png"),
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            } => card_img("KD.png"),
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            } => card_img("QD.png"),
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            } => card_img("JD.png"),
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            } => card_img("TD.png"),
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamonds,
            } => card_img("9D.png"),
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            } => card_img("8D.png"),
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamonds,
            } => card_img("7D.png"),
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            } => card_img("6D.png"),
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            } => card_img("5D.png"),
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            } => card_img("4D.png"),
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            } => card_img("3D.png"),
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            } => card_img("2D.png"),
        }
    }
}

pub(in crate::gui) trait HandToImgs {
    fn get_hand_imgs(&self) -> Vec<Image>;
}

impl HandToImgs for [Card] {
    fn get_hand_imgs(&self) -> Vec<Image> {
        self.iter().map(|c| c.get_card_img()).collect()
    }
}

fn get_rgba8img(filename: &str) -> ImageResult<RgbaImage> {
    Ok(ImageReader::open(filename)?.decode()?.to_rgba8())
}

pub fn get_icon() -> Option<Icon> {
    get_rgba8img(&format!("{}/telluricdeckay.png", config_h::get_pixmapsdir())).map_or_else(
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
