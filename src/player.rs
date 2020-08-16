// use crate::game::Game;
use ionic_deckhandler::{Card, Rank, Suit};

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Fold,
    Check,
    Open,
    Call,
    Raise,
}

/* impl Action {
    pub fn new() -> Self {
        Self {
            fold: false,
            check: false,
            open: false,
            bet: 0,
            call: false,
            raise: 0,
        }
    }
} */

// TODO: Add check to make sure player has enough chips, handle case
pub fn open(
    input_bet: i32,
    chips: &mut i32,
    pl_total_amount_added_this_round: &mut i32,
    initial_bet_plus_raises: &mut i32,
    pot: &mut i32,
) {
    *chips -= input_bet;
    *pl_total_amount_added_this_round += input_bet;
    *pot += input_bet;
    *initial_bet_plus_raises += input_bet;
}

pub fn call(
    name: &str,
    chips: &mut i32,
    pl_total_amount_added_this_round: &mut i32,
    initial_bet_plus_raises: &i32,
    pot: &mut i32,
) {
    let t = initial_bet_plus_raises - *pl_total_amount_added_this_round;
    *chips -= t;
    *pl_total_amount_added_this_round += t;
    *pot += t;
    println!("{} calls bet of ${}", name, t);
}

pub fn raise(
    name: &str,
    input_raise: &i32,
    chips: &mut i32,
    pl_total_amount_added_this_round: &mut i32,
    initial_bet_plus_raises: &mut i32,
    pot: &mut i32,
) {
    call(
        name,
        chips,
        pl_total_amount_added_this_round,
        initial_bet_plus_raises,
        pot,
    );

    let t = input_raise;
    *chips -= t;
    *pl_total_amount_added_this_round += t;
    *pot += t;
    println!("and raises ${}", input_raise);
    *initial_bet_plus_raises += input_raise;
}

// Players that fold shouldn't be popped from the 'players' vector;
// the server and client needs to know about them throughout the
// game, and at the end, still display that they folded.
pub fn fold(name: &str) -> bool {
    println!("{} folds.", name);
    true
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name: &'static str,
    pub hand: [Card; 5],
    pub chips: i32,
    pub total_amount_added_this_round: i32,
    pub has_folded: bool,
}

impl Player {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name,
            hand: [
                // Is there a better way to intialize this array?
                Card::new(Rank::Five, Suit::Clubs),
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Eight, Suit::Clubs),
                Card::new(Rank::Nine, Suit::Clubs),
            ],
            chips: 100,
            total_amount_added_this_round: 0,
            has_folded: false,
        }
    }
}
