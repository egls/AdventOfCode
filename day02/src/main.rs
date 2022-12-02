use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::vec::Vec;

// A = Rock, B = Paper, C = Scissors
// X = Rock, Y = Paper, Z = Scissors
//
// Scoring:
// Rock = 1, Paper = 2, Scissors = 3
// 0 = lost, 3 = draw, 6 = win
//
// Rules:
// Rock Defeats Scissors
// Scissors Defeat Paper
// Paper Defeats Rock
// Same shape = draw
//
//
fn evaluate_game(op: &str, my: &str) -> i32 {
    match op {
        "A" => {
            match my { 
                "X" => {4}, 
                "Y" => {8},
                "Z" => {3}, 
                _ => {0} 
            }
        },
        "B" => {
            match my {
                "X" => {1}, 
                "Y" => {5}, 
                "Z" => {9},
                _ => {0} 
            }
        },
        "C" => {
            match my { 
                "X" => {7},
                "Y" => {2}, 
                "Z" => {6}, 
                _ => {0} }
        },
        _ => 0,
    }    
}


fn main() {
    println!("Advent of code 2021, day 2!");

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;


    for (_index, line) in reader.lines().enumerate() {
        let line_uw = line.unwrap();
        let mut split = line_uw.split_whitespace();

        let op_shape = split.next().unwrap();
        let my_shape = split.next().unwrap();
        
        let result = evaluate_game(op_shape, my_shape);
        //println!("op: {}, my: {}, result: {}", op_shape, my_shape, result);
      
        sum += result;
        println!("sum: {}", sum);
    }

    println!("final sum: {}", sum);



}
