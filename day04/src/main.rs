use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    println!("Advent of code 2021, day 4!");

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut nbr_of_pairs = 0;
    for (_idx, line) in reader.lines().enumerate() {
        //println!("{:?}", line);

        // split string on ,
        let uw = line.unwrap();
        let tokens: Vec<&str> = uw.split(",").collect();
        //println!("token 1: {}, token 2: {}", tokens[0], tokens[1]);
        let range1: Vec<&str> = tokens[0].split("-").collect();
        let range1_low = range1[0].parse::<i32>().unwrap();
        let range1_high = range1[1].parse::<i32>().unwrap();

        let range2: Vec<&str> = tokens[1].split("-").collect();
        let range2_low = range2[0].parse::<i32>().unwrap();
        let range2_high = range2[1].parse::<i32>().unwrap();

        if range1_low <= range2_low && range1_high >= range2_high {
            println!("case 1 -> 1: {}, 2: {}", tokens[0], tokens[1]);
            nbr_of_pairs+= 1;
        } else  if range1_low >= range2_low && range1_high <= range2_high {
            println!("case 2 -> 1: {}, 2: {}", tokens[0], tokens[1]);
            nbr_of_pairs+=1;
        }
    }
    println!("nbr of pairs: {}", nbr_of_pairs);
}


