use crate::player::Player;
use ionic_deckhandler::Card;

#[derive(Debug)]
pub struct Game<'a> {
    pub players: &'a mut Vec<Player>,
    pub number_of_players: usize,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub card_dealing: CardDealing,
    pub round: BettingRound,
}

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    // FiveCardDoubleDraw,
    // SevenCardStud,
    // FiveCardStud,
}

#[derive(Debug)]
pub struct BettingRound {
    pub previous_player: Option<Player>,
    pub initial_bet_plus_raises: i32,
    pub turns: usize,
    pub all_bets_paid: bool,
}

impl BettingRound {
    pub fn new() -> Self {
        Self {
            previous_player: None,
            initial_bet_plus_raises: 0,
            turns: 0,
            all_bets_paid: false,
        }
    }
}
