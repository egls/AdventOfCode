use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() {
    println!("Advent of code 2022, day 1");

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut calories = 0;
    let mut max = 0;
    let mut vec = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if calories > max {
                max = calories;
                vec.push(calories);
            }
            calories = 0;

        } else {
            calories += line.parse::<i32>().unwrap();
        }

    }

    vec.sort();
    vec.reverse();

    let sum: i32 = vec[0..3].iter().sum();
    println!("max: {}", max);
    println!("sum of 3 biggest: {}", sum);
}
