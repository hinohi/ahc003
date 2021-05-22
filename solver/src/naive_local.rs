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

    fn calc_path(&self, start: (usize, usize), end: (usize, usize)) -> Vec<Direction> {
        let mut heap = BinaryHeap::with_capacity(10);
        heap.push(State {
            dist: 0.0,
            pos: start,
            path: Vec::new(),
        });
        let mut visited = [[1e100; N]; N];
        while let Some(state) = heap.pop() {
            if state.pos == end {
                return state.path;
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
        let path = self.calc_path((query[0], query[1]), (query[2], query[3]));
        judge.answer(&dir_to_str(&path));
        let length = judge.path_length() as f64;
        let avg = length / path.len() as f64;
        let mut y = query[0];
        let mut x = query[1];
        for d in path {
            match d {
                Direction::U => {
                    y -= 1;
                    self.vertical[y][x] = (self.vertical[y][x] + avg) * 0.5;
                }
                Direction::D => {
                    self.vertical[y][x] = (self.vertical[y][x] + avg) * 0.5;
                    y += 1;
                }
                Direction::L => {
                    x -= 1;
                    self.horizon[y][x] = (self.horizon[y][x] + avg) * 0.5;
                }
                Direction::R => {
                    self.horizon[y][x] = (self.horizon[y][x] + avg) * 0.5;
                    x += 1;
                }
            }
        }
    }
}
