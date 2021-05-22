mod consts;
mod judge;
mod naive_local;
#[cfg(feature = "naive_manhattan")]
mod naive_manhattan;
mod utils;

use judge::OuterJudge;
use naive_local::NaiveLocalSolver;
#[cfg(feature = "naive_manhattan")]
use naive_manhattan::NaiveManhattan;

fn main() {
    let mut judge = OuterJudge;
    let mut solver = NaiveLocalSolver::new();
    for _ in 0..crate::consts::Q {
        solver.solve(&mut judge);
    }
}
