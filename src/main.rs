use limp::flags::Config;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let conf = Config::parse(&args);
}
