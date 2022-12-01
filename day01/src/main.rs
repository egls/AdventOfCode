use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    println!("Advent of code 2022, day 1");


    // read in input from file
    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut calories = 0;

    let mut max = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            // println!("{}", "empty");
            println!("{}", calories);
            // create a new Elve
            if calories > max {
                max = calories;
            }

            calories = 0;

        } else {
            calories += line.parse::<i32>().unwrap();
        }

        //println!("{}. {}", index+1, line);
    }

    println!("max: {}", max);
}
