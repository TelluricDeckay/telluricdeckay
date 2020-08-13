use ionic_deckhandler::{Card, Rank, Suit};

struct Action {
    fold: bool,
    check: bool,
    open: bool,
    bet: i32,
    raise: i32,
}

#[derive(Debug)]
pub struct Player {
    name: &'static str,
    hand: [Card; 5],
    chips: i32,
    // actions: Action,
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
