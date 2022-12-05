use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    println!("Advent of code 2021, day 5!");
    
    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // extract and process the header containing the stacks
    // write lines into vec until we find character 1

    for (_idx, line) in reader.lines().enumerate() {




    }


}