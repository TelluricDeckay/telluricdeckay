use crate::player::{Action, Player};
use ionic_deckhandler::Card;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub pot: i32,
    pub initial_bet_plus_raises: i32,
    pub deck: Vec<Card>,
    pub card_dealing: CardDealing,
    pub previous_player: Option<Player>,
    pub previous_player_action: Action,
    pub turns_this_round: usize,
    pub all_bets_paid: bool,
}

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    // FiveCardDoubleDraw,
    // SevenCardStud,
    // FiveCardStud,
}
