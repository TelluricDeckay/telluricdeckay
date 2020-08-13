mod config;
mod game;
mod player;
use std::io;
use ionic_deckhandler::{Card, Deck, Rank, Suit};

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

    // The server will keep track of connected players.
    //
    // Connected players will select if they will be joining a pending game
    let players = vec![player::Player::new("P1"), player::Player::new("P2")] ;

    // The host will select to start the game
    let new_game = game::Game::FiveCardDraw { players: players, pot: 0, deck: Card::get_deck() };

    // println!("{:?}", game);

    // Deal the cards
    let mut cards_dealt = 0;
    for i in 0..5 {
        for j in &players {
            let receive = &new_game { players, pot, deck[cards_dealt] };
            new_game { players: player[j].hand[i] } = receive;
            cards_dealt += 1;
        }
    }


    Ok(())
}
