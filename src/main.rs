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
                deck: Card::get_deck(),
                card_dealing: game::CardDealing::FiveCardDraw,
                last_player: None,
            }

            // println!("{:?}", game);
        };

        // Ante
        for pl in new_game.players.iter_mut() {
            pl.chips -= 1;
            new_game.pot += 1;
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
        // last_player_turn = Some(player);
        while all_bets_paid == false {
            for pl in new_game.players.iter_mut() {
                match new_game.last_player {
                    None => {
                        // some of this repetition will be moved into a function or maybe
                        // an implementation
                        pl.action.bet = 5;
                        pl.action.open = true;
                        pl.chips -= pl.action.bet;
                        new_game.pot += pl.action.bet;
                        println!("{} opens with {}", pl.name, pl.action.bet);
                        new_game.last_player = Some(*pl);
                    }
                    _ => {
                        if new_game.last_player.unwrap().action.open {
                            pl.action.call = true;
                            pl.action.bet = new_game.last_player.unwrap().action.bet;
                            pl.chips -= pl.action.bet;
                            println!("{} calls bet of {}", pl.name, pl.action.bet);
                            new_game.pot += new_game.last_player.unwrap().action.bet;
                            new_game.last_player = Some(*pl);
                        }
                    }
                }
            }

            // Player 1 opens
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
