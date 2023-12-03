use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    testfile: Option<PathBuf>,
}
fn main() {
    let cli = Cli::parse();
    if let Some(input_file_path) = cli.testfile {
        let calibration_doc = CalibrationDocument {
            path: input_file_path,
        };
        let calibration_content = calibration_doc
            .read()
            .expect("Expected content in the file");
        let mut sum: i32 = 0;
        for line in calibration_content.lines() {
            let parse = parse_line(line);
            println!("line: {line}, parse: {parse}");
            sum = sum + parse;
        }
        println!("FINAL RESULT: {sum}");
    } else {
        println!("No test file provided, check help");
        exit(1)
    }
}

#[derive(Debug)]
struct CalibrationDocument {
    path: PathBuf,
}

impl CalibrationDocument {
    fn read(&self) -> Result<String, io::Error> {
        let content = fs::read_to_string(&self.path);

        content
    }
}

fn parse_line(line: &str) -> i32 {
    let start: char = find_digit(line, true);
    let end: char = find_digit(line, false);
    let mut res = String::from("");
    res.push(start);
    res.push(end);
    let res = res.parse::<i32>().unwrap();
    res
}

fn find_digit(line: &str, start: bool) -> char {
    let mut key: Option<&str> = None;
    let mut bound: usize = line.len();
    let numbers = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("0", '0'),
    ]);
    if start {
        for num in numbers.keys().into_iter() {
            let idx = line.find(num).unwrap_or(bound);
            if bound > idx {
                key = Some(num);
                bound = idx;
            }
        }
    } else {
        for num in numbers.keys().into_iter() {
            let rev_line: String = line.chars().rev().collect();
            let rev_num: String = num.chars().rev().collect();
            let idx = rev_line.as_str().find(rev_num.as_str()).unwrap_or(bound);
            if bound > idx {
                key = Some(num);
                bound = idx;
            }
        }
    };
    let res = numbers.get(key.unwrap()).unwrap().to_owned();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parsing() {
        let res = parse_line("46threevqs8114");
        assert_eq!(res, 44);
        let res = parse_line("threetwoonez1gtrd");
        assert_eq!(res, 31);
        let res = parse_line("a1b2c3d");
        assert_eq!(res, 13);
        let res = parse_line("two1nine");
        assert_eq!(res, 29);
        let res = parse_line("eightwothree");
        assert_eq!(res, 83);
        let res = parse_line("abcone2threexyz");
        assert_eq!(res, 13);
        let res = parse_line("xtwone3four");
        assert_eq!(res, 24);
        let res = parse_line("4nineeightseven2");
        assert_eq!(res, 42);
        let res = parse_line("zoneight234");
        assert_eq!(res, 14);
        let res = parse_line("7pqrstsixteen}");
        assert_eq!(res, 76);
    }
}
