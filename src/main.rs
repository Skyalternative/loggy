use structopt::StructOpt;

use config::CliOptions;

use crate::config::get_config;

mod config;

fn main() {
    let options = get_config();
    println!("{:?}", options);
}
