use crate::player::{self, Player};
// use crate::player;
use ionic_deckhandler::Card;

#[derive(Debug)]
pub struct Game<'a> {
    pub players: &'a mut Vec<Player>,
    pub number_of_players: usize,
    pub pot: i32,
    pub deck: Vec<Card>,
    pub round: BettingRound,
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

pub fn ante(new_game: &mut Game) {
    for pl in new_game.players.iter_mut() {
        pl.chips -= 1;
        new_game.pot += 1;
        println!("{} adds 1 for the ante", pl.name);
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
            match new_game.round.previous_player {
                None => {
                    input = player::Action::Check;
                    new_game.round.previous_player = Some(*pl);

                    // The proper conversions will need to be done for this to work
                    //
                    /* if testing_interactive {
                        println!("Player {} action:", pl.name);

                        io::stdin().read_line(&mut input)
                            .expect("Failed to read line");

                        let input: player::Action = match input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };
                    } else {
                        input = player::Action::Open;
                    } */

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
                            println!("{} opens with {}", pl.name, input_open);
                            new_game.round.initial_bet_plus_raises = input_open;
                        }

                        // This player should now be allowed to still "observe" the game
                        // until the end. They don't have to show their cards.
                        player::Action::Fold => pl.has_folded = player::fold(&pl.name),

                        player::Action::Check => player::check(&pl.name),
                        _ => println!("!Condition mismatch 1"), // The UI should only allow the options above
                    }
                }
                _ => {
                    match new_game.round.initial_bet_plus_raises == 0 {
                        true => {
                            match new_game.round.turns {
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
                                    println!("{} opens with {}", pl.name, input_open);
                                    new_game.round.initial_bet_plus_raises = input_open;
                                }
                                player::Action::Fold => {
                                    pl.has_folded = player::fold(&pl.name);
                                }
                                player::Action::Check => player::check(&pl.name),

                                _ => println!("!Condition mismatch 2"), // The UI should only allow the options above
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
                                    player::call(
                                        &pl.name,
                                        &mut pl.chips,
                                        &mut pl.total_amount_added_this_round,
                                        &new_game.round.initial_bet_plus_raises,
                                        &mut new_game.pot,
                                    );
                                }
                                player::Action::Raise => {
                                    let input_raise = 8;
                                    player::raise(
                                        &pl.name,
                                        &input_raise,
                                        &mut pl.chips,
                                        &mut pl.total_amount_added_this_round,
                                        &mut new_game.round.initial_bet_plus_raises,
                                        &mut new_game.pot,
                                    );
                                }
                                player::Action::Fold => pl.has_folded = player::fold(&pl.name),
                                _ => println!("!Condition mismatch 3"),
                            }
                        }
                    }
                }
            }
        }
    }
}
