use std::fs::File;
use std::io::{BufRead, BufReader};

fn evaluate_game_2(op: &str, my: &str) -> i32 {
    match op {
        "A" => {
            match my { 
                "X" => {3}, 
                "Y" => {4},
                "Z" => {8}, 
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
                "X" => {2},
                "Y" => {6}, 
                "Z" => {7}, 
                _ => {0} }
        },
        _ => 0,
    }    
}

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

    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line_uw = line.unwrap();
        let mut split = line_uw.split_whitespace();

        let op_shape = split.next().unwrap();
        let my_shape = split.next().unwrap();
        
        let result_1 = evaluate_game(op_shape, my_shape);
        let result_2 = evaluate_game_2(op_shape, my_shape);
              
        sum_1 += result_1;
        sum_2 += result_2;
    }

    println!("final sum 1: {}", sum_1);
    println!("final sum 2: {}", sum_2);

}
