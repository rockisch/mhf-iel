#![windows_subsystem = "windows"]
use mhf_iel::{Config, MhfConfig};

use std::{fs::File, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(about)]
pub struct CliConfig {
    #[arg(
        long,
        help = "JSON config file (defaults to 'config.json' in current folder"
    )]
    pub config_file: Option<PathBuf>,
    #[arg(
        long,
        help = "JSON config file (defaults to 'config.json' in current folder"
    )]
    pub config_data: Option<String>,
    #[arg(help = "game folder (defaults to current folder)")]
    pub game_folder: Option<PathBuf>,
}

fn main() {
    let cli_config = CliConfig::parse();
    let config_data = cli_config
        .config_data
        .or_else(|| {
            cli_config
                .config_file
                .or_else(|| std::env::current_dir().map(|d| d.join("config.json")).ok())
                .and_then(|v| File::open(v).ok())
                .and_then(|v| std::io::read_to_string(v).ok())
        })
        .expect("unable to locate 'config.json' file");
    let mhf_config: MhfConfig = serde_json::from_str(&config_data).unwrap();
    let config = Config {
        game_folder: cli_config.game_folder,
        mhf_flags: vec![],
    };
    mhf_iel::run(config, mhf_config).unwrap();
}
