use crate::config_h;
use iced::{widget::svg::Handle, window::Icon, Length, Svg};
use image::{io::Reader as ImageReader, ImageResult, RgbaImage};
use ionic_deckhandler::{Card, Rank, Suit};

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub(in crate::gui) const CARD_FILES: [&str; 52] = [
    "AS.svg", "KS.svg", "QS.svg", "JS.svg", "TS.svg", "9S.svg", "8S.svg", "7S.svg", "6S.svg",
    "5S.svg", "4S.svg", "3S.svg", "2S.svg", "AH.svg", "KH.svg", "QH.svg", "JH.svg", "TH.svg",
    "9H.svg", "8H.svg", "7H.svg", "6H.svg", "5H.svg", "4H.svg", "3H.svg", "2H.svg", "AC.svg",
    "KC.svg", "QC.svg", "JC.svg", "TC.svg", "9C.svg", "8C.svg", "7C.svg", "6C.svg", "5C.svg",
    "4C.svg", "3C.svg", "2C.svg", "AD.svg", "KD.svg", "QD.svg", "JD.svg", "TD.svg", "9D.svg",
    "8D.svg", "7D.svg", "6D.svg", "5D.svg", "4D.svg", "3D.svg", "2D.svg",
];

// TODO: these assets can take time to load. Perhaps look at way to cache them.
// Card display.
fn card_img(card_name: &str) -> Svg {
    Svg::new(Handle::from_memory(read_bytes(&format!(
        "{}/assets/cards/{}",
        config_h::get_assetsdir(),
        card_name
    ))))
    .width(Length::Units(50))
    .height(Length::Units(50))
}

fn read_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect(&format!("cannot open: {}", filename));
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)
        .expect(&format!("cannot open: {}", filename));
    buffer
}

pub(in crate::gui) async fn async_card_img(card_name: &str) -> Vec<u8> {
    async_read_bytes(&format!(
        "{}/assets/cards/{}",
        config_h::get_assetsdir(),
        card_name
    ))
    .await
}

async fn async_read_bytes(filename: &str) -> Vec<u8> {
    use async_std::prelude::*;
    let mut f = async_std::fs::File::open(filename)
    .await
    .expect(&format!("cannot open: {}", filename));
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)
        .await
        .expect(&format!("cannot open: {}", filename));
    buffer
}

pub(in crate::gui) enum CardSvgOrText {
    Svg(Vec<u8>),
    Text(String),
}
pub(in crate::gui) struct AssetCacher {
    card_svgs: [CardSvgOrText; 52],
}

impl AssetCacher {
    pub(in crate::gui) fn new() -> AssetCacher {
        AssetCacher {
            card_svgs: [
                CardSvgOrText::Text("Ace of Spades".to_owned()),
                CardSvgOrText::Text("king of Spades".to_owned()),
                CardSvgOrText::Text("Queen of Spades".to_owned()),
                CardSvgOrText::Text("Jack of Spades".to_owned()),
                CardSvgOrText::Text("Ten of Spades".to_owned()),
                CardSvgOrText::Text("Nine of Spades".to_owned()),
                CardSvgOrText::Text("Eight of Spades".to_owned()),
                CardSvgOrText::Text("Seven of Spades".to_owned()),
                CardSvgOrText::Text("Six of Spades".to_owned()),
                CardSvgOrText::Text("Five of Spades".to_owned()),
                CardSvgOrText::Text("Four of Spades".to_owned()),
                CardSvgOrText::Text("Three of Spades".to_owned()),
                CardSvgOrText::Text("Two of Spades".to_owned()),
                CardSvgOrText::Text("Ace of Hearts".to_owned()),
                CardSvgOrText::Text("king of Hearts".to_owned()),
                CardSvgOrText::Text("Queen of Hearts".to_owned()),
                CardSvgOrText::Text("Jack of Hearts".to_owned()),
                CardSvgOrText::Text("Ten of Hearts".to_owned()),
                CardSvgOrText::Text("Nine of Hearts".to_owned()),
                CardSvgOrText::Text("Eight of Hearts".to_owned()),
                CardSvgOrText::Text("Seven of Hearts".to_owned()),
                CardSvgOrText::Text("Six of Hearts".to_owned()),
                CardSvgOrText::Text("Five of Hearts".to_owned()),
                CardSvgOrText::Text("Four of Hearts".to_owned()),
                CardSvgOrText::Text("Three of Hearts".to_owned()),
                CardSvgOrText::Text("Two of Hearts".to_owned()),
                CardSvgOrText::Text("Ace of Clubs".to_owned()),
                CardSvgOrText::Text("king of Clubs".to_owned()),
                CardSvgOrText::Text("Queen of Clubs".to_owned()),
                CardSvgOrText::Text("Jack of Clubs".to_owned()),
                CardSvgOrText::Text("Ten of Clubs".to_owned()),
                CardSvgOrText::Text("Nine of Clubs".to_owned()),
                CardSvgOrText::Text("Eight of Clubs".to_owned()),
                CardSvgOrText::Text("Seven of Clubs".to_owned()),
                CardSvgOrText::Text("Six of Clubs".to_owned()),
                CardSvgOrText::Text("Five of Clubs".to_owned()),
                CardSvgOrText::Text("Four of Clubs".to_owned()),
                CardSvgOrText::Text("Three of Clubs".to_owned()),
                CardSvgOrText::Text("Two of Clubs".to_owned()),
                CardSvgOrText::Text("Ace of Diamonds".to_owned()),
                CardSvgOrText::Text("king of Diamonds".to_owned()),
                CardSvgOrText::Text("Queen of Diamonds".to_owned()),
                CardSvgOrText::Text("Jack of Diamonds".to_owned()),
                CardSvgOrText::Text("Ten of Diamonds".to_owned()),
                CardSvgOrText::Text("Nine of Diamonds".to_owned()),
                CardSvgOrText::Text("Eight of Diamonds".to_owned()),
                CardSvgOrText::Text("Seven of Diamonds".to_owned()),
                CardSvgOrText::Text("Six of Diamonds".to_owned()),
                CardSvgOrText::Text("Five of Diamonds".to_owned()),
                CardSvgOrText::Text("Four of Diamonds".to_owned()),
                CardSvgOrText::Text("Three of Diamonds".to_owned()),
                CardSvgOrText::Text("Two of Diamonds".to_owned()),
            ],
        }
    }

    pub(in crate::gui) fn set_card_asset(&mut self, idx: usize, img: Vec<u8>) {
        self.card_svgs[idx] = CardSvgOrText::Svg(img);
    }

    pub(in crate::gui) fn get_card_asset(&self, card: &Card) -> &CardSvgOrText {
        &self.card_svgs[AssetCacher::get_card_index(card)]
    }

    fn get_card_index(card: &Card) -> usize {
        match *card {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            } => 0,
            Card {
                rank: Rank::King,
                suit: Suit::Spades,
            } => 1,
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            } => 2,
            Card {
                rank: Rank::Jack,
                suit: Suit::Spades,
            } => 3,
            Card {
                rank: Rank::Ten,
                suit: Suit::Spades,
            } => 4,
            Card {
                rank: Rank::Nine,
                suit: Suit::Spades,
            } => 5,
            Card {
                rank: Rank::Eight,
                suit: Suit::Spades,
            } => 6,
            Card {
                rank: Rank::Seven,
                suit: Suit::Spades,
            } => 7,
            Card {
                rank: Rank::Six,
                suit: Suit::Spades,
            } => 8,
            Card {
                rank: Rank::Five,
                suit: Suit::Spades,
            } => 9,
            Card {
                rank: Rank::Four,
                suit: Suit::Spades,
            } => 10,
            Card {
                rank: Rank::Three,
                suit: Suit::Spades,
            } => 11,
            Card {
                rank: Rank::Two,
                suit: Suit::Spades,
            } => 12,
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            } => 13,
            Card {
                rank: Rank::King,
                suit: Suit::Hearts,
            } => 14,
            Card {
                rank: Rank::Queen,
                suit: Suit::Hearts,
            } => 15,
            Card {
                rank: Rank::Jack,
                suit: Suit::Hearts,
            } => 16,
            Card {
                rank: Rank::Ten,
                suit: Suit::Hearts,
            } => 17,
            Card {
                rank: Rank::Nine,
                suit: Suit::Hearts,
            } => 18,
            Card {
                rank: Rank::Eight,
                suit: Suit::Hearts,
            } => 19,
            Card {
                rank: Rank::Seven,
                suit: Suit::Hearts,
            } => 20,
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            } => 21,
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            } => 22,
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            } => 23,
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            } => 24,
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts,
            } => 25,
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            } => 26,
            Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            } => 27,
            Card {
                rank: Rank::Queen,
                suit: Suit::Clubs,
            } => 28,
            Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            } => 29,
            Card {
                rank: Rank::Ten,
                suit: Suit::Clubs,
            } => 30,
            Card {
                rank: Rank::Nine,
                suit: Suit::Clubs,
            } => 31,
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            } => 32,
            Card {
                rank: Rank::Seven,
                suit: Suit::Clubs,
            } => 33,
            Card {
                rank: Rank::Six,
                suit: Suit::Clubs,
            } => 34,
            Card {
                rank: Rank::Five,
                suit: Suit::Clubs,
            } => 35,
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            } => 36,
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            } => 37,
            Card {
                rank: Rank::Two,
                suit: Suit::Clubs,
            } => 38,
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            } => 39,
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            } => 40,
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamonds,
            } => 41,
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamonds,
            } => 42,
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds,
            } => 43,
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamonds,
            } => 44,
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamonds,
            } => 45,
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamonds,
            } => 46,
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            } => 47,
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            } => 48,
            Card {
                rank: Rank::Four,
                suit: Suit::Diamonds,
            } => 49,
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds,
            } => 50,
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            } => 51,
        }
    }

    async fn load_card_img(&mut self, card: Card) {}
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
    get_rgba8img(&format!(
        "{}/telluricdeckay.png",
        config_h::get_pixmapsdir()
    ))
    .map_or_else(
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
