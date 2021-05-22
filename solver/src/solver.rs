use crate::judge::Judge;

pub struct NaiveManhattan;

impl NaiveManhattan {
    pub fn solve<J: Judge>(&self, judge: &mut J) {
        eprintln!("start");
        let query = judge.next_query();
        eprintln!("{:?}", query);
        let mut ans = String::new();
        if query[0] < query[2] {
            for _ in query[0]..query[2] {
                ans.push('D');
            }
        } else {
            for _ in query[2]..query[0] {
                ans.push('U');
            }
        }
        if query[1] < query[3] {
            for _ in query[1]..query[3] {
                ans.push('R');
            }
        } else {
            for _ in query[3]..query[1] {
                ans.push('L');
            }
        }
        judge.answer(&ans);
        eprintln!("send");
        let _ = judge.path_length();
        eprintln!("path_length");
    }
}
