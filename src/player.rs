use ionic_deckhandler::{Card, Rank, Suit};

#[derive(Debug, Copy, Clone)]
pub struct Action {
    pub fold: bool,
    pub check: bool,
    pub open: bool,
    pub bet: i32,
    pub call: bool,
    pub raise: i32,
}

impl Action {
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
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name: &'static str,
    pub hand: [Card; 5],
    pub chips: i32,
    pub action: Action,
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
            action: Action::new(),
        }
    }
}
