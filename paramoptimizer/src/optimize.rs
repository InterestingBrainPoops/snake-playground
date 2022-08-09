use crate::{board::Position, eval::score};
use rayon::prelude::*;

pub struct Optimizer {
    pub positions: Vec<Position>,
}

impl Optimizer {
    pub fn local_optimize(&self, k: f64, initial_guess: Vec<i32>) -> Vec<i32> {
        let n_params = initial_guess.len();
        let mut best_e = self.evaluation_error(k, &initial_guess);
        let mut best_par_values = initial_guess;
        let mut improved = true;
        let mut x = 0;
        while improved {
            improved = false;
            for pi in 0..n_params {
                let mut new_par_values = best_par_values.clone();
                new_par_values[pi] += 1i32;
                let mut new_e = self.evaluation_error(k, &new_par_values);
                if new_e < best_e {
                    best_e = new_e;
                    best_par_values = new_par_values;
                    improved = true;
                } else {
                    new_par_values[pi] -= 2i32;
                    new_e = self.evaluation_error(k, &new_par_values);
                    if new_e < best_e {
                        best_e = new_e;
                        best_par_values = new_par_values;
                        improved = true;
                    }
                }
            }
            if x % 10 == 0 {
                println!("error: {}, best_params : {:?}", best_e, best_par_values);
            }
            x += 1;
        }
        print!("Final error: {} ", best_e);
        best_par_values
    }

    pub fn minimize_k(&self, start: f64, params: &Vec<i32>) -> f64 {
        let mut best = start;
        let mut best_e = self.evaluation_error(start, &params);
        let mut improved = true;
        let mut x = 0;
        while improved {
            improved = false;
            let mut new_k = best + 0.001;
            let mut new_e = self.evaluation_error(new_k, &params);
            if new_e < best_e {
                best_e = new_e;
                best = new_k;
                improved = true;
            } else {
                new_k -= 0.002;
                new_e = self.evaluation_error(new_k, &params);
                if new_e < best_e {
                    best_e = new_e;
                    best = new_k;
                    improved = true;
                }
            }

            if x % 10 == 0 {
                println!("error: {}, best_k : {:?}", best_e, best);
            }
            x += 1;
        }
        best
    }

    fn evaluation_error(&self, k: f64, values: &Vec<i32>) -> f64 {
        let n = self.positions.len();
        let n_inverse = 1.0 / (n as f64);
        let sum: f64 = self
            .positions
            .par_iter()
            .map(|position| {
                let score: i32 = position
                    .param_values
                    .iter()
                    .enumerate()
                    .map(|x| *x.1 * values[x.0])
                    .sum();
                let actual: f64 = position.status.into();
                (actual - sigmoid(k, score as f64)).powf(2.0)
            })
            .sum();
        n_inverse * sum
    }
}

fn sigmoid(k: f64, score: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((-k * score) / 400.0))
}
