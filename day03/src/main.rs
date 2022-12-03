use std::fs::File;
use std::io::{BufRead, BufReader};


fn problem_1() {
    let lookup = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (_index, line) in reader.lines().enumerate() {
        // split string in half (substring)
        let str = line.unwrap();
        let len = str.len() / 2;
        let subs = str.split_at(len);

        //println!("1: {}, 2: {}",subs.0,subs.1);
        // search through string for same character
        for c in subs.0.chars() {

            if subs.1.contains(c){
                //println!("subs.1 contains: {}", c);
                let cl = c.to_lowercase();
                let mut index = lookup.iter().position(|&elem| elem == cl.to_string() ).unwrap() + 1;
                //print!(" index: {}\n", index);
                if c.is_uppercase() {
                    index += 26;
                }
                sum += index;
                break;
            }

        }
    }
     println!("sum: {}", sum);
}

fn main() {
    println!("Advent of code 2021, day 3!");
    problem_1();

    let lookup = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];

    let filename = "rsc/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut group: Vec<String> = Vec::with_capacity(3);
    let mut ctr = 1;
    for (idx, line) in reader.lines().enumerate() {
   
        if ctr!=0 && ctr%3==0 {
            group.push(line.unwrap());
            // process
            // println!("idx: {}, 1: {}, 2: {}, 3: {}", ctr, group[0].as_str(), group[1].as_str(), group[2].as_str());
            
            for c in group[0].chars(){
                if group[1].contains(c) && group[2].contains(c){
                    let cl = c.to_lowercase();
                    let mut index = lookup.iter().position(|&elem| elem == cl.to_string() ).unwrap() + 1;
                    print!("char: {}, index: {}\n", c, index);
                    if c.is_uppercase() {
                        index += 26;
                    }
                    sum += index;
                    break;
                }

            }
            
            println!("sum group: {}", sum);
            group.clear();            
        } else {
           
            group.push(line.unwrap());
        }
        ctr+=1;
    }
    println!("sum: {}", sum);
}
