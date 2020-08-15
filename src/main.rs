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

    for _test_games in 0..9 {
        let mut new_game = {
            // The server will keep track of connected players.
            //
            // Connected players will select if they will be joining a pending game

            // FIXME: Players should not start with the same amount of chips for every game.
            let players = vec![player::Player::new("P1"), player::Player::new("P2")];

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

                if (pl.total_amount_added_this_round == new_game.initial_bet_plus_raises)
                    && (new_game.turns_this_round > number_of_players)
                {
                    new_game.all_bets_paid = true;
                    break;
                }
                match new_game.previous_player {
                    None => {
                        input = player::Action::Open;
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
                            player::Action::Fold => new_game.previous_player_action = input,
                            // This player should now be allowed to still "observe" the game
                            // until the end. They don't have to show their cards.
                            //
                            player::Action::Check => new_game.previous_player_action = input,
                            _ => println!("Condition mismatch 1"), // The UI should only allow the options above
                        }
                    }
                    _ => {
                        if new_game.turns_this_round >= 4 {
                            input = player::Action::Call;
                        } else {
                            input = player::Action::Raise;
                        }

                        new_game.previous_player = Some(*pl);
                        match new_game.previous_player_action {
                            player::Action::Open => {
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
                                        // In a "real" game, a player must call first then raise. So we'll
                                        // use both the call and raise functions here.
                                        player::call(
                                            &pl.name,
                                            &mut pl.chips,
                                            &mut pl.total_amount_added_this_round,
                                            &new_game.initial_bet_plus_raises,
                                            &mut new_game.pot,
                                        );
                                        player::raise(
                                            &input_raise,
                                            &mut pl.chips,
                                            &mut pl.total_amount_added_this_round,
                                            &mut new_game.initial_bet_plus_raises,
                                            &mut new_game.pot,
                                        );
                                    }
                                    player::Action::Check => {
                                        new_game.previous_player_action = input
                                    }
                                    player::Action::Fold => new_game.previous_player_action = input,
                                    _ => println!("Condition mismatch 2"),
                                }
                            }
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
                                // In a "real" game, a player must call first then raise. So we'll
                                // use both the call and raise functions here.
                                player::call(
                                    &pl.name,
                                    &mut pl.chips,
                                    &mut pl.total_amount_added_this_round,
                                    &new_game.initial_bet_plus_raises,
                                    &mut new_game.pot,
                                );
                                player::raise(
                                    &input_raise,
                                    &mut pl.chips,
                                    &mut pl.total_amount_added_this_round,
                                    &mut new_game.initial_bet_plus_raises,
                                    &mut new_game.pot,
                                );
                            }
                            player::Action::Check => new_game.previous_player_action = input,
                            player::Action::Fold => new_game.previous_player_action = input,
                            _ => println!("Condition mismatch 3"), // The UI should only allow the options above.
                        }
                    }
                }
            }
        }

        // Showdown

        if telluric_handeval::poker::evaluate(&mut new_game.players[0].hand)
            > telluric_handeval::poker::evaluate(&mut new_game.players[1].hand)
        {
            new_game.players[0].chips += new_game.pot;
            println!("{} wins!", new_game.players[0].name);
        } else if telluric_handeval::poker::evaluate(&mut new_game.players[0].hand)
            < telluric_handeval::poker::evaluate(&mut new_game.players[1].hand)
        {
            new_game.players[1].chips += new_game.pot;
            println!("{} wins!", new_game.players[1].name);
        } else {
            println!("A tie!");
        }

        for pl in new_game.players.iter_mut() {
            println!("Total in pot = ${}", new_game.pot);
            println!(
                "Player {} got a {} and had {} chips remaining",
                pl.name,
                telluric_handeval::poker::evaluate(&mut pl.hand).name(),
                pl.chips
            );
        }
        println!();
    }

    Ok(())
}
