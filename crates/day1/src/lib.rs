use std::path::Path;
use tracing::{error, event, instrument, Level};
use util::path_to_lines;

#[instrument]
pub fn day1_part1(file_name: &Path) -> anyhow::Result<i32> {
    let lines = path_to_lines(file_name)?;

    let result = lines
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_string().parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|line| {
            let first = line
                .first()
                .ok_or_else(|| error!("Error getting first value of \"line\" {:?}", line))
                .unwrap();
            let last = line
                .last()
                .ok_or_else(|| error!("Error getting last value of \"line\" {:?}", line))
                .unwrap();
            format!("{}{}", first, last)
        })
        .filter_map(|string| string.parse::<i32>().ok())
        .sum();

    event!(Level::INFO, "Returned value: \n{:?}", result);
    Ok(result)
}

#[instrument]
pub fn day1_part2(file_name: &Path) -> anyhow::Result<i32> {
    let lines = path_to_lines(file_name)?;

    Ok(42)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn day1_test() -> anyhow::Result<()> {
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(true)
            .with_level(true)
            .pretty()
            .finish();
        subscriber.init();
        let path = Path::new("src/test.txt").canonicalize()?;
        day1_part1(path.as_path())?;
        Ok(())
    }

    #[test]
    fn day1_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let result = day1_part1(path.as_path())?;
        println!("{}", result);
        Ok(())
    }
}
