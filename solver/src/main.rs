mod backpropagation;
mod consts;
mod judge;
#[cfg(feature = "naive_local")]
mod naive_local;
#[cfg(feature = "naive_manhattan")]
mod naive_manhattan;
mod utils;

use std::{env::args, fs};

use crate::backpropagation::BPSolver;
use crate::judge::{BestJudge, Judge};
#[cfg(feature = "naive_local")]
use crate::naive_local::NaiveLocalSolver;
#[cfg(feature = "naive_manhattan")]
use naive_manhattan::NaiveManhattan;

fn main() {
    let input = args()
        .skip(1)
        .next()
        .map_or("".to_owned(), |path| fs::read_to_string(path).unwrap());
    let mut judge = BestJudge::new(&input);
    let mut solver = BPSolver::new();
    for q in 0..crate::consts::Q {
        solver.solve(&mut judge, q);
    }
    let score = judge.score();
    if score != 0 {
        eprintln!("Score = {}", score);
    }
}
