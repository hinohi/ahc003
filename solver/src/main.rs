mod consts;
mod judge;
mod solver;

use judge::OuterJudge;
use solver::NaiveManhattan;

fn main() {
    let mut judge = OuterJudge;
    let solver = NaiveManhattan;
    for _ in 0..crate::consts::Q {
        solver.solve(&mut judge);
    }
}
