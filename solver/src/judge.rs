use std::io::{stdin, stdout, Write};

pub trait Judge {
    fn next_query(&mut self) -> [usize; 4];

    fn answer(&mut self, ans: &str);

    fn path_length(&mut self) -> u32;
}

pub struct OuterJudge;

impl Judge for OuterJudge {
    fn next_query(&mut self) -> [usize; 4] {
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
        let mut stdout = stdout();
        writeln!(stdout, "{}", ans).expect("fail to write answer to stdout");
        stdout.flush().expect("fail to flush stdout");
    }

    fn path_length(&mut self) -> u32 {
        let stdin = stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("fail to read path length");
        buf.trim().parse().expect("not int at path length")
    }
}
