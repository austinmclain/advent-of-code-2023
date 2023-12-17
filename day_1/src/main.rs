use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
 * Open the data file and read each line.
 * Get the first and last integer that appears in each line.
 * Combine them and add to sum variable.
 * After reading each line, print the sum.
 */
fn main() {
    let filename = "data/data.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(text) = line {
                for character in text.chars() {
                    if character.is_numeric() {
                        let num = character.to_digit(10).unwrap();
                        sum += num * 10;
                        break;
                    }
                }
                for character in text.chars().rev() {
                    if character.is_numeric() {
                        let num = character.to_digit(10).unwrap();
                        sum += num;
                        break;
                    }
                }
            }
        }
        println!("{}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

