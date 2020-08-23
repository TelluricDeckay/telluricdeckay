mod config;
mod game;
mod player;
use ionic_deckhandler::{Card, Deck};
use std::io;
use structopt::StructOpt;
use telluricdeckay::cli_options;
use iced::Sandbox;
mod gui;

fn main() -> Result<(), io::Error> {

    gui::Gui::run(iced::Settings::default());
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
        match i.option.as_ref() {
            "PlayerNick" => config_data.player_nick = i.value.primary.clone(),
            "testing.interactive" => testing_interactive = true,

            // Needs conversion from str to i32
            // "Server.port" => config_data.server_port = i.value.primary.clone(),
            _ => (), // Not yet handled.
        }
    }

    println!("Player Nick is {}", config_data.player_nick);

    let mut players = vec![
        player::Player::new("Bambi"),
        player::Player::new("Randi"),
        player::Player::new("Candi"),
        player::Player::new("Paul"),
        player::Player::new("John"),
    ];

    let n = players.len();

    for _i in 0..4 {
        let mut new_game = {
            // The server will keep track of connected players.
            //
            // Connected players will select if they will be joining a pending game

            // reset some values for players before the next hand is dealt.
            for pl in players.iter_mut() {
                pl.total_amount_added_this_round = 0;
                pl.has_folded = false;
            }

            // The host will select to start the game
            game::Game {
                players: &mut players,
                number_of_players: n,
                pot: 0,
                deck: Card::get_deck(),
                round: game::BettingRound::new(),
            }

            // println!("{:?}", game);
        };

        new_game.deck.shuffle_deck();

        let input_game_variation = game::CardDealing::FiveCardDraw;
        match input_game_variation {
            game::CardDealing::FiveCardDraw => {
                // Deal the cards
                for i in 0..5 {
                    for pl in new_game.players.iter_mut() {
                        pl.hand[i] = new_game
                            .deck
                            .take_from_top()
                            .expect("Error: deck is empty!");
                    }
                }

                game::ante(&mut new_game);
                game::round(&mut new_game);
                // discard/draw
                // another round
                // showdown
            }
            game::CardDealing::SevenCardStud => {
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
        println!("Total in pot = ${}", new_game.pot);
        for pl in new_game.players.iter_mut() {
            println!(
                "Player {} got a {} and has {} chips remaining",
                pl.name,
                telluric_handeval::poker::evaluate(&mut pl.hand).0.name(),
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
