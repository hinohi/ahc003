use crate::consts::N;
use crate::judge::Judge;
use crate::utils::{calc_path, dir_to_str, Direction, EdgeCost};

pub struct NaiveLocalSolver {
    vertical: [[f64; N]; N - 1],
    horizon: [[f64; N - 1]; N],
}

impl EdgeCost for NaiveLocalSolver {
    #[inline]
    fn vertical(&self, y: usize, x: usize) -> f64 {
        self.vertical[y][x]
    }

    #[inline]
    fn horizon(&self, y: usize, x: usize) -> f64 {
        self.horizon[y][x]
    }
}

impl NaiveLocalSolver {
    pub fn new() -> NaiveLocalSolver {
        NaiveLocalSolver {
            vertical: [[5000.0; N]; N - 1],
            horizon: [[5000.0; N - 1]; N],
        }
    }

    pub fn solve<J: Judge>(&mut self, judge: &mut J) {
        let query = judge.next_query();
        let (dist, path) = calc_path(self, (query[0], query[1]), (query[2], query[3]));
        judge.answer(&dir_to_str(&path));
        let length = judge.path_length() as f64;

        let mut y = query[0];
        let mut x = query[1];
        let mut st_v = [(0.0, 0); N];
        let mut st_h = [(0.0, 0); N];
        for &d in path.iter() {
            match d {
                Direction::U => {
                    y -= 1;
                    st_v[x].0 += self.vertical[y][x];
                    st_v[x].1 += 1;
                }
                Direction::D => {
                    st_v[x].0 += self.vertical[y][x];
                    st_v[x].1 += 1;
                    y += 1
                }
                Direction::L => {
                    x -= 1;
                    st_h[y].0 += self.horizon[y][x];
                    st_h[y].1 += 1;
                }
                Direction::R => {
                    st_h[y].0 += self.horizon[y][x];
                    st_h[y].1 += 1;
                    x += 1;
                }
            }
        }

        let p: f64 = 0.4;
        for (x, &(st_len, count)) in st_v.iter().enumerate() {
            if count == 0 {
                continue;
            }
            let avg = length * st_len / dist / count as f64;
            for y in 0..N - 1 {
                self.vertical[y][x] = mix(p, self.vertical[y][x], avg);
            }
        }
        for (y, &(st_len, count)) in st_h.iter().enumerate() {
            if count == 0 {
                continue;
            }
            let avg = length * st_len / dist / count as f64;
            for x in 0..N - 1 {
                self.horizon[y][x] = mix(p, self.horizon[y][x], avg);
            }
        }
    }
}

fn mix(p: f64, a: f64, b: f64) -> f64 {
    a * (1.0 - p) + b * p
}
