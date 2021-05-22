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
