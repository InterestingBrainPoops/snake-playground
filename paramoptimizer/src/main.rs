mod board;
mod eval;
mod optimize;

use crate::{
    board::{Game, Position, Status, NUM_PARAMS},
    optimize::Optimizer,
};
use brotli2::read::BrotliDecoder;
use indicatif::ProgressBar;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};

use std::{
    fs,
    io::{prelude::*, Cursor},
    path::Path,
    time::Instant,
};

#[derive(Debug)]
struct GameData {
    id: i32,
    game_id: String,
    game_type: String,
    unique_snake_count: u8,
    compressed_frames_json: Option<Vec<u8>>,
}
#[derive(Serialize, Deserialize, Clone)]
struct Frames {
    frames: Vec<Position>,
}
fn main() -> Result<()> {
    let mut frames = vec![];
    if Path::new("./datastore").exists() {
        println!("Found old datastore, going to use");
        let start = Instant::now();
        let thing = fs::read(Path::new("./datastore")).unwrap();
        let thing2 = <Frames>::deserialize(&mut rmp_serde::Deserializer::new(&thing[..])).unwrap();
        frames = thing2.frames;
        println!("Finished getting all frames from file");
        println!("Time taken : {:?}", Instant::now() - start);
    } else {
        let conn = Connection::open("./two_snake_snakedump.sqlite")?;

        let mut stmt = conn.prepare("SELECT id, game_id , game_type, unique_snake_count , compressed_frames_json FROM snake_games")?;
        let start = Instant::now();

        println!("Getting sql data");
        let person_iter = stmt.query_map([], |row| {
            Ok(GameData {
                id: row.get(0)?,
                game_id: row.get(1)?,
                game_type: row.get(2)?,
                unique_snake_count: row.get(3)?,
                compressed_frames_json: row.get(4)?,
            })
        })?;
        let mut games = vec![];
        println!("Time taken : {:?}", Instant::now() - start);
        println!("Collecting all games from sql");
        let start = Instant::now();

        for contents in person_iter {
            let mut decompressor = BrotliDecoder::new(Cursor::new(
                contents.unwrap().compressed_frames_json.unwrap(),
            ));
            let mut contents = String::new();
            decompressor.read_to_string(&mut contents).unwrap();
            let game: Game = serde_json::from_str(&contents).unwrap();
            let mut me_health = 100;
            let mut they_health = 100;
            for (idx, _) in game.positions.iter().enumerate() {
                if idx == 0 {
                    continue;
                }
                let old_food = game.positions[idx - 1].food.clone();
                let my_head = game.positions[idx].snakes[0].body[0];
                let other_head = game.positions[idx].snakes[1].body[0];
                me_health -= 1;
                they_health -= 1;
                if old_food.contains(&my_head) {
                    me_health = 100;
                }
                if old_food.contains(&other_head) {
                    they_health = 100;
                }
            }

            let last = game.positions.last().unwrap();
            let they_dead = last.snakes[1].dead(last, they_health);
            let me_dead = last.snakes[0].dead(last, me_health);
            let status = if me_dead && they_dead {
                Status::Draw
            } else if !me_dead && they_dead {
                Status::Win
            } else {
                Status::Loss
            };
            let mut positions = vec![];
            for (idx, position) in game.positions.iter().enumerate() {
                if game.positions.len() - 1 == idx {
                    continue;
                }
                if idx == 0 {
                    continue;
                }
                let old_food = game.positions[idx - 1].food.clone();
                let my_head = game.positions[idx].snakes[0].body[0];
                let other_head = game.positions[idx].snakes[1].body[0];
                me_health -= 1;
                they_health -= 1;
                if old_food.contains(&my_head) {
                    me_health = 100;
                }
                if old_food.contains(&other_head) {
                    they_health = 100;
                }
                let mut all_bb: u128 = 0;
                for snake in &position.snakes {
                    for piece in &snake.body {
                        all_bb |= u128::from(*piece);
                    }
                }
                positions.push(Position {
                    status,
                    my_health: me_health,
                    their_health: they_health,
                    board: position.clone(),
                    param_values: Default::default(),
                    all_bb,
                });
            }
            games.append(&mut positions);
        }
        println!("Time taken : {:?}", Instant::now() - start);
        println!("Finished adding all frames");
        println!("Processing all frames");
        let start = Instant::now();

        let bar = ProgressBar::new(games.len() as u64);

        games.par_iter_mut().for_each(|x| {
            let param_values = eval::score(x);
            x.param_values = param_values;
            bar.inc(1);
        });
        bar.finish();
        println!();
        println!("Time taken : {:?}", Instant::now() - start);
        println!("Finished processing all frames");
        frames = games;

        println!("Loading frames into datastore file");
        let start = Instant::now();

        let mut buf = vec![];
        Frames {
            frames: frames.clone(),
        }
        .serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .unwrap();

        fs::write("./datastore", buf).expect("unable to write to ./datastore");
        println!("Time taken : {:?}", Instant::now() - start);

        println!("Finished writing all frames to file");
    }

    println!("Number of frames: {}", frames.len());
    // let x = Optimizer { positions: games };
    // let min_k = x.minimize_k(0.16, &vec![36, -41, -4, 113]);
    // println!("min_k : {min_k}");
    let start = Instant::now();
    let x = Optimizer { positions: frames };
    let new_params = x.local_optimize(0.155, vec![44, -20, -4, 51, 6]);
    println!("Final parameters: {:?}", new_params);
    println!("Time taken: {:?}", Instant::now() - start);
    Ok(())
}
