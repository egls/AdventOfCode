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

    //let path = "rsc/test.txt"; // 13
    let path = "rsc/input.txt"; // 20869 too high

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let (card_number, data) = line.trim().split_at(line.find(':').unwrap());
        let (win_data, game_data) = data.trim().split_at(data.find('|').unwrap());

        println!("...................................");
        println!("card_number: {}", card_number);
        println!("win_data: {}", win_data);
        println!("game_data: {}", game_data);

        let win_vec = win_data.trim().split(' ').collect::<Vec<&str>>();
        let game_vec = game_data.trim().split(' ').collect::<Vec<&str>>();

        for i in 0..win_vec.len() {
            let cmp_win: Result<i32, _> = win_vec[i].parse();

            if cmp_win.is_ok() {
                //println!("cmp: {:?}", cmp_win);

                for j in 0..game_vec.len() {
                    let cmp2: Result<i32, _> = game_vec[j].parse();

                    if cmp2.is_ok() {
                        //println!("cmp2: {:?}", cmp2);
                        if cmp_win == cmp2 {
                            counter += 1;
                        }
                    } else {
                        //println!("cmp2: {:?}", cmp2);
                    }
                }
            } else {
                //println!("cmp bad: {:?}", cmp_win);
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
