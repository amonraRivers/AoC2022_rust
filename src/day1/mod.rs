use crate::utils::parser::parse;

pub mod p1;
pub mod p2;

#[cfg(test)]
pub mod tests;

pub fn solution() {
    let contents = parse("../2022/day1.txt");

    let parsed_contents = solution_parse(contents);
    let r1 = p1::problem1(&parsed_contents);

    let r2 = p2::problem2(&parsed_contents);

    println!("{} {}", r1, r2);
}

fn solution_parse(contents: Vec<String>) -> Vec<i32> {
    contents
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
