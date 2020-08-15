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
pub fn open(input_bet: i32, chips: &mut i32, pot: &mut i32) {
    *chips -= input_bet;
    *pot += input_bet;
}

pub fn call(name: &str, chips: &mut i32, initial_bet_plus_raises: &i32, pot: &mut i32) {
    let t = initial_bet_plus_raises;
    *chips -= t;
    *pot += t;
    println!("{} calls bet of ${}", name, initial_bet_plus_raises);
}

pub fn raise(input_raise: &i32, chips: &mut i32, pot: &mut i32) {
    let t = input_raise;
    *chips -= t;
    *pot += t;
    println!("and raises ${}", input_raise);
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name: &'static str,
    pub hand: [Card; 5],
    pub chips: i32,
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
        }
    }
}
