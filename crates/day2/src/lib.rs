mod parser;
mod printer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::GameRun;
    use rayon::prelude::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn day2_part1_test() -> anyhow::Result<()> {
        // TODO: add tracing
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(true)
            .with_level(true)
            .pretty()
            .finish();
        subscriber.init();
        let path = Path::new("src/test.txt").canonicalize()?;
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let par_file = file_string.par_lines();
        let game_vec: Vec<_> = par_file
            .filter_map(|line| parser::game_run(line).ok())
            .filter_map(|(_, game_run)| serde_json::to_string_pretty(&game_run).ok())
            .collect();

        for item in game_vec {
            println!("{item}")
        }
        Ok(())
    }
}
