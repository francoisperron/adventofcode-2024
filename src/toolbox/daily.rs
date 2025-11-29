use std::env;

pub fn daily_input(day: u32) -> String {
    match get_daily_input(day) {
        Ok(input) => input,
        Err(err) => panic!("Failed to get daily input: {}", err),
    }
}

fn get_daily_input(day: u32) -> Result<String, Box<dyn std::error::Error>> {
    let session = env::var("AOC_SESSION")?;

    let mut response = ureq::get(&format!("https://adventofcode.com/2024/day/{}/input", day))
        .header("Cookie", &format!("session={}", session))
        .call()?;

    let mut content = response.body_mut().read_to_string()?;
    content.pop();

    Ok(content)
}
