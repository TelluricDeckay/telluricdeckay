mod config;
mod player;
mod round;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut config_data = config::Data::new();
    let config_vec = configster::parse_file("telluricdeckayrc", ',')?;

    // let mut player_nick: &str;
    for i in &config_vec {
        if i.option == "PlayerNick" {
            config_data.player_nick = i.value.primary.clone();
        }
    }

    println!("Player Nick is {}", config_data.player_nick);

    Ok(())
}
