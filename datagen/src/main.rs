use generator::Generator;
use standard::Standard;

mod board;
mod generator;
mod ruleset;
mod standard;

fn main() {
    let x = Generator::new(vec![], Standard::new(11, 11, 2, 25));
}
