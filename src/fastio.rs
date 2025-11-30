use std::fmt::Debug;
use std::{
    io::{self, Read},
    str::FromStr,
};

pub fn spar() -> String {
    try_spar().expect("No input param")
}

pub fn try_spar() -> Option<String> {
    let bytes = io::stdin().lock().bytes();
    let param: String = bytes
        .map(|res_b| res_b.expect("Could not convert to u8"))
        .map(char::from)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    if param.is_empty() {
        None
    } else {
        Some(param)
    }
}

pub fn par<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    spar().parse().expect("Failed to parse")
}

pub fn try_par<T: FromStr>() -> Option<T>
where
    <T as FromStr>::Err: Debug,
{
    try_spar().map(|s| s.parse().expect("Failed to parse"))
}

pub fn apar<T: FromStr>(n: usize) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    (0..n).map(|_| spar().parse().expect("Failed to parse")).collect()
}

pub fn ipar() -> i32 {
    par()
}

pub fn iapar(n: usize) -> Vec<i32> {
    apar(n)
}

pub fn upar() -> usize {
    par()
}

pub fn lpar() -> i64 {
    par()
}

pub fn lapar(n: usize) -> Vec<i64> {
    apar(n)
}

pub fn dpar() -> f64 {
    par()
}

pub fn dapar(n: usize) -> Vec<f64> {
    apar(n)
}

pub fn linepar() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Input error");

    line.trim().to_string()
}
