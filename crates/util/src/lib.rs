use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn path_to_lines(file_name: &Path) -> anyhow::Result<Lines<BufReader<File>>> {
    let file = File::open(file_name)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
