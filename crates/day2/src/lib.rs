mod checker;
mod parser;
mod printer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::PlayedSet;
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
        let game_list: Vec<_> = par_file
            .filter_map(|line| parser::game_run(line).ok())
            .map(|(_, game)| game)
            .collect();

        let truth = PlayedSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let sum: i32 = game_list
            .iter()
            .filter_map(|game| game.clone().verify_run(&truth))
            .sum();

        println!("Sum {sum}");
        /*let game_vec: Vec<_> = par_file
            .filter_map(|line| parser::game_run(line).ok())
            .filter_map(|(_, game_run)| serde_json::to_string_pretty(&game_run).ok())
            .collect();

        for item in game_vec {
            println!("{item}")
        }*/
        Ok(())
    }

    #[test]
    fn day2_part1_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let par_file = file_string.par_lines();
        let game_list: Vec<_> = par_file
            .filter_map(|line| parser::game_run(line).ok())
            .map(|(_, game)| game)
            .collect();

        let truth = PlayedSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let sum: i32 = game_list
            .iter()
            .filter_map(|game| game.clone().verify_run(&truth))
            .sum();

        println!("Sum {sum}");

        Ok(())
    }

    #[test]
    fn day2_part2_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let par_file = file_string.par_lines();
        let game_list: Vec<_> = par_file
            .filter_map(|line| parser::game_run(line).ok())
            .map(|(_, game)| game)
            .collect();

        let sum_of_powers: i32 = game_list
            .iter()
            .filter_map(|game| game.clone().get_run_minimum_needed().ok())
            .map(|set| set.calculate_power())
            .sum();

        println!("Sum of powers: {sum_of_powers}");

        Ok(())
    }
}
