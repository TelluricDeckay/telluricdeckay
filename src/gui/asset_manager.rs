use crate::config;
use iced::{window::Icon};
use image::{io::Reader as ImageReader, ImageResult, RgbaImage};
use ionic_deckhandler::{Card, Rank, Suit};

pub(in crate::gui) const CARD_FILES: [&str; 52] = [
    "AS.png", "KS.png", "QS.png", "JS.png", "TS.png", "9S.png", "8S.png", "7S.png", "6S.png",
    "5S.png", "4S.png", "3S.png", "2S.png", "AH.png", "KH.png", "QH.png", "JH.png", "TH.png",
    "9H.png", "8H.png", "7H.png", "6H.png", "5H.png", "4H.png", "3H.png", "2H.png", "AC.png",
    "KC.png", "QC.png", "JC.png", "TC.png", "9C.png", "8C.png", "7C.png", "6C.png", "5C.png",
    "4C.png", "3C.png", "2C.png", "AD.png", "KD.png", "QD.png", "JD.png", "TD.png", "9D.png",
    "8D.png", "7D.png", "6D.png", "5D.png", "4D.png", "3D.png", "2D.png",
];

pub(in crate::gui) async fn async_card_img(card_name: &str) -> Vec<u8> {
    async_read_bytes(&format!(
        "{}/{}",
        config::get_cardsdir(),
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

pub(in crate::gui) enum CardImgOrText {
    Img(Vec<u8>),
    Text(String),
}

pub(in crate::gui) struct AssetCacher {
    card_svgs: [CardImgOrText; 52],
}

impl AssetCacher {
    pub(in crate::gui) fn new() -> AssetCacher {
        AssetCacher {
            card_svgs: [
                CardImgOrText::Text("Ace of Spades".to_owned()),
                CardImgOrText::Text("king of Spades".to_owned()),
                CardImgOrText::Text("Queen of Spades".to_owned()),
                CardImgOrText::Text("Jack of Spades".to_owned()),
                CardImgOrText::Text("Ten of Spades".to_owned()),
                CardImgOrText::Text("Nine of Spades".to_owned()),
                CardImgOrText::Text("Eight of Spades".to_owned()),
                CardImgOrText::Text("Seven of Spades".to_owned()),
                CardImgOrText::Text("Six of Spades".to_owned()),
                CardImgOrText::Text("Five of Spades".to_owned()),
                CardImgOrText::Text("Four of Spades".to_owned()),
                CardImgOrText::Text("Three of Spades".to_owned()),
                CardImgOrText::Text("Two of Spades".to_owned()),
                CardImgOrText::Text("Ace of Hearts".to_owned()),
                CardImgOrText::Text("king of Hearts".to_owned()),
                CardImgOrText::Text("Queen of Hearts".to_owned()),
                CardImgOrText::Text("Jack of Hearts".to_owned()),
                CardImgOrText::Text("Ten of Hearts".to_owned()),
                CardImgOrText::Text("Nine of Hearts".to_owned()),
                CardImgOrText::Text("Eight of Hearts".to_owned()),
                CardImgOrText::Text("Seven of Hearts".to_owned()),
                CardImgOrText::Text("Six of Hearts".to_owned()),
                CardImgOrText::Text("Five of Hearts".to_owned()),
                CardImgOrText::Text("Four of Hearts".to_owned()),
                CardImgOrText::Text("Three of Hearts".to_owned()),
                CardImgOrText::Text("Two of Hearts".to_owned()),
                CardImgOrText::Text("Ace of Clubs".to_owned()),
                CardImgOrText::Text("king of Clubs".to_owned()),
                CardImgOrText::Text("Queen of Clubs".to_owned()),
                CardImgOrText::Text("Jack of Clubs".to_owned()),
                CardImgOrText::Text("Ten of Clubs".to_owned()),
                CardImgOrText::Text("Nine of Clubs".to_owned()),
                CardImgOrText::Text("Eight of Clubs".to_owned()),
                CardImgOrText::Text("Seven of Clubs".to_owned()),
                CardImgOrText::Text("Six of Clubs".to_owned()),
                CardImgOrText::Text("Five of Clubs".to_owned()),
                CardImgOrText::Text("Four of Clubs".to_owned()),
                CardImgOrText::Text("Three of Clubs".to_owned()),
                CardImgOrText::Text("Two of Clubs".to_owned()),
                CardImgOrText::Text("Ace of Diamonds".to_owned()),
                CardImgOrText::Text("king of Diamonds".to_owned()),
                CardImgOrText::Text("Queen of Diamonds".to_owned()),
                CardImgOrText::Text("Jack of Diamonds".to_owned()),
                CardImgOrText::Text("Ten of Diamonds".to_owned()),
                CardImgOrText::Text("Nine of Diamonds".to_owned()),
                CardImgOrText::Text("Eight of Diamonds".to_owned()),
                CardImgOrText::Text("Seven of Diamonds".to_owned()),
                CardImgOrText::Text("Six of Diamonds".to_owned()),
                CardImgOrText::Text("Five of Diamonds".to_owned()),
                CardImgOrText::Text("Four of Diamonds".to_owned()),
                CardImgOrText::Text("Three of Diamonds".to_owned()),
                CardImgOrText::Text("Two of Diamonds".to_owned()),
            ],
        }
    }

    pub(in crate::gui) fn set_card_asset(&mut self, idx: usize, img: Vec<u8>) {
        self.card_svgs[idx] = CardImgOrText::Img(img);
    }

    pub(in crate::gui) fn get_card_asset(&self, card: &Card) -> &CardImgOrText {
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
}

fn get_rgba8img(filename: &str) -> ImageResult<RgbaImage> {
    Ok(ImageReader::open(filename)?.decode()?.to_rgba8())
}

pub fn get_icon() -> Option<Icon> {
    get_rgba8img(&format!(
        "{}/{}.png",
        config::get_pixmapsdir(),
        env!("CARGO_PKG_NAME")
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

#[test]
fn test_get_icon() {
    assert_eq!(get_icon().is_some(), true);
}
