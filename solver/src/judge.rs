#[cfg(feature = "internal_tester")]
pub type BestJudge = InternalJudge;
#[cfg(not(feature = "internal_tester"))]
pub type BestJudge = OuterJudge;

pub trait Judge {
    fn next_query(&mut self) -> [usize; 4];

    fn answer(&mut self, ans: &str);

    fn path_length(&mut self) -> u32;

    fn score(&self) -> i64 {
        0
    }
}

#[cfg(not(feature = "internal_tester"))]
pub struct OuterJudge;

#[cfg(not(feature = "internal_tester"))]
impl OuterJudge {
    pub fn new(_: &str) -> OuterJudge {
        OuterJudge
    }
}

#[cfg(not(feature = "internal_tester"))]
impl Judge for OuterJudge {
    fn next_query(&mut self) -> [usize; 4] {
        use std::io::stdin;

        let stdin = stdin();
        let mut buf = String::with_capacity(11);
        stdin.read_line(&mut buf).expect("fail to read query");
        let mut words = buf.split_ascii_whitespace();
        [
            words
                .next()
                .expect("no query 0")
                .parse()
                .expect("not int at query 0"),
            words
                .next()
                .expect("no query 1")
                .parse()
                .expect("not int at query 1"),
            words
                .next()
                .expect("no query 2")
                .parse()
                .expect("not int at query 2"),
            words
                .next()
                .expect("no query 3")
                .parse()
                .expect("not int at query 3"),
        ]
    }

    fn answer(&mut self, ans: &str) {
        use std::io::{stdout, Write};

        let mut stdout = stdout();
        writeln!(stdout, "{}", ans).expect("fail to write answer to stdout");
        stdout.flush().expect("fail to flush stdout");
    }

    fn path_length(&mut self) -> u32 {
        use std::io::stdin;

        let stdin = stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("fail to read path length");
        buf.trim().parse().expect("not int at path length")
    }
}

#[cfg(feature = "internal_tester")]
pub struct InternalJudge {
    input: tools::Input,
    outs: Vec<String>,
    visited: Vec<Vec<usize>>,
}

#[cfg(feature = "internal_tester")]
impl InternalJudge {
    pub fn new(input: &str) -> InternalJudge {
        use crate::consts::N;
        use tools::mat;

        InternalJudge {
            input: tools::read_input_str(input),
            outs: Vec::new(),
            visited: mat![!0; N; N],
        }
    }
}

#[cfg(feature = "internal_tester")]
impl Judge for InternalJudge {
    fn next_query(&mut self) -> [usize; 4] {
        let i = self.outs.len();
        let query = [
            self.input.s[i].0,
            self.input.s[i].1,
            self.input.t[i].0,
            self.input.t[i].1,
        ];
        query
    }

    fn answer(&mut self, ans: &str) {
        self.outs.push(ans.to_owned());
    }

    fn path_length(&mut self) -> u32 {
        let k = self.outs.len() - 1;
        tools::compute_path_length(&self.input, k, self.outs.last().unwrap(), &mut self.visited)
            .unwrap()
            .0 as u32
    }

    fn score(&self) -> i64 {
        tools::compute_score_detail(&self.input, &self.outs).0
    }
}
