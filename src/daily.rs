use std::env;

pub fn daily_input(day: u32) -> String {
    let session = env::var("AOC_SESSION").unwrap();

    let mut content = ureq::get(&format!("https://adventofcode.com/2024/day/{day}/input"))
        .set("Cookie", &format!("session={}", session))
        .call().unwrap()
        .into_string().unwrap();
    content.pop();
    content
}