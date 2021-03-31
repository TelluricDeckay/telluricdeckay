use std::{
    env, fs,
    io::{self, ErrorKind, Write},
    path::Path,
};
use structopt::StructOpt;
use telluricdeckay::{cli_options, config_h};


pub fn get_datadir_with_package_name() -> String {
  format!("{}/{}", config_h::get_datadir(), env!("CARGO_PKG_NAME"))
}

pub fn get_cardsdir() -> String {
  if Path::new("./assets/cards").exists() {
    return ".".to_owned();
  }
  format!("{}/{}", get_datadir_with_package_name(), "cards" )
}

pub fn get_localedir() -> String {
  format!("{}/{}", config_h::get_datadir(), "locale")
}

pub fn get_pixmapsdir() -> String {
  if Path::new("./assets").exists() {
    return "./assets".to_owned();
  }
  format!("{}/{}", config_h::get_datadir(), "pixmaps")
}

fn get_homedir() -> io::Result<String> {
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

// Write all values stored in c_data to a config file. This can be used
// to create the initial config file, and also, later when the UI supports
// changing options, the existing config file can be overwritten with new
// values.
fn write_config(f: &str, c_data: &Data) -> std::io::Result<()> {
    let mut w = Vec::new();
    writeln!(&mut w, "PlayerNick = {}", c_data.player_nick)?;
    writeln!(&mut w, "Server.{}", c_data.is_server)?;
    writeln!(&mut w, "Server.port = {}", c_data.server_port)?;
    writeln!(&mut w, "max.players = {}", c_data.max_players)?;
    writeln!(&mut w, "max.bet = {}", c_data.max_bet)?;
    writeln!(&mut w, "max.raises = {}", c_data.max_raises)?;

    fs::write(&f, &w)
}

// Get the path and filename to the config file.
// If it wasn't given as a cli argument,
// first check for its existence in the current directory; if not found there,
// check to see if $XDG_CONFIG_HOME is set, if not, use homedir.
fn get_filename(opt_cfg: Option<String>, c_data: &Data) -> String {
    if opt_cfg.is_none() {
        let basename = "telluricdeckayrc".to_owned();
        let cwd = "./".to_owned();
        let default = cwd + &basename;
        if std::path::Path::new(&default).exists() {
            return default;
        }
        let config_home = match env::var("XDG_CONFIG_HOME") {
            Ok(val) => val,
            // TODO: Handle error checking from get_homedir
            Err(_e) => get_homedir().unwrap() + "/.config",
        };
        if !Path::new(&config_home).exists() {
            println!("Creating {}", &config_home);
            fs::create_dir_all(&config_home).expect("Unable to create config directory");
        }

        let config_file = config_home + "/" + &basename;
        if !std::path::Path::new(&config_file).exists() {
            write_config(&config_file, c_data).expect("Error writing config file");
        }

        return config_file;
    }
    opt_cfg.unwrap()
}

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
            // Set the default values for writing an initial configuration file.
            // If a config file already exists, the values in the struct will get
            // overwritten when the file is read.
            player_nick: "New Player".to_owned(),
            is_server: false,
            server_port: 61357,
            max_players: 5,
            max_bet: 50,
            max_raises: 3,
        }
    }
}

pub fn get() -> Data {
    let opt = cli_options::Opt::from_args();

    if opt.version {
        cli_options::get_version();
    }

    let mut config_data = Data::new();
    let config_file = get_filename(opt.custom_config_file, &config_data);
    let config_vec = configster::parse_file(&config_file, ',').expect("Error reading config file");

    // Example config usage
    for i in &config_vec {
        match i.option.as_ref() {
            "PlayerNick" => config_data.player_nick = i.value.primary.clone(),
            // TODO: Handle server.true and server.false
            "Server.port" => {
                config_data.server_port = i.value.primary.parse().expect("Invalid port number")
            }
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
            _ => (), // Not yet handled.
        }
    }
    config_data
}
