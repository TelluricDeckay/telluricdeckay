use ionic_deckhandler::Card;

struct Actions {
  fold: bool,
  check: bool,
  open: bool,
  bet: i32,
  raise: i32
}

pub struct Player {
  hand: [Card; 5],
  chips: i32,
  actions: Actions
}
