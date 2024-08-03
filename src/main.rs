use helpers::*;
use std::collections::*;

type Num = usize;

fn main() {
    init_input!(ii);

    let _n: usize = ii.get_num();
    let (n, k) = ii.get_two::<usize>();
    let (c, d, f) = ii.get_three::<i32>();
    let vec: Vec<Num> = ii.get_vec();
    let line = ii.next_line().unwrap();

    println!("{}", solve(n, k, &vec));
}

fn solve(n: usize, k: usize, vec: &[Num]) -> Num {
    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let (n, k) = (0, 0);
        //let vec: Vec<Num> = vec![1,2,3];
        let input = "1 2 3";
        let vec = input
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<Num>>();
        assert_eq!(1, solve(n, k, &vec));
    }
}

///
/// HELPERS
///
mod helpers {
    use std::io::{self, BufRead};
    use std::str::FromStr;
    #[macro_export]
    macro_rules! init_input {
        ($ii:ident) => {
            let stdin = std::io::stdin();
            let stdin_lock = stdin.lock();
            let mut $ii = In::new(stdin_lock);
        };
    }
    pub struct In<'a> {
        it: io::Lines<io::StdinLock<'a>>,
    }
    impl<'a> In<'a> {
        pub fn new(lock: io::StdinLock<'a>) -> Self {
            Self { it: lock.lines() }
        }
        pub fn get_num<T: FromStr>(&mut self) -> T
        where
            <T as FromStr>::Err: std::fmt::Debug,
        {
            self.it.next().unwrap().unwrap().parse::<T>().unwrap()
        }
        pub fn get_two<T: FromStr + Copy>(&mut self) -> (T, T)
        where
            <T as FromStr>::Err: std::fmt::Debug,
        {
            let vec = self.get_vec::<T>();
            (vec[0], vec[1])
        }
        pub fn get_three<T: FromStr + Copy>(&mut self) -> (T, T, T)
        where
            <T as FromStr>::Err: std::fmt::Debug,
        {
            let vec = self.get_vec::<T>();
            (vec[0], vec[1], vec[2])
        }
        pub fn get_vec<T: FromStr>(&mut self) -> Vec<T>
        where
            <T as FromStr>::Err: std::fmt::Debug,
        {
            let row = self.it.next().unwrap().unwrap();
            row.split_whitespace()
                .map(|n| n.parse::<T>().unwrap())
                .collect::<Vec<_>>()
        }
        pub fn next_line(&mut self) -> Option<String> {
            self.it.next().map(|line| line.unwrap())
        }
    }
}
