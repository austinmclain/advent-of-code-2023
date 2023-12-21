use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/*
 * Open the data file and read each line.
 * Get the first and last integer that appears in each line.
 * Combine them and add to sum variable.
 * After reading each line, print the sum.
 */
fn main() {
    let mut digits = HashMap::new();
    digits.insert(String::from("one"), 1);
    digits.insert(String::from("two"), 2);
    digits.insert(String::from("three"), 3);
    digits.insert(String::from("four"), 4);
    digits.insert(String::from("five"), 5);
    digits.insert(String::from("six"), 6);
    digits.insert(String::from("seven"), 7);
    digits.insert(String::from("eight"), 8);
    digits.insert(String::from("nine"), 9);

    let filename = "data/data.txt";

    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(text) = line {
                // Gets the first integer
                for i in 0..text.len() {
                    // Checks if current character is an integer
                    let character = text.chars().nth(i).unwrap();
                    if character.is_numeric() {
                        let num = character.to_digit(10).unwrap();
                        sum += num * 10;
                        break;
                    }
                    // Checks if current window contains integer spelled out with letters
                    let substring = &text[0..i+1];
                    let found_key = digits.keys().find(|&key| substring.contains(key));
                    match found_key {
                        Some(key) => {
                            let num = digits.get(key).unwrap();
                            sum += num * 10;
                            break;
                        }
                        None => {
                        }
                    }
                }
                // Gets the last integer
                for i in (0..text.len()).rev() {
                    // Checks if current character is an integer
                    let character = text.chars().nth(i).unwrap();
                    if character.is_numeric() {
                        let num = character.to_digit(10).unwrap();
                        sum += num;
                        break;
                    }
                    // Checks if current window contains integer spelled out with letters
                    let substring = &text[i..text.len()];
                    let found_key = digits.keys().find(|&key| substring.contains(key));
                    match found_key {
                        Some(key) => {
                            let num = digits.get(key).unwrap();
                            sum += num;
                            break;
                        }
                        None => {
                        }
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

