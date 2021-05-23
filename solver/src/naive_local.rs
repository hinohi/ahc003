use std::collections::BinaryHeap;

use crate::consts::N;
use crate::judge::Judge;
use crate::utils::{dir_to_str, Direction};

pub struct NaiveLocalSolver {
    vertical: [[f64; N]; N - 1],
    horizon: [[f64; N - 1]; N],
}

#[derive(Debug)]
struct State {
    dist: f64,
    pos: (usize, usize),
    path: Vec<Direction>,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.dist == other.dist
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        // flip for BinaryHeap
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl NaiveLocalSolver {
    pub fn new() -> NaiveLocalSolver {
        NaiveLocalSolver {
            vertical: [[5000.0; N]; N - 1],
            horizon: [[5000.0; N - 1]; N],
        }
    }

    fn calc_path(&self, start: (usize, usize), end: (usize, usize)) -> (f64, Vec<Direction>) {
        let mut heap = BinaryHeap::with_capacity(10);
        heap.push(State {
            dist: 0.0,
            pos: start,
            path: Vec::new(),
        });
        let mut visited = [[1e300; N]; N];
        while let Some(state) = heap.pop() {
            if state.pos == end {
                return (state.dist, state.path);
            }
            let (y, x) = state.pos;

            let mut push = |y: usize, x: usize, dist, path: &[Direction], dir| {
                if dist < visited[y][x] {
                    visited[y][x] = dist;
                    let mut path = path.to_vec();
                    path.push(dir);
                    heap.push(State {
                        dist,
                        pos: (y, x),
                        path,
                    });
                }
            };

            // up
            if 0 < y {
                let y = y - 1;
                let dist = state.dist + self.vertical[y][x];
                push(y, x, dist, &state.path, Direction::U);
            }
            // down
            if y + 1 < N {
                let dist = state.dist + self.vertical[y][x];
                push(y + 1, x, dist, &state.path, Direction::D);
            }
            // left
            if 0 < x {
                let x = x - 1;
                let dist = state.dist + self.horizon[y][x];
                push(y, x, dist, &state.path, Direction::L);
            }
            // right
            if x + 1 < N {
                let dist = state.dist + self.horizon[y][x];
                push(y, x + 1, dist, &state.path, Direction::R);
            }
        }
        unreachable!()
    }

    pub fn solve<J: Judge>(&mut self, judge: &mut J) {
        let query = judge.next_query();
        let (dist, path) = self.calc_path((query[0], query[1]), (query[2], query[3]));
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
