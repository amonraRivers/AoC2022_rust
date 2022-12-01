use std::fs;

pub fn parse(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path);
    let mut y: Vec<String> = vec![];
    match contents {
        Ok(x) => y = x.lines().map(|z| z.to_owned()).collect(),
        Err(x) => {
            println!("{}", x)
        }
    }
    y
}
