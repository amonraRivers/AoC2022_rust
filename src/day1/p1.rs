use crate::utils::parser::parse;
use itertools::Itertools;

pub fn problem1(contents: &Vec<i32>) -> i32 {
    let res = contents
        .iter()
        .combinations(2)
        .filter(|x| x[0] + x[1] == 2020)
        .map(|x| x[0] * x[1])
        .collect::<Vec<i32>>();
    res[0]
}
