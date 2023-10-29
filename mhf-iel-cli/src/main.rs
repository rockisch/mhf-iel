#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mhf_iel::MhfConfig;

use std::{fs::File, path::PathBuf, process::exit};

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(about = Some("Runs MHF. Config data can be specified through arguments, and defaults to a 'config.json' file in the current folder."))]
pub struct CliConfig {
    #[arg(long, help = "JSON config file")]
    pub config_file: Option<PathBuf>,
    #[arg(long, help = "JSON config data")]
    pub config_data: Option<String>,
}

fn main() {
    let cli_config = CliConfig::try_parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });
    let config_data = cli_config
        .config_data
        .or_else(|| {
            cli_config
                .config_file
                .or_else(|| std::env::current_dir().map(|d| d.join("config.json")).ok())
                .and_then(|v| File::open(v).ok())
                .and_then(|v| std::io::read_to_string(v).ok())
        })
        .unwrap_or_else(|| {
            eprintln!("unable to locate 'config.json' file");
            exit(2);
        });
    let mhf_config: MhfConfig = serde_json::from_str(&config_data).unwrap_or_else(|e| {
        eprintln!("error parsing config data: {}", e);
        exit(3);
    });
    let result = mhf_iel::run(mhf_config);
    if let Err(e) = result {
        eprintln!("error running mhf: {}", e);
        exit(4);
    }
    exit(0);
}
