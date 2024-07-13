use helpers::*;
use std::collections::*;

type Num = usize;

fn main() {
    init_input!(ii);

    let _n: usize = ii.get_num();
    //let vec: Vec<Num> = ii.get_vec();
    //let (a, b) = ii.get_two::<usize>();
    //let (c, d, f) = ii.get_three::<i32>();
}

///
/// HELPERS
///
#[allow(dead_code)]
mod helpers {
    use std::io::{self, BufRead};
    use std::str::FromStr;
    pub fn vec_fmt<T: FromStr + std::fmt::Display>(vec: Vec<T>) -> String {
        vec.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub fn p_vec<T: FromStr + std::fmt::Display>(vec: Vec<T>) {
        println!("{}", vec_fmt(vec));
    }
    pub fn p_empty<T: FromStr + std::fmt::Display>(vec: Vec<T>) {
        if vec.is_empty() {
            println!("-1");
        } else {
            p_vec(vec);
        }
    }
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
