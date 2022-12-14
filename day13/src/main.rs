use std::fs::File;
use std::io::{BufRead,BufReader};

struct Values {
    first: String,
    second: String,
}

fn main() {
    println!("Advent of code 2021, day 13!");
  
    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut new_comp = false;
    let mut values = Values{first: String::from(""), second: String::from("")};
    for (_idx, iter) in reader.lines().enumerate() {
        
        
            let line = iter.unwrap();

            if line.is_empty() {

                values.first = line;
                
                new_comp = true;
                
                // start a new comparision


            }
        
        
    }
}
