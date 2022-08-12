use std::f64::consts::E;

use crate::{board::Position, eval::score};
use rayon::prelude::*;

pub struct Optimizer {
    pub positions: Vec<Position>,
}

impl Optimizer {
    /// entry point function for optimizing the initial guess across the dataset in self.
    pub fn local_optimize(&self, k: f64, initial_guess: Vec<f64>) -> Vec<f64> {
        self.perceptron_learn(initial_guess)
    }

    /// perceptron learning function
    fn perceptron_learn(&self, initial_guess: Vec<f64>) -> Vec<f64> {
        // initialize the weights as the initial guess.
        let mut weights = initial_guess;
        // iteration count
        let mut iteration = 0;
        // loop forever
        loop {
            // iterate through all positions
            for position in &self.positions {
                // compute current score
                let score: f64 = better_sigmoid(
                    position
                        .param_values
                        .iter()
                        .enumerate()
                        .map(|x| *x.1 * weights[x.0])
                        .sum(),
                );
                // get the actual value
                let actual: f64 = position.status.into();
                // compute the error
                let error = (score - actual) * transfer_derivative(score);
                // update weights correspondingly
                for (idx, weight) in weights.iter_mut().enumerate() {
                    // learning rate is 0.0001
                    *weight -= (error * 0.0001 * position.param_values[idx]);
                }
            }
            iteration += 1;
            if iteration % 10 == 0 {
                // print every 10 iterations to prevent terminal spam
                println!(
                    "MSE: {}, Weights: {:?}, iteration : {}",
                    self.better_evaluation_error(&weights),
                    weights,
                    iteration
                );
            }
        }
    }
// local search optimization routine
    fn local_search(&self, k: f64, initial_guess: Vec<f64>) -> Vec<f64> {
        // number of parameters
        let n_params = initial_guess.len();
        // lowest error so far, initialized as the current error
        let mut best_e = self.evaluation_error(k, &initial_guess);
        // best parameter values so far
        let mut best_par_values = initial_guess;
        // have we improved this iteration
        let mut improved = true;
        // iteration count
        let mut x = 0;
        while improved {
            // assue we wont improve
            improved = false;
            // go through all parameters
            for pi in 0..n_params {
                // make a new parameter value vector
                let mut new_par_values = best_par_values.clone();
                // increment it
                new_par_values[pi] += 1.0;
                // calculate the mse across the dataset
                let mut new_e = self.evaluation_error(k, &new_par_values);
                // if the new error is less than the current error, update the error and param valeus
                if new_e < best_e {
                    best_e = new_e;
                    best_par_values = new_par_values;
                    improved = true;
                } else { // if it isnt, subtract it by two ( + 1 - 2 has the effect of reducing the current parameter by one)
                    new_par_values[pi] -= 2.0;
                    // find the new error
                    new_e = self.evaluation_error(k, &new_par_values);
                    if new_e < best_e {
                        // if the new error is less than the current error, update the error and param values
                        best_e = new_e;
                        best_par_values = new_par_values;
                        improved = true;
                    }
                }
            }
            if x % 10 == 0 {
                // print every 10 iterations to prevent terminal spam
                println!("error: {}, best_params : {:?}", best_e, best_par_values);
            }
            x += 1;
        }
        // print the final error so that we have an idea of just how close it is now
        print!("Final error: {} ", best_e);
        best_par_values
    }

    /// routine to minimize the k value for your evaluation.
    /// K is a scaling value used ONLY IN LOCAL SEARCH
    /// you dont need to update this every time you tune, only once.
    /// DO THIS BEFORE YOU START TUNING
    /// ITS VERY IMPORTANT
    /// you can start the scaling value at 0
    /// It optimizes k using localsearch.
    pub fn minimize_k(&self, start: f64, params: &Vec<f64>) -> f64 {
        // best k
        let mut best = start;
        // best error
        let mut best_e = self.evaluation_error(start, &params);
        // have we improved this iteration
        let mut improved = true;
        // iteration count
        let mut x = 0;
        while improved {
            // assume we wont improve
            improved = false;
            // set the new k
            let mut new_k = best + 0.001;
            // find the mse using the new k
            let mut new_e = self.evaluation_error(new_k, &params);
            if new_e < best_e {
                // if the error is lower, update best_e
                best_e = new_e;
                // update best k
                best = new_k;
                // we have improved
                improved = true;
            } else {
                // go -.001
                new_k -= 0.002;
                // calculate the mse using the lowered k and the parameters
                new_e = self.evaluation_error(new_k, &params);
                if new_e < best_e {
                    // if the error is lower, update best_e
                    best_e = new_e;
                    // update best k
                    best = new_k;
                    // we have improved
                    improved = true;
                }
            }

            if x % 10 == 0 {
                // print every 10 to prevent terminal spam
                println!("error: {}, best_k : {:?}", best_e, best);
            }
            // update iter count
            x += 1;
        }
        best
    }
    // find the evaluation error given a k value and values
    // this function is only used in local search
    fn evaluation_error(&self, k: f64, values: &Vec<f64>) -> f64 {
        // total number of positions
        let n = self.positions.len();
        
        // the inverse of the number of positions
        let n_inverse = 1.0 / (n as f64);
        
        // sum of all of the squared errors
        let sum: f64 = self
            .positions
            .par_iter()
            .map(|position| {
                // calculate the score for the given position
                let score: f64 = position
                    .param_values
                    .iter()
                    .enumerate()
                    .map(|x| *x.1 * values[x.0])
                    .sum();
                // find the actual value of the position
                let actual: f64 = position.status.into();
                // return the squared error
                (actual - sigmoid(k, score as f64)).powf(2.0)
            })
            .sum();
        // sum / n_positions
        n_inverse * sum
    }

    // uses normal sigmoid with no scaling factor
    // used in the perceptron optimizer
    pub fn better_evaluation_error(&self, values: &Vec<f64>) -> f64 {
        // number of positions
        let n = self.positions.len();
        // inverse of number of positions
        let n_inverse = 1.0 / (n as f64);
        // sum of the square errors acrosss the entire dataset
        let sum: f64 = self
            .positions
            .par_iter()
            .map(|position| {
                // score of the position
                let score: f64 = position
                    .param_values
                    .iter()
                    .enumerate()
                    .map(|x| *x.1 * values[x.0])
                    .sum();
                // the actual value from the ending of the game
                let actual: f64 = position.status.into();
                // return the squared error
                (actual - better_sigmoid(score as f64)).powf(2.0)
            })
            .sum();
        // sum / n_positions
        n_inverse * sum
    }
}

// sigmoid with scaling factor
fn sigmoid(k: f64, score: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((-k * score) / 400.0))
}
// sigmoid as described in wikipedia
fn better_sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + E.powf(-value))
}
// transfer derivative of sigmoid
fn transfer_derivative(value: f64) -> f64 {
    value * (1.0 - value)
}
