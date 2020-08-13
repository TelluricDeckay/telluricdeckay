use crate::player::Player;
use ionic_deckhandler::{Card, Deck, Rank, Suit};

#[derive(Debug)]
pub enum Game {
    FiveCardDraw {
        players: Vec<Player>,
        pot: i32,
        deck: Vec<Card>,
    },
    FiveCardDoubleDraw,
}
