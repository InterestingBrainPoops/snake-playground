use crate::{board::Position, eval::score};

pub struct Optimizer {
    pub positions: Vec<Position>,
}

impl Optimizer {
    pub fn local_optimize(&self, initial_guess: Vec<i32>) -> Vec<i32> {
        let n_params = initial_guess.len();
        let mut best_e = self.evaluation_error(&initial_guess);
        let mut best_par_values = initial_guess;
        let mut improved = true;
        let mut x = 0;
        while improved {
            improved = false;
            for pi in 0..n_params {
                let mut new_par_values = best_par_values.clone();
                new_par_values[pi] += 1i32;
                let mut new_e = self.evaluation_error(&new_par_values);
                if new_e < best_e {
                    best_e = new_e;
                    best_par_values = new_par_values;
                    improved = true;
                } else {
                    new_par_values[pi] -= 2i32;
                    new_e = self.evaluation_error(&new_par_values);
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
        best_par_values
    }

    fn evaluation_error(&self, values: &Vec<i32>) -> f64 {
        let n = self.positions.len();
        let n_inverse = 1.0 / (n as f64);
        let mut sum = 0.0;
        for position in &self.positions {
            let score = score(position, values);
            let actual: f64 = position.status.into();
            sum += (actual - sigmoid(0.16, score as f64)).powf(2.0);
        }
        n_inverse * sum
    }
}

fn sigmoid(k: f64, score: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((-k * score) / 400.0))
}
