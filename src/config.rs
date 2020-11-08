use std::{
    env, fs,
    io::{self, ErrorKind, Write},
    path::Path,
};
use structopt::StructOpt;
use telluricdeckay::{cli_options, config_h};

pub fn get_homedir() -> io::Result<String> {
    let homedir: String = match dirs::home_dir() {
        Some(homedir) => homedir.to_str().unwrap().into(),
        None => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "Unable to determine homedir",
            ))
        }
    };
    Ok(homedir)
}

fn write_config(f: &str) -> std::io::Result<()> {
    let mut w = Vec::new();
    writeln!(&mut w, "PlayerNick = New Player")?;
    writeln!(&mut w, "Server.true")?;
    writeln!(&mut w, "Server.port = 61357")?;
    writeln!(&mut w, "max.players = 5")?;
    writeln!(&mut w, "max.bet = 50")?;
    writeln!(&mut w, "max.raises = 3")?;

    fs::write(&f, &w)
}

// Get the path and filename to the config file.
// If it wasn't given as a cli argument,
// first check for its existence in the current directory; if not found there,
// check to see if $XDG_CONFIG_HOME is set, if not, use homedir.
pub fn get_filename(opt_cfg: Option<String>) -> String {
    if opt_cfg.is_none() {
        let basename = "telluricdeckayrc".to_owned();
        let cwd = "./".to_owned();
        let default = cwd + &basename;
        if std::path::Path::new(&default).exists() {
            return default;
        }
        let config_home = match env::var("XDG_CONFIG_HOME") {
            Ok(val) => val,
            Err(_e) => get_homedir().unwrap() + "/.config",
        };
        if !Path::new(&config_home).exists() {
            println!("Creating {}", &config_home);
            fs::create_dir_all(&config_home).expect("Unable to create config directory");
        }

        let config_file = config_home + "/" + &basename;
        if !std::path::Path::new(&config_file).exists() {
            write_config(&config_file).expect("Error writing config file");
        }

        return config_file;
    }
    opt_cfg.unwrap()
}

// I think the comment below is outdated. Is there any good reason that
// should be changed to an enum? If not, please remove the comment.

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
            "Server.port" => {
                config_data.server_port = i.value.primary.parse().expect("Invalid port number")
            }
            "testing.interactive" => testing_interactive = true,
            "max.players" => {
                config_data.max_players = i
                    .value
                    .primary
                    .parse()
                    .expect("Invalid max players specified")
            }
            "max.bet" => {
                config_data.max_bet = i.value.primary.parse().expect("Invalid max bet specified")
            }
            "max.raises" => {
                config_data.max_raises = i
                    .value
                    .primary
                    .parse()
                    .expect("Invalid max raises specified")
            }

            // Needs conversion from str to i32
            // "Server.port" => config_data.server_port = i.value.primary.clone(),
            _ => (), // Not yet handled.
        }
    }
    config_data
}
