use limp::actions::CommandHandler;

fn main() {
    let matches = CommandHandler::build().get_matches();
    let ch = CommandHandler::parse(&matches);
    if let Err(e) = ch.make_action() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}
