use crate::utils::parser::parse;
use itertools::Itertools;

pub fn problem2(contents: &Vec<i32>) -> i32 {
    let res = contents
        .iter()
        .combinations(3)
        .filter(|x| x[0] + x[1] + x[2] == 2020)
        .map(|x| x[0] * x[1] * x[2])
        .collect::<Vec<i32>>();
    res[0]
}
