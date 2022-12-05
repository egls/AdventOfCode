use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;



fn main() {
    println!("Advent of code 2021, day 5!");
    let mut configuration: Vec<Vec<char>> = Vec::new();
    let nbr = 9;
    for _ in 0..nbr {
          configuration.push(vec![]);
    }

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut reversed = false;
    

    for(_idx, iter) in reader.lines().enumerate() {
        let line = iter.unwrap();
        if line.contains("move"){

            //reverse configuration vector
            if !reversed {
                for i in 0..nbr {
                    configuration[i].reverse();
                }
                println!("initial configuration: {:?}", configuration);
                reversed = true;
            }

            // start moving around
            // extract numbers: 1: nbr of items, 2: src, 3: destination
            let re = Regex::new(r"\d+").unwrap();
            let mut vec: Vec<&str> = Vec::new();
            for caps in re.captures_iter(&line) {
                //println!("caps: {}", caps.get(0).map_or("", |m| m.as_str()));
                vec.push(caps.get(0).map_or("", |m| m.as_str() ));
            }       
            println!("line_ {}",_idx);
            
            // apply moves 
            let nbr_of_ops: usize = vec[0].parse().unwrap();
            for n in 0..nbr_of_ops {
                let src: usize = vec[1].parse().unwrap();
                let dst: usize = vec[2].parse().unwrap();
                let pop = configuration[src-1].pop();
                configuration[dst-1].push(pop.unwrap());
             }
             println!("altered config: {:?}", configuration);
        }else if line.contains("["){
            // write stacks
            let mut offset = 1;
            for i in 0..nbr {
                let c = line.chars().nth(offset).unwrap();
                if !c.is_whitespace() {
                    println!("push char: {} to bin: {}", c,i);
                    configuration[i].push(c);
                }
                offset +=4;
            }
        }
    }

    

}



