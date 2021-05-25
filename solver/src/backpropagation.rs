use crate::consts::{N, Q};
use crate::judge::Judge;
use crate::utils::{calc_path, dir_to_str, Direction, EdgeCost};

pub struct BPSolver {
    v_street: [f64; N],
    h_street: [f64; N],
    v_edge: [[f64; N]; N - 1],
    h_edge: [[f64; N - 1]; N],
}

impl EdgeCost for BPSolver {
    #[inline]
    fn vertical(&self, y: usize, x: usize) -> f64 {
        (self.v_street[x] + self.v_edge[y][x]).max(1.0)
    }

    #[inline]
    fn horizon(&self, y: usize, x: usize) -> f64 {
        (self.h_street[y] + self.h_edge[y][x]).max(1.0)
    }
}

impl BPSolver {
    pub fn new() -> BPSolver {
        BPSolver {
            v_street: [5000.0; N],
            h_street: [5000.0; N],
            v_edge: [[0.0; N]; N - 1],
            h_edge: [[0.0; N - 1]; N],
        }
    }

    pub fn solve<J: Judge>(&mut self, judge: &mut J, q: usize) {
        let query = judge.next_query();
        let (now_dist, path) = calc_path(self, (query[0], query[1]), (query[2], query[3]));
        judge.answer(&dir_to_str(&path));
        // 1.0033534773107562 = log(1.1/0.9) / (1.1 - 0.9)
        let actual_dist = judge.path_length() as f64 * 1.0033534773107562;
        let q = q as f64 / Q as f64;
        let lr = 1e2 * (1.0 - q) + 1e-1 * q;
        let err = (if actual_dist > now_dist { 1.0 } else { -1.0 }) * rl;
        println!("{}", dir_to_str(&path));

        let mut y = query[0];
        let mut x = query[1];
        for d in path {
            match d {
                Direction::U => {
                    y -= 1;
                    self.v_street[x] += err;
                    self.v_edge[y][x] += err;
                }
                Direction::D => {
                    self.v_street[x] += err;
                    self.v_edge[y][x] += err;
                    y += 1;
                }
                Direction::L => {
                    x -= 1;
                    self.h_street[y] += err;
                    self.h_edge[y][x] += err;
                }
                Direction::R => {
                    self.h_street[y] += err;
                    self.h_edge[y][x] += err;
                    x += 1;
                }
            }
        }
    }
}
