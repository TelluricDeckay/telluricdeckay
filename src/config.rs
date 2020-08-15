pub fn get_filename(opt_cfg: Option<String>) -> String {
    if opt_cfg.is_none() {
        return "./telluricdeckayrc".to_string();
    }
    opt_cfg.unwrap()
}

pub struct Data {
    pub player_nick: String,
    pub is_server: bool,
    pub server_port: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            player_nick: String::new(),
            is_server: false,
            server_port: 0,
        }
    }
}
