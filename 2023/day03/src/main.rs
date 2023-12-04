use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // day 01 part 01
    part_one();

    // day 01 part 02
    //part_two();
}

fn part_one() {
    let mut counter = 0;
    let mut total_sum = 0;

    // let path = "rsc/test.txt";
    let path = "rsc/input.txt"; // 20869 too high

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let (card_number, data) = line.trim().split_at(line.find(":").unwrap());
        let (win_data, game_data) = data.trim().split_at(data.find("|").unwrap());

        println!("...................................");
        println!("card_number: {}", card_number);
        println!("win_data: {}", win_data);
        println!("game_data: {}", game_data);

        let sub = win_data.split(" ").collect::<Vec<&str>>();
        for i in 1..sub.len() - 1 {
            let digit = sub[i].to_string();
            if (digit.is_empty()) {
                println!("digit is empty");
            } else {
                if game_data.contains(digit.as_str()) {
                    println!("game_data contains: {}", sub[i]);
                    counter += 1;
                }
            }
        }

        println!("counter: {}", counter);

        if counter > 0 {
            let result = i32::pow(2, counter - 1);
            println!("result: {}", result);
            counter = 0;
            total_sum += result;
        }
    }
    println!("total_sum: {}", total_sum);
}
