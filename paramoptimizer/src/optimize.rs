use std::f64::consts::E;

use crate::{board::Position, eval::score};
use rayon::prelude::*;

pub struct Optimizer {
    pub positions: Vec<Position>,
}

impl Optimizer {
    pub fn local_optimize(&self, k: f64, initial_guess: Vec<f64>) -> Vec<f64> {
        self.perceptron_learn(initial_guess)
    }

    fn perceptron_learn(&self, initial_guess: Vec<f64>) -> Vec<f64> {
        let mut weights = initial_guess;
        let mut iteration = 0;
        loop {
            for position in &self.positions {
                // find guess
                let score: f64 = better_sigmoid(
                    position
                        .param_values
                        .iter()
                        .enumerate()
                        .map(|x| *x.1 * weights[x.0])
                        .sum(),
                );
                // calculate error
                let actual: f64 = position.status.into();
                let error = (score - actual) * transfer_derivative(score);
                // update weights correspondingly
                for (idx, weight) in weights.iter_mut().enumerate() {
                    *weight -= (error * 0.0001 * position.param_values[idx]);
                }
            }
            iteration += 1;
            if iteration % 10 == 0 {
                println!(
                    "MSE: {}, Weights: {:?}, iteration : {}",
                    self.better_evaluation_error(&weights),
                    weights,
                    iteration
                );
            }
        }
    }

    fn local_search(&self, k: f64, initial_guess: Vec<f64>) -> Vec<f64> {
        let n_params = initial_guess.len();
        let mut best_e = self.evaluation_error(k, &initial_guess);
        let mut best_par_values = initial_guess;
        let mut improved = true;
        let mut x = 0;
        while improved {
            improved = false;
            for pi in 0..n_params {
                let mut new_par_values = best_par_values.clone();
                new_par_values[pi] += 1.0;
                let mut new_e = self.evaluation_error(k, &new_par_values);
                if new_e < best_e {
                    best_e = new_e;
                    best_par_values = new_par_values;
                    improved = true;
                } else {
                    new_par_values[pi] -= 2.0;
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

    pub fn minimize_k(&self, start: f64, params: &Vec<f64>) -> f64 {
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

    fn evaluation_error(&self, k: f64, values: &Vec<f64>) -> f64 {
        let n = self.positions.len();
        let n_inverse = 1.0 / (n as f64);
        let sum: f64 = self
            .positions
            .par_iter()
            .map(|position| {
                let score: f64 = position
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
    pub fn better_evaluation_error(&self, values: &Vec<f64>) -> f64 {
        let n = self.positions.len();
        let n_inverse = 1.0 / (n as f64);
        let sum: f64 = self
            .positions
            .par_iter()
            .map(|position| {
                let score: f64 = position
                    .param_values
                    .iter()
                    .enumerate()
                    .map(|x| *x.1 * values[x.0])
                    .sum();
                let actual: f64 = position.status.into();
                (actual - better_sigmoid(score as f64)).powf(2.0)
            })
            .sum();
        n_inverse * sum
    }
}

fn sigmoid(k: f64, score: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((-k * score) / 400.0))
}
fn better_sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + E.powf(-value))
}
fn transfer_derivative(value: f64) -> f64 {
    value * (1.0 - value)
}
