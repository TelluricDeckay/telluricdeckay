use crate::player::{self, Player};
use ionic_deckhandler::{Card, Deck};
use telluric_handeval::poker::{HandRank, PokerRankedHand};

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub number_of_players: usize,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub round: BettingRound,
    pub status: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: Vec::<Player>::new(),
            number_of_players: 0,
            pot: 0,
            deck: Card::get_deck(),
            round: BettingRound::new(),
            status: Vec::new(),
        }
    }

    pub fn player_bet(&mut self, player_id: usize, bet_amount: i32) {
        if bet_amount < self.round.initial_bet_plus_raises {
            if self.round.initial_bet_plus_raises < self.players[player_id].chips {
                self.status
                    .push("Bet is less than initial plus raises.".to_string());
                return;
            }
            else if bet_amount < self.players[player_id].chips {
                self.status
                    .push("You must bet all your chips.".to_string());
                return;
            }
        }
        if self.players[player_id].chips < bet_amount {
            self.status
                .push("Bet exceeds player's remaining chips!".to_string());
        }
        else {
            self.players[player_id].chips -= bet_amount;
            self.pot += bet_amount;
            self.round.initial_bet_plus_raises = bet_amount;
            self.status.push(format!(
                "{} bets {} chips.",
                self.players[player_id].name, bet_amount
            ));
        }
    }
}

pub fn start(new_game: &mut Game) {
    let mut players = vec![
        player::Player::new("Bambi"),
        player::Player::new("Randi"),
        player::Player::new("Candi"),
        player::Player::new("Paul"),
        player::Player::new("John"),
    ];

    // TODO: The server will keep track of connected players.

    // let mut new_game = Game::new();

    // reset some values for players before the next hand is dealt.
    for pl in players.iter_mut() {
        pl.total_amount_added_this_round = 0;
        pl.has_folded = false;
        // TODO: When a player connects, a new Player struct will be created and they'll
        //  be added to the new_game.players Vector
        new_game.players.push(*pl);
    }

    new_game.number_of_players = new_game.players.len();

    new_game.deck.shuffle_deck();

    let input_game_variation = CardDealing::FiveCardDraw;
    // Each of these cases will likely get moved to the Gui file as a "Step"
    match input_game_variation {
        CardDealing::FiveCardDraw => {
            // Deal the cards
            for i in 0..5 {
                for pl in new_game.players.iter_mut() {
                    pl.hand[i] = new_game
                        .deck
                        .take_from_top()
                        .expect("Error: deck is empty!");
                }
            }

            ante(new_game);
            round(new_game);
            // discard/draw
            // another round
            // showdown
        }
        CardDealing::SevenCardStud => {
            ()
            // deal
            // round
            // deal a card face up to each player
            // round
            // deal a card face up to each player
            // round
            // deal a card face up to each player
            // round
            // deal a card face DOWN to each player
            // round
            // showdown
        }
        _ => (),
    }

    // TODO: Don't show the hands for players that folded

    // Showdown
    new_game
        .status
        .push(format!("Total in pot = ${}", new_game.pot));
    for pl in new_game.players.iter_mut() {
        new_game.status.push(format!(
            "Player {} got a {:?} and has {} chips remaining",
            pl.name,
            pl.hand.evaluate_hand(),
            pl.chips
        ));
    }
    println!();
}

#[derive(Debug)]
pub enum CardDealing {
    FiveCardDraw,
    // FiveCardDoubleDraw,
    SevenCardStud,
    // FiveCardStud,
}

#[derive(Debug)]
pub struct BettingRound {
    pub initial_bet_plus_raises: i32,
    pub turns: usize,
    pub all_bets_paid: bool,
}

impl BettingRound {
    pub fn new() -> Self {
        Self {
            initial_bet_plus_raises: 0,
            turns: 0,
            all_bets_paid: false,
        }
    }
}

pub fn ante(new_game: &mut Game) {
    for pl in new_game.players.iter_mut() {
        pl.chips -= 1;
        new_game.pot += 1;
        new_game
            .status
            .push(format!("{} adds 1 for the ante", pl.name));
    }
}

pub fn round(new_game: &mut Game) {
    while new_game.round.all_bets_paid == false {
        let mut input: player::Action;
        for pl in new_game.players.iter_mut() {
            // println!("{:?}-{:?}", &new_game.initial_bet_plus_raises, &pl.total_amount_added_this_round);
            // println!("{:?}", &new_game.roundturns_this_round);
            // println!("pot - {:?}", &new_game.pot);
            // println!();

            if (pl.total_amount_added_this_round == new_game.round.initial_bet_plus_raises
                && new_game.round.turns > new_game.number_of_players)
                || (new_game.round.turns == new_game.number_of_players
                    && new_game.round.initial_bet_plus_raises == 0)
            {
                new_game.round.all_bets_paid = true;

                // reset some values before the start of the next betting round
                for pl in new_game.players.iter_mut() {
                    pl.total_amount_added_this_round = 0;
                }

                break;
            }

            new_game.round.turns += 1;

            if pl.has_folded {
                continue;
            }
            match new_game.round.initial_bet_plus_raises == 0 {
                true => {
                    match new_game.round.turns {
                        1 => input = player::Action::Check,
                        2 => input = player::Action::Check,
                        3 => input = player::Action::Open,
                        4 => input = player::Action::Fold,
                        _ => input = player::Action::Fold,
                    }
                    match input {
                        player::Action::Open => {
                            let input_open = 5;
                            player::open(
                                input_open,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &mut new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            );
                            new_game
                                .status
                                .push(format!("{} opens with {}", pl.name, input_open));
                            new_game.round.initial_bet_plus_raises = input_open;
                        }
                        player::Action::Fold => {
                            new_game.status.push(player::fold(&pl.name, &mut pl.has_folded));
                        }
                        player::Action::Check => new_game.status.push(player::check(&pl.name)),
                        _ => println!("!Condition mismatch 1"),
                         // The UI should only allow the options above
                    }
                }

                false => {
                    match new_game.round.turns {
                        4 => input = player::Action::Raise,
                        5 => input = player::Action::Call,
                        6 => input = player::Action::Raise,
                        7 => input = player::Action::Fold,
                        8 => input = player::Action::Raise,
                        9 => input = player::Action::Raise,
                        10 => input = player::Action::Fold,
                        _ => input = player::Action::Call,
                    }
                    match input {
                        player::Action::Call => {
                            new_game.status.push(player::call(
                                &pl.name,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            ));
                        }
                        player::Action::Raise => {
                            let input_raise = 8;
                            new_game.status.push(player::raise(
                                &pl.name,
                                &input_raise,
                                &mut pl.chips,
                                &mut pl.total_amount_added_this_round,
                                &mut new_game.round.initial_bet_plus_raises,
                                &mut new_game.pot,
                            ));
                        }
                        player::Action::Fold => new_game.status.push(player::fold(&pl.name, &mut pl.has_folded)),
                        _ => println!("!Condition mismatch 2"),
                    }
                }
            }
        }
    }
}
