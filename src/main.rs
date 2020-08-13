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

    let mut new_game = {
        // The server will keep track of connected players.
        //
        // Connected players will select if they will be joining a pending game
        let players = vec![player::Player::new("P1"), player::Player::new("P2")] ;

        // The host will select to start the game
        game::Game { players: players, pot: 0, deck: Card::get_deck(), card_dealing: game::CardDealing::FiveCardDraw }

        // println!("{:?}", game);
    };

    // Deal the cards
    let mut cards_dealt = 0;
    for i in 0..5 {
        for pl in new_game.players.iter_mut() {
            // let receive = &new_game;// { players, pot, deck[cards_dealt] };
            pl.hand[i] = new_game.deck.take_from_top().expect("Error: deck is empty!");
            cards_dealt += 1;
        }
    }


    Ok(())
}
