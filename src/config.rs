use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use text_io::read;

#[derive(StructOpt, Debug)]
#[structopt(name = "loggy", about = "Small utility to extract data from DB and save it to file")]
pub struct CliOptions {
    #[structopt(name = "TEST_ID", help = "Test id to use to extract logs")]
    test_id: Option<usize>,
}

#[derive(Debug)]
pub struct LoggyConfig {
    config_file: LocalConfig,
    cli_opts: CliOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    url: String,
    sql: String,
}

pub fn get_config() -> LoggyConfig {
    let args = CliOptions::from_args();

    LoggyConfig {
        config_file: get_local_loggy_config(),
        cli_opts: args,
    }
}

fn get_local_loggy_config() -> LocalConfig {
    if let Some(home) = home::home_dir() {
        let loggy_config_dir = home.join(".loggy").into_boxed_path();
        if !loggy_config_dir.exists() {
            println!("No config dir exist at {}, creating", loggy_config_dir.to_str().unwrap_or(""));
            fs::create_dir_all(loggy_config_dir.clone()).expect("Unable to create loggy config directory");
        }
        let loggy_config_file = loggy_config_dir.join("config.json");
        return if !loggy_config_file.exists() {
            println!("No config file found in {}, initializing new configuration", loggy_config_file.to_str().unwrap_or("user home directory"));
            println!("Provide full URL to connect to DB");
            let url: String = read!();
            let new_config = LocalConfig {
                url,
                sql: "SELECT * FROM VIEW".to_string(),
            };

            let mut config_file = File::create(loggy_config_file).expect("Unable to create config file");
            let config_string = serde_json::to_string_pretty(&new_config).unwrap();
            config_file.write_all(config_string.as_bytes()).expect("Unable to write new config file");
            new_config
        } else {
            let config_file = File::open(loggy_config_file).expect("Unable to read config file");
            let config_file = BufReader::new(config_file);

            let config_file: LocalConfig = serde_json::from_reader(config_file).expect("Unable to read config file");

            config_file
        };
    } else {
        panic!("Unable to get user home directory")
    }
}