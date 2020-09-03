use structopt::StructOpt;
use telluricdeckay::cli_options;

pub fn get_filename(opt_cfg: Option<String>) -> String {
    if opt_cfg.is_none() {
        return "./telluricdeckayrc".to_string();
    }
    opt_cfg.unwrap()
}

// TODO change to enum and add stringification
pub struct Data {
    pub player_nick: String,
    pub is_server: bool,
    pub server_port: u32,
    pub max_players: u32,
    pub max_bet: u32,
    pub max_raises: u32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            player_nick: String::new(),
            is_server: false,
            server_port: 0,
            max_players: 0,
            max_bet: 0,
            max_raises: 0,
        }
    }
}

pub fn get() -> Data {
    let opt = cli_options::Opt::from_args();

    if opt.version {
        cli_options::get_version();
    }

    // This var is also used when cli options are parsed
    let mut testing_interactive: bool = false;
    if opt.interactive {
        testing_interactive = true;
    }

    let mut config_data = Data::new();
    let config_file = get_filename(opt.custom_config_file);
    let config_vec = configster::parse_file(&config_file, ',').expect("Error reading config file");

    // Example config usage
    for i in &config_vec {
        match i.option.as_ref() {
            "PlayerNick" => config_data.player_nick = i.value.primary.clone(),
            "Server.port" => config_data.server_port = i.value.primary.parse().expect("Invalid port number"),
            "testing.interactive" => testing_interactive = true,
            "max.players" => config_data.max_players = i.value.primary.parse().expect("Invalid max players specified"),
            "max.bet" => config_data.max_bet = i.value.primary.parse().expect("Invalid max bet specified"),
            "max.raises" => config_data.max_raises = i.value.primary.parse().expect("Invalid max raises specified"),

            // Needs conversion from str to i32
            // "Server.port" => config_data.server_port = i.value.primary.clone(),
            _ => (), // Not yet handled.
        }
    }
    config_data
}
