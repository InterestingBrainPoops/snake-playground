use std::{f32::consts::E, ops::Mul};

use crate::{
    board::{Position, NUM_PARAMS},
    eval::score,
};
use ndarray::{Array, ArrayBase};
use rayon::prelude::*;

pub struct Optimizer {
    pub positions: Vec<Position>,
}

impl Optimizer {
    /// entry point function for opt&imizing the initial guess across the dataset in self.
    pub fn local_optimize(&self, k: f32, initial_guess: Vec<f32>, limit: u64) -> Vec<f32> {
        self.adam_optimizer(&initial_guess, limit)
    }

    /// perceptron learning function
    fn perceptron_learn(&self, initial_guess: Vec<f32>) -> Vec<f32> {
        // initialize the weights as the initial guess.
        let mut weights = initial_guess;
        // iteration count
        let mut iteration = 0;
        // loop forever
        loop {
            // iterate through all positions
            for position in &self.positions {
                // compute current score
                let score: f32 = better_sigmoid(
                    position
                        .param_values
                        .iter()
                        .enumerate()
                        .map(|x| *x.1 * weights[x.0])
                        .sum(),
                );
                // get the actual value
                let actual: f32 = position.status.into();
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
    fn gradient(&self, guess: &Vec<f32>) -> Vec<f32> {
        let mut out = vec![];
        for x in 0..guess.len() {
            let mut new_guess = guess.clone();
            new_guess[x] -= 0.001;
            out.push(
                -(self.better_evaluation_error(&new_guess) - self.better_evaluation_error(&guess))
                    / 0.001,
            );
        }
        out
    }
    fn adam_optimizer(&self, initial_guess: &Vec<f32>, iteration_limit: u64) -> Vec<f32> {
        let initial_mse = self.better_evaluation_error(&initial_guess);
        let alpha = 0.00001;
        let beta_1: f32 = 0.9;
        let beta_2 = 0.999;
        let epsilon = 1e-8;
        let mut theta_0 = initial_guess.clone();
        let mut m_t = vec![0.; theta_0.len()];
        let mut v_t = vec![0.; theta_0.len()];
        let mut t = 0;

        loop {
            t += 1;
            let g_t = self.gradient(&theta_0);
            m_t = add_vec(&multiply(beta_1, &m_t), &multiply((1. - beta_1), &g_t));
            v_t = add_vec(
                &multiply(beta_2, &v_t),
                &multiply(1. - beta_2, &multiply_vec(&g_t, &g_t)),
            );
            let m_cap = divide(&m_t, 1. - (beta_1.powi(t)));
            let v_cap = divide(&v_t, 1. - (beta_2.powi(t)));
            let theta_prev = theta_0.clone();
            theta_0 = subtract(
                &theta_0,
                &divide_vec(
                    &multiply(alpha, &m_cap),
                    &add(
                        &v_cap.iter().map(|&x| x.sqrt()).collect::<Vec<f32>>(),
                        epsilon,
                    ),
                ),
            );
            if t % 100 == 0 {
                let eval = self.better_evaluation_error(&theta_0);
                println!("MSE : {}", eval);
                println!(
                    "Iteration : {}, Parameters : {:?}. Improvement : {}",
                    t,
                    theta_0,
                    eval - initial_mse
                );
            }
            if theta_0 == theta_prev {
                break;
            }
            if t as u64 == iteration_limit {
                break;
            }
        }

        theta_0
    }

    // local search optimization routine
    fn local_search(&self, k: f32, initial_guess: Vec<f32>) -> Vec<f32> {
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
                } else {
                    // if it isnt, subtract it by two ( + 1 - 2 has the effect of reducing the current parameter by one)
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
    pub fn minimize_k(&self, start: f32, params: &Vec<f32>) -> f32 {
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
    fn evaluation_error(&self, k: f32, values: &Vec<f32>) -> f32 {
        // total number of positions
        let n = self.positions.len();

        // the inverse of the number of positions
        let n_inverse = 1.0 / (n as f32);

        // sum of all of the squared errors
        let sum: f32 = self
            .positions
            .par_iter()
            .map(|position| {
                // calculate the score for the given position
                let score: f32 = position
                    .param_values
                    .iter()
                    .enumerate()
                    .map(|x| *x.1 * values[x.0])
                    .sum();
                // find the actual value of the position
                let actual: f32 = position.status.into();
                // return the squared error
                (actual - sigmoid(k, score as f32)).powf(2.0)
            })
            .sum();
        // sum / n_positions
        n_inverse * sum
    }

    // uses normal sigmoid with no scaling factor
    // used in the perceptron optimizer
    pub fn better_evaluation_error(&self, values: &Vec<f32>) -> f32 {
        // number of positions
        let n = self.positions.len();
        // inverse of number of positions
        let n_inverse = 1.0 / (n as f32);
        // sum of the square errors acrosss the entire dataset
        let sum: f32 = self
            .positions
            .par_iter()
            .map(|position| {
                let params = Array::from_vec(position.param_values.to_vec());
                let nvalues = Array::from_vec(values.clone());
                // score of the position
                let score: f32 = params.dot(&nvalues);
                // the actual value from the ending of the game
                let actual: f32 = position.status.into();
                // return the squared error
                (actual - better_sigmoid(score as f32)).powf(2.0)
            })
            .sum();
        // sum / n_positions
        n_inverse * sum
    }
}

// sigmoid with scaling factor
fn sigmoid(k: f32, score: f32) -> f32 {
    1.0 / (1.0 + 10f32.powf((-k * score) / 400.0))
}
// sigmoid as described in wikipedia
fn better_sigmoid(value: f32) -> f32 {
    1.0 / (1.0 + E.powf(-value))
}
// transfer derivative of sigmoid
fn transfer_derivative(value: f32) -> f32 {
    value * (1.0 - value)
}

fn multiply(x: f32, y: &Vec<f32>) -> Vec<f32> {
    y.iter().map(|&z| x * z).collect::<Vec<f32>>()
}
fn multiply_vec(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    x.iter()
        .enumerate()
        .map(|(idx, &z)| z * y[idx])
        .collect::<Vec<f32>>()
}
fn divide(x: &Vec<f32>, y: f32) -> Vec<f32> {
    x.iter().map(|&z| z / y).collect::<Vec<f32>>()
}
fn subtract(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    x.iter()
        .enumerate()
        .map(|(idx, &z)| z - y[idx])
        .collect::<Vec<f32>>()
}
fn add(x: &Vec<f32>, y: f32) -> Vec<f32> {
    x.iter().map(|&z| y + z).collect::<Vec<f32>>()
}
fn add_vec(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    x.iter()
        .enumerate()
        .map(|(idx, &z)| z + y[idx])
        .collect::<Vec<f32>>()
}
fn divide_vec(x: &Vec<f32>, y: &Vec<f32>) -> Vec<f32> {
    x.iter()
        .enumerate()
        .map(|(idx, &z)| z / y[idx])
        .collect::<Vec<f32>>()
}
type Thing = Vec<f32>;
use packed_simd::f32x4;

pub fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 4 == 0);

    a.chunks_exact(4)
        .map(f32x4::from_slice_unaligned)
        .zip(b.chunks_exact(4).map(f32x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f32x4>()
        .sum()
}
