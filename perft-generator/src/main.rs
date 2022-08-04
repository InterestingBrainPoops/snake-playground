mod board;
mod cartprod;
mod coordinate;
mod genmove;
mod perft;
mod snake;
use std::fs;

struct Args {
    #[clap(short, long, value_parser)]
    path: String,
}
fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
