use crate::player::Player;
use ionic_deckhandler::Card;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub card_dealing: CardDealing,
    pub last_player: Option<Player>,
}

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    // FiveCardDoubleDraw,
    // SevenCardStud,
    // FiveCardStud,
}
