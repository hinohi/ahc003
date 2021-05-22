mod consts;
mod judge;
mod solver;

fn main() {
    let mut judge = crate::judge::OuterJudge;
    let solver = crate::solver::NaiveManhattan;
    for _ in 0..crate::consts::Q {
        solver.solve(&mut judge);
    }
}
