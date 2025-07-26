pub mod arg;
pub mod command;
pub mod dependency;
pub mod error;
pub mod project;

use command::{command, LimpCommand};

fn main() {
    let args = command().get_matches();
    let limp_command = LimpCommand::parse(&args).unwrap();
    println!("{:#?}", limp_command)
}
