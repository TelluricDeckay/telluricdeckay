mod config;
mod game;
mod player;
use ionic_deckhandler::{Card, Deck};
use std::io;
use structopt::StructOpt;
use telluricdeckay::cli_options;

fn main() -> Result<(), io::Error> {
    // Some of this code for checking options and getting the configuration
    // can likely get moved to separate modules later.
    let mut config_data = config::Data::new();

    let opt = cli_options::Opt::from_args();

    if opt.version {
        cli_options::get_version();
    }

    // This var is also used when cli options are parsed
    let mut testing_interactive: bool = false;
    if opt.interactive {
        testing_interactive = true;
    }

    let config_file = crate::config::get_filename(opt.custom_config_file);
    let config_vec = configster::parse_file(&config_file, ',')?;

    // Example config usage
    for i in &config_vec {
        if i.option == "PlayerNick" {
            config_data.player_nick = i.value.primary.clone();
        }
        if i.option == "testing.interactive" {
            testing_interactive = true;
        }
    }
    println!("Player Nick is {}", config_data.player_nick);

    for _test_games in 0..4 {
        let mut new_game = {
            // The server will keep track of connected players.
            //
            // Connected players will select if they will be joining a pending game

            // FIXME: Players should not start with the same amount of chips for every game.
            let players = vec![
                player::Player::new("Bambi"),
                player::Player::new("Randi"),
                player::Player::new("Candi"),
                player::Player::new("Paul"),
                player::Player::new("John"),
            ];

            // The host will select to start the game
            game::Game {
                players: players,
                pot: 0,
                initial_bet_plus_raises: 0,
                deck: Card::get_deck(),
                card_dealing: game::CardDealing::FiveCardDraw,
                previous_player: None,

                // Only used to init the game. The action is changed the first time a player executes a turn.
                previous_player_action: player::Action::Fold,
                turns_this_round: 0,
                all_bets_paid: false,
            }

            // println!("{:?}", game);
        };

        let number_of_players = new_game.players.len();

        // Ante
        for pl in new_game.players.iter_mut() {
            pl.chips -= 1;
            new_game.pot += 1;
            println!("{} adds 1 for the ante", pl.name);
        }

        new_game.deck.shuffle_deck();
        // Deal the cards
        for i in 0..5 {
            for pl in new_game.players.iter_mut() {
                pl.hand[i] = new_game
                    .deck
                    .take_from_top()
                    .expect("Error: deck is empty!");
            }
        }

        while new_game.all_bets_paid == false {
            let mut input: player::Action;
            for pl in new_game.players.iter_mut() {
                // println!("{:?}-{:?}", &new_game.initial_bet_plus_raises, &pl.total_amount_added_this_round);
                // println!("{:?}", &new_game.turns_this_round);
                // println!("pot - {:?}", &new_game.pot);
                // println!();
                new_game.turns_this_round += 1;

                if pl.has_folded {
                    continue;
                }

                if (pl.total_amount_added_this_round == new_game.initial_bet_plus_raises
                    && new_game.turns_this_round > number_of_players
                    || new_game.turns_this_round == number_of_players
                        && new_game.initial_bet_plus_raises == 0)
                {
                    new_game.all_bets_paid = true;
                    break;
                }
                match new_game.previous_player {
                    None => {
                        input = player::Action::Check;
                        new_game.previous_player = Some(*pl);

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
                                new_game.previous_player_action = input;
                                let input_open = 5;
                                player::open(
                                    input_open,
                                    &mut pl.chips,
                                    &mut pl.total_amount_added_this_round,
                                    &mut new_game.initial_bet_plus_raises,
                                    &mut new_game.pot,
                                );
                                println!("{} opens with {}", pl.name, input_open);
                                new_game.initial_bet_plus_raises = input_open;
                            }
                            player::Action::Fold => pl.has_folded = player::fold(&pl.name),

                            // This player should now be allowed to still "observe" the game
                            // until the end. They don't have to show their cards.
                            //
                            player::Action::Check => {
                                new_game.previous_player_action = input;
                                println!("{} checks.", pl.name);
                            }
                            _ => println!("!Condition mismatch 1"), // The UI should only allow the options above
                        }
                    }
                    _ => {
                        match new_game.initial_bet_plus_raises == 0 {
                            true => {
                                match new_game.turns_this_round {
                                    2 => input = player::Action::Check,
                                    _ => input = player::Action::Open,
                                }
                                match input {
                                    player::Action::Open => {
                                        new_game.previous_player_action = input;
                                        let input_open = 5;
                                        player::open(
                                            input_open,
                                            &mut pl.chips,
                                            &mut pl.total_amount_added_this_round,
                                            &mut new_game.initial_bet_plus_raises,
                                            &mut new_game.pot,
                                        );
                                        println!("{} opens with {}", pl.name, input_open);
                                        new_game.initial_bet_plus_raises = input_open;
                                    }
                                    player::Action::Fold => {
                                        pl.has_folded = player::fold(&pl.name);
                                    }
                                    // This player should now be allowed to still "observe" the game
                                    // until the end. They don't have to show their cards.
                                    //
                                    player::Action::Check => {
                                        new_game.previous_player_action = input;
                                        println!("{} checks.", pl.name);
                                    }
                                    _ => println!("!Condition mismatch 2"), // The UI should only allow the options above
                                }
                            }

                            false => {
                                match new_game.turns_this_round {
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
                                        new_game.previous_player_action = input;
                                        player::call(
                                            &pl.name,
                                            &mut pl.chips,
                                            &mut pl.total_amount_added_this_round,
                                            &new_game.initial_bet_plus_raises,
                                            &mut new_game.pot,
                                        );
                                    }
                                    player::Action::Raise => {
                                        new_game.previous_player_action = input;
                                        let input_raise = 8;
                                        player::raise(
                                            &pl.name,
                                            &input_raise,
                                            &mut pl.chips,
                                            &mut pl.total_amount_added_this_round,
                                            &mut new_game.initial_bet_plus_raises,
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

        // TODO: Don't show the hands for players that folded

        // Showdown
        println!("Total in pot = ${}", new_game.pot);
        for pl in new_game.players.iter_mut() {
            println!(
                "Player {} got a {} and has {} chips remaining",
                pl.name,
                telluric_handeval::poker::evaluate(&mut pl.hand).name(),
                pl.chips
            );
        }
        println!();

        // TODO: Hands need to be evaluated for highest rank, and in the event of a tie
        // compared against each other
        //
    }

    Ok(())
}
