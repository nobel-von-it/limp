pub mod arg;
pub mod command;
pub mod dependency;
pub mod error;
pub mod project;

use crate::{
    arg::ArgManager,
    command::{ClapCommandParser, ClapCommandProvider, MainApplication},
};

fn main() {
    let am = &ArgManager;
    let args = MainApplication::command(am).get_matches();
    let app = MainApplication::from_matches(&args).unwrap();
    println!("{:#?}", app)
}
