pub mod config_h;

pub mod cli_options {
    //    use std::path::PathBuf;
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    pub struct Opt {
        // Show version
        #[structopt(short = "V", long = "version")]
        pub version: bool,

        // Specify path/filename of alternate configuration file
        #[structopt(short = "c", long = "config")]
        pub custom_config_file: Option<String>,
    }

    pub fn get_version() {
        println!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }
}
