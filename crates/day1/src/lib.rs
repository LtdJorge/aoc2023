use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;
use tracing::{debug, error, event, info, instrument, trace, warn, Level};
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

/// Not used for now, the problem with the output probably lies in the concatenation of the same digit
#[instrument]
pub fn day1_part2(file_name: &Path) -> anyhow::Result<i32> {
    const SPELLED_NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut file = File::open(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let par_file_content = file_content.as_parallel_string();
    let lines = par_file_content.lines();

    let result: i32 = lines
        .map(|line| {
            let mut resulting_string = line.to_string();
            info!("Working with string: {}", resulting_string);
            loop {
                // Find the spelled out number that appears first
                let replacer_index = match SPELLED_NUMBERS
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &spelling)| {
                        resulting_string.find(spelling).map(|found_at| {
                            event!(
                                Level::TRACE,
                                "Found {} in line {}",
                                spelling,
                                resulting_string
                            );
                            (index, found_at)
                        })
                    })
                    .min_by(|(_, found_at_a), (_, found_at_b)| found_at_a.cmp(found_at_b))
                {
                    None => break,
                    Some((replacer_index, _)) => {
                        event!(
                            Level::TRACE,
                            "Found first appearing number: {}",
                            SPELLED_NUMBERS[replacer_index]
                        );
                        replacer_index
                    }
                };

                // Replace that first spelled out number with the corresponding numeral
                resulting_string = resulting_string.replace(
                    SPELLED_NUMBERS[replacer_index],
                    &(replacer_index + 1).to_string(),
                );
            }

            event!(
                Level::DEBUG,
                "Transformed string \"{}\" into \"{}\"",
                line,
                resulting_string
            );
            resulting_string
        })
        // Discard any non-numeric character
        .map(|line| {
            let only_numbers = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<String>();
            event!(Level::TRACE, "Final numeral string: {}", only_numbers);
            only_numbers
        })
        // Leave only the first and last characters, concatenated and turned into an i32
        .map(|line| {
            if line.chars().count() == 1 {
                let char = line.chars().next().unwrap();
                event!(Level::WARN, "Less than two numbers found: {}", char);
                let string = String::from(char);
                return string.parse::<i32>().unwrap();
            }
            let first = line.chars().next().unwrap();
            let last = line.chars().last().unwrap();
            let string_form = format!("{}{}", first, last);
            event!(
                Level::DEBUG,
                "Found first and last characters: {}",
                string_form
            );
            string_form.parse::<i32>().unwrap()
        })
        .sum();

    event!(Level::INFO, "Returned value: \n{:?}", result);
    Ok(result)
}

/// Creates an array for spelled-out numbers and an array for digit numbers (as [str][std::str])
/// Iterates over the lines in the input file in parallel,
#[instrument]
pub fn day1_part2_2(file_name: &Path) -> anyhow::Result<i32> {
    static SPELLED_NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    static DIGIT_NUMBERS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut file = File::open(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let par_file_content = file_content.as_parallel_string();
    let lines = par_file_content.par_lines();

    let result = lines
        .map(|line| {
            info!("Handling line: {}", line);
            let spelled_iterator = SPELLED_NUMBERS.iter().filter_map(|&number| {
                line.find(number).map(|position| {
                    trace!(
                        "Found position {} for number {} within spelled_iterator",
                        position,
                        number
                    );
                    (
                        position,
                        *DIGIT_NUMBERS.index(
                            SPELLED_NUMBERS
                                .iter()
                                .position(|&spelled_number| spelled_number == number)
                                .unwrap_or_else(|| {
                                    error!(
                                        "Failed finding the position of item {} in {:?}",
                                        number, SPELLED_NUMBERS
                                    );
                                    0
                                }),
                        ),
                    )
                })
            });
            let reverse_spelled_iterator = SPELLED_NUMBERS.iter().filter_map(|&number| {
                line.rfind(number).map(|position| {
                    trace!(
                        "Found position {} for number {} within spelled_iterator",
                        position,
                        number
                    );
                    (
                        position,
                        *DIGIT_NUMBERS.index(
                            SPELLED_NUMBERS
                                .iter()
                                .position(|&spelled_number| spelled_number == number)
                                .unwrap_or_else(|| {
                                    error!(
                                        "Failed finding the position of item {} in {:?}",
                                        number, SPELLED_NUMBERS
                                    );
                                    0
                                }),
                        ),
                    )
                })
            });
            let digit_iterator = DIGIT_NUMBERS.iter().filter_map(|&digit| {
                line.find(digit).map(|position| {
                    trace!(
                        "Found position {} for digit {} within spelled_iterator",
                        position,
                        digit
                    );
                    (position, digit)
                })
            });
            let reverse_digit_iterator = DIGIT_NUMBERS.iter().filter_map(|&digit| {
                line.rfind(digit).map(|position| {
                    trace!(
                        "Found position {} for digit {} within spelled_iterator",
                        position,
                        digit
                    );
                    (position, digit)
                })
            });
            let first = spelled_iterator
                .clone()
                .chain(digit_iterator.clone())
                .min_by_key(|(position, _number)| *position)
                .map(|(_position, number)| number)
                .unwrap_or_else(|| {
                    error!("Encountered None in \"first\"");
                    ""
                });
            if !first.is_empty() {
                debug!("First is set to {}", first);
            }
            if spelled_iterator.count() + digit_iterator.count() == 1 {
                warn!("Sum of findings in both iterators is 1");
                let concat = format!("{first}{first}");
                info!("Setting iteration output to {}", concat);
                return concat.parse::<i32>().unwrap();
            }
            let last = reverse_spelled_iterator
                .chain(reverse_digit_iterator)
                .max_by_key(|(position, _number)| *position)
                .map(|(_position, number)| number)
                .unwrap_or_else(|| {
                    error!("Encountered None in \"last\"");
                    ""
                });
            if !last.is_empty() {
                debug!("Last is set to {}", last);
            }
            let string = format!("{}{}", first, last);
            info!("Setting iteration output to {}", string);
            string.parse::<i32>().unwrap()
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn day1_part1_test() -> anyhow::Result<()> {
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
    fn day1_part1_main() -> anyhow::Result<()> {
        let path = Path::new("src/input.txt").canonicalize()?;
        let result = day1_part1(path.as_path())?;
        println!("{}", result);
        Ok(())
    }

    #[test]
    fn day1_part2_test() -> anyhow::Result<()> {
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(true)
            .with_level(true)
            .with_max_level(Level::INFO)
            .pretty()
            .finish();
        subscriber.init();
        let path = Path::new("src/test2.txt").canonicalize()?;
        let result = day1_part2_2(path.as_path())?;
        println!("{}", result);
        Ok(())
    }

    #[test]
    fn day1_part2_main() -> anyhow::Result<()> {
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(true)
            .with_level(true)
            .with_max_level(Level::TRACE)
            .finish();
        subscriber.init();
        let path = Path::new("src/input.txt").canonicalize()?;
        let result = day1_part2_2(path.as_path())?;
        println!("{}", result);
        Ok(())
    }
}
