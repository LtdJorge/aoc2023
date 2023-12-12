mod data;
mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::{ParallelIterator, ParallelString};
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn day4_part1_test() -> anyhow::Result<()> {
        let path = Path::new("src/test.txt").canonicalize()?;
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let par_file = file_string.par_lines();
        let cards: Vec<_> = par_file
            .filter_map(|line| parser::card(line).ok())
            .collect();
        let sum: i32 = cards.iter().map(|(_, card)| card.calculate_points()).sum();
        println!("Sum is {sum}");
        Ok(())
    }

    #[test]
    fn day4_part1_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let par_file = file_string.par_lines();
        let cards: Vec<_> = par_file
            .filter_map(|line| parser::card(line).ok())
            .collect();
        let sum: i32 = cards.iter().map(|(_, card)| card.calculate_points()).sum();
        println!("{sum}");
        Ok(())
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
