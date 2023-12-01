use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{error, event, Level};

#[tracing::instrument]
pub fn day1_fun(file_name: &Path) -> anyhow::Result<i32> {
    let file = File::open(file_name)?;
    let file_reader = BufReader::new(file);

    let val = file_reader
        .lines()
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

    event!(Level::INFO, "Returned value: \n{:?}", val);
    Ok(val)
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
        day1_fun(path.as_path())?;
        Ok(())
    }

    #[test]
    fn day1_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let result = day1_fun(path.as_path())?;
        println!("{}", result);
        Ok(())
    }
}
