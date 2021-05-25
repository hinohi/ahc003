#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    U,
    D,
    R,
    L,
}

pub fn dir_to_str(dir: &[Direction]) -> String {
    let mut s = String::with_capacity(dir.len());
    for d in dir {
        let c = match *d {
            Direction::U => 'U',
            Direction::D => 'D',
            Direction::R => 'R',
            Direction::L => 'L',
        };
        s.push(c);
    }
    s
}

#[derive(Debug)]
pub struct State {
    pub dist: f64,
    pub pos: (usize, usize),
    pub path: Vec<Direction>,
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

pub trait EdgeCost {
    fn vertical(&self, y: usize, x: usize) -> f64;

    fn horizon(&self, y: usize, x: usize) -> f64;
}

pub fn calc_path<T: EdgeCost>(
    cost: &T,
    start: (usize, usize),
    end: (usize, usize),
) -> (f64, Vec<Direction>) {
    use crate::consts::N;
    use std::collections::BinaryHeap;

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
            let dist = state.dist + cost.vertical(y, x);
            push(y, x, dist, &state.path, Direction::U);
        }
        // down
        if y + 1 < N {
            let dist = state.dist + cost.vertical(y, x);
            push(y + 1, x, dist, &state.path, Direction::D);
        }
        // left
        if 0 < x {
            let x = x - 1;
            let dist = state.dist + cost.horizon(y, x);
            push(y, x, dist, &state.path, Direction::L);
        }
        // right
        if x + 1 < N {
            let dist = state.dist + cost.horizon(y, x);
            push(y, x + 1, dist, &state.path, Direction::R);
        }
    }
    unreachable!()
}
