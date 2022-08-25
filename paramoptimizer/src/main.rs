mod board;
mod eval;
mod optimize;

use crate::{
    board::{Game, Position, Status, NUM_PARAMS},
    optimize::Optimizer,
};
use brotli2::read::BrotliDecoder;
use colored::Colorize;
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};
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

/// game data as taken from the SQL db
#[derive(Debug)]
struct GameData {
    /// SQL id
    id: i32,
    /// Game id as from the bsnake servers
    game_id: String,
    /// game type
    game_type: String,
    /// numebr of unique snakes
    unique_snake_count: u8,
    /// brotli blob of the json game
    compressed_frames_json: Option<Vec<u8>>,
}

/// struct used for the datastore feature
#[derive(Serialize, Deserialize, Clone)]
struct Frames {
    /// all of the frames that are being stored
    frames: Vec<Position>,
}
fn main() -> Result<()> {
    // the frames to be used in the optimizer
    let mut frames = vec![];
    // check if we have a datastore file
    if Path::new("./datastore").exists() {
        // if we do then read from it
        println!("Found old datastore, going to use");
        // get t0
        let start = Instant::now();
        // open the file
        let thing = fs::read(Path::new("./datastore")).unwrap();
        // deserialize the file from messagepack to the Frames struct
        let thing2 = <Frames>::deserialize(&mut rmp_serde::Deserializer::new(&thing[..])).unwrap();
        // set frames to the Frames.positions
        frames = thing2.frames;
        println!("Finished getting all frames from file");
        println!("Time taken : {:?}", Instant::now() - start);
    } else {
        // open connection to the sqlite db
        let conn = Connection::open("./two_snake_snakedump.sqlite")?;
        // prepare the statement to grab all of the info
        let mut stmt = conn.prepare("SELECT id, game_id , game_type, unique_snake_count , compressed_frames_json FROM snake_games")?;
        // get t0
        let start = Instant::now();

        println!("Getting sql data");
        // get an iterator over all of the gamedata
        let person_iter = stmt.query_map([], |row| {
            Ok(GameData {
                id: row.get(0)?,
                game_id: row.get(1)?,
                game_type: row.get(2)?,
                unique_snake_count: row.get(3)?,
                compressed_frames_json: row.get(4)?,
            })
        })?;

        println!("Time taken : {:?}", Instant::now() - start);
        println!("Collecting all games from sql");
        // get t0
        let start = Instant::now();
        // storage struct for all of the games
        let mut games = vec![];
        // go throug each game blob
        for contents in person_iter {
            // create the decompressor for the brotli blob
            let mut decompressor = BrotliDecoder::new(Cursor::new(
                contents.unwrap().compressed_frames_json.unwrap(),
            ));
            // storage for the un-brotli'd blob
            let mut contents = String::new();
            // decompress the brotli into the contents string
            decompressor.read_to_string(&mut contents).unwrap();
            // deserialize from json using serde_json
            let game: Game = serde_json::from_str(&contents).unwrap();
            // my health set to 100
            let mut me_health = 100;
            // their health set to 100
            let mut they_health = 100;
            // go through all of the posiitons in the game
            for (idx, _) in game.positions.iter().enumerate() {
                // ignore startpos and endpos
                if idx == 0 || game.positions.len() - 1 == idx {
                    continue;
                }
                // store food from previous position
                let old_food = game.positions[idx - 1].food.clone();
                // find my head
                let my_head = game.positions[idx].snakes[0].body[0];
                // find the other guys head
                let other_head = game.positions[idx].snakes[1].body[0];
                // decrement my health
                me_health -= 1;
                // decrement their health
                they_health -= 1;
                // if the old food contains my head, set my health to 100
                if old_food.contains(&my_head) {
                    me_health = 100;
                }
                // if the old food contains the other guys head, set their health to 100
                if old_food.contains(&other_head) {
                    they_health = 100;
                }
            }
            // find the last position in the game
            let last = game.positions.last().unwrap();
            // check if they are dead
            let they_dead = last.snakes[1].dead(last, they_health);
            // check if I am dead
            let me_dead = last.snakes[0].dead(last, me_health);
            // calcualte the end game status
            let status = if me_dead && they_dead {
                // if we are both dead, then its a draw
                Status::Draw
            } else if !me_dead && they_dead {
                // if i am alive and they are dead, then I am the winner
                Status::Win
            } else {
                // They are alive and i am dead, thus i loose
                Status::Loss
            };
            // reset health values for me and them
            let mut me_health = 100;
            let mut they_health = 100;
            // storage for all positions of this game
            let mut positions = vec![];
            // go through all positions
            for (idx, position) in game.positions.iter().enumerate() {
                // ignore endpos
                if game.positions.len() - 1 == idx {
                    continue;
                }
                // ignore startpos
                if idx == 0 {
                    continue;
                }
                // find old food
                let old_food = game.positions[idx - 1].food.clone();
                // my head
                let my_head = game.positions[idx].snakes[0].body[0];
                // their head
                let other_head = game.positions[idx].snakes[1].body[0];
                // decrement my health
                me_health -= 1;
                // decrement their health
                they_health -= 1;
                // if old food contains my head, set my health to 100
                if old_food.contains(&my_head) {
                    me_health = 100;
                }
                // if old food contains other head, set other health to 100
                if old_food.contains(&other_head) {
                    they_health = 100;
                }
                // calcualte the bitboard
                let mut all_bb: u128 = 0;
                for snake in &position.snakes {
                    for piece in &snake.body {
                        all_bb |= u128::from(*piece);
                    }
                }
                // add the current position to the positions vec
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
        // progress bar
        let bar = ProgressBar::new(games.len() as u64);
        // score all of the games
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
        // serialize this into the buf
        Frames {
            frames: frames.clone(),
        }
        .serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .unwrap();
        // write the buffer into the datastore file
        fs::write("./datastore", buf).expect("unable to write to ./datastore");
        println!("Time taken : {:?}", Instant::now() - start);

        println!("Finished writing all frames to file");
    }

    println!("Number of frames: {}", frames.len());
    // let x = Optimizer { positions: games };
    // let min_k = x.minimize_k(0.16, &vec![36, -41, -4, 113]);
    // println!("min_k : {min_k}");

    // initialize the optimzer
    let optimizer = Optimizer { positions: frames };
    let start = vec![
        0.05181902380016712,
        -0.01775290032244476,
        0.03470805959801342,
        0.005056952455828632,
    ];
    let t0 = Instant::now();
    let mut start_mse = optimizer.better_evaluation_error(&start);
    for x in 0..100 {
        start_mse = optimizer.better_evaluation_error(&start);
    }

    println!("Time taken : {:?}", (Instant::now() - t0) / 100);
    println!("Starting MSE: {}", start_mse);
    let mut walked_inputs = vec![];
    let mut rng = thread_rng();
    for _ in 0..10 {
        let mut adder = vec![];
        for x in &start {
            println!("{:?}", (x - x * 0.05)..(x + x * 0.05));
            let first = x - x * 0.05;
            let last = x + x * 0.05;
            let range = if last < first {
                last..first
            } else {
                first..last
            };
            adder.push(x + rng.gen_range(range));
        }
        walked_inputs.push(adder);
    }
    let mut new_walked = vec![];
    for (idx, input) in walked_inputs.iter().enumerate() {
        // add in the parameters and optimize
        let optimized = optimizer.local_optimize(0.155, input.clone(), 500);
        let new_mse = optimizer.better_evaluation_error(input);

        println!("Iteration : {idx}");
        // println!("{} : {}", "Iteration".yellow(), x);
        if new_mse < start_mse {
            new_walked.push((new_mse, optimized));
        }
    }
    if new_walked.is_empty() {
        println!("None were better this round");
    }

    let mut smallest = new_walked[0].clone();
    for x in &new_walked {
        if x.0 < smallest.0 {
            smallest = x.clone();
        }
    }
    println!("Best parameters : {:?}", smallest.1);
    println!("MSE : {}", smallest.0);
    /*        vec![
        0.05181902380016712,
        -0.01775290032244476,
        0.00035877484889895907,
        0.03470805959801342,
        0.005056952455828632,
    ], */
    Ok(())
}
