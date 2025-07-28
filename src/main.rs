mod arg;
mod command;
mod context;
mod dependency;
mod error;
mod project;
mod storage;
mod utils;

use crate::{
    arg::ArgManager,
    command::{ClapCommandParser, ClapCommandProvider, MainApplication},
};

fn main() {
    let am = &ArgManager;
    let args = MainApplication::command(am).get_matches();
    let app = MainApplication::from_matches(&args).unwrap();
    println!("{app:#?}")
}
