mod config;
mod game;
mod player;
use ionic_deckhandler::{Card, Deck};
use std::io;

fn main() -> Result<(), io::Error> {
    let mut config_data = config::Data::new();
    let config_vec = configster::parse_file("telluricdeckayrc", ',')?;

    // Example config usage
    for i in &config_vec {
        if i.option == "PlayerNick" {
            config_data.player_nick = i.value.primary.clone();
        }
    }
    println!("Player Nick is {}", config_data.player_nick);

    for _test_games in 0..9 {
        let mut new_game = {
            // The server will keep track of connected players.
            //
            // Connected players will select if they will be joining a pending game
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
            }

            // println!("{:?}", game);
        };

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

        let mut all_bets_paid: bool = false;

        while all_bets_paid == false {
            let mut input;
            for pl in new_game.players.iter_mut() {
                match new_game.previous_player {
                    None => {
                        input = player::Action::Open;
                        new_game.previous_player = Some(*pl);

                        match input {
                            player::Action::Open => {
                                new_game.previous_player_action = input;
                                let input_open = 5;
                                player::open(input_open, &mut pl.chips, &mut new_game.pot);
                                println!("{} opens with {}", pl.name, input_open);
                                new_game.initial_bet_plus_raises = input_open;
                            }
                            player::Action::Fold => new_game.previous_player_action = input,
                            // This player should now be allowed to still "observe" the game
                            // until the end. They don't have to show their cards.
                            //
                            player::Action::Check => new_game.previous_player_action = input,
                            _ => (), // The UI should only allow the options above
                        }
                    }
                    _ => {
                        input = player::Action::Call;
                        new_game.previous_player = Some(*pl);

                        match new_game.previous_player_action {
                            player::Action::Open => {
                                new_game.previous_player_action = input;
                                player::call(&mut pl.chips, &new_game.initial_bet_plus_raises, &mut new_game.pot);
                                println!(
                                    "{} calls bet of ${}",
                                    pl.name, new_game.initial_bet_plus_raises
                                );
                            }
                            player::Action::Raise => {
                                new_game.previous_player_action = input;
                                let input_raise = 8;
                                player::raise(
                                    &input_raise,
                                    &new_game.initial_bet_plus_raises,
                                    &mut pl.chips,
                                    &mut new_game.pot,
                                );
                                println!(
                                    "{} calls bet of ${} and raises ${}",
                                    pl.name, new_game.initial_bet_plus_raises, input_raise
                                );
                            }
                            player::Action::Check => new_game.previous_player_action = input,
                            player::Action::Fold => new_game.previous_player_action = input,
                            _ => (),
                        }
                    }
                }
            }

            all_bets_paid = true;
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
