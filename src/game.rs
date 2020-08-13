use crate::player::Player;
use ionic_deckhandler::{Card, Deck, Rank, Suit};

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub card_dealing: CardDealing,
}

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    FiveCardDoubleDraw,
    SevenCardStud,
    FiveCardStud,
}
