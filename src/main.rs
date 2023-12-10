// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};

// extract integer from string
// first number in string is multiplied by 10
// last number in string is multiplied by 1
// both numbers are added and returned
fn extract_int_part_1(line: &str) -> i64 {
    let mut num_1: i64 = 0;
    let mut num_2: i64 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            num_1 = c.to_digit(10).unwrap() as i64 * 10;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            num_2 = c.to_digit(10).unwrap() as i64;
            break;
        }
    }
    num_1 + num_2
}

// replace string numbers with their numeric value
fn replace_string_numbers(line: &str) -> String {
    let mut new_line = line.to_string();
    new_line = new_line.replace("one", "o1e");
    new_line = new_line.replace("two", "t2o");
    new_line = new_line.replace("three", "3");
    new_line = new_line.replace("four", "4");
    new_line = new_line.replace("five", "5");
    new_line = new_line.replace("six", "6");
    new_line = new_line.replace("seven", "7");
    new_line = new_line.replace("eight", "e8t");
    new_line = new_line.replace("nine", "9");

    new_line
}

// extract integer from string
// first number in string is multiplied by 10
// last number in string is multiplied by 1
// both numbers are added and returned
fn extract_int_part_2(line: &str) -> i64 {
    let mut num_1: i64 = 0;
    let mut num_2: i64 = 0;
    println!("{}", line);

    let new_line = replace_string_numbers(line);
    let new_line_str = new_line.as_str();
    println!("{}", new_line_str);

    for c in new_line_str.chars() {
        if c.is_digit(10) {
            num_1 = c.to_digit(10).unwrap() as i64 * 10;
            break;
        }
    }
    for c in new_line_str.chars().rev() {
        if c.is_digit(10) {
            num_2 = c.to_digit(10).unwrap() as i64;
            break;
        }
    }
    num_1 + num_2
}


fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    // create vec of int64
    let mut int_vec: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // int_vec.push(extract_int_part_1(&line));
        int_vec.push(extract_int_part_2(&line));
    }

    // sum vec and print
    let sum: i64 = int_vec.iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}