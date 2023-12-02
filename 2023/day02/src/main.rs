use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // day 01 part 01
    part_one();

    // day 01 part 02
    part_two();
}

fn part_one() {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let mut total_sum = 0;

    //let path = "rsc/test.txt";
    let path = "rsc/input.txt";

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let (game_number, game_data) = line.split_at(line.find(":").unwrap());
        let mut valid = true;
        let sections: Vec<&str> = game_data.split(";").collect();

        for section in sections {
            if valid == false {
                break;
            }

            let cubes: Vec<&str> = section.split(",").collect();

            for cube in cubes {
                let cube_split: Vec<&str> = cube.split(" ").collect();
                let cube_number = cube_split[1];
                let cube_color = cube_split[2];

                match cube_color {
                    "red" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_RED {
                            valid = false;
                        }
                    }
                    "green" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_GREEN {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_BLUE {
                            valid = false;
                        }
                    }
                    _ => println!("No match"),
                }
            } // cubes
        } // sections

        if valid {
            let gamenbr = game_number.split(" ").collect::<Vec<&str>>()[1];
            total_sum += gamenbr.parse::<i32>().unwrap();
        }
    }
    println!("Total Sum: {}", total_sum);
}

fn part_two() {
    let mut red_min = 0;
    let mut blue_min = 0;
    let mut green_min = 0;
    let mut total_sum = 0;

    //let path = "rsc/test.txt";
    let path = "rsc/input.txt";

    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let (_g_number, g_date) = line.split_at(line.find(":").unwrap());
        let sections: Vec<&str> = g_date.split(";").collect();

        for section in sections {
            let cubes: Vec<&str> = section.split(",").collect();

            for cube in cubes {
                let cube_split: Vec<&str> = cube.split(" ").collect();
                let cube_number = cube_split[1];
                let cube_color = cube_split[2];

                match cube_color {
                    "red" => {
                        if cube_number.parse::<u32>().unwrap() > red_min {
                            red_min = cube_number.parse::<u32>().unwrap();
                        }
                    }
                    "green" => {
                        if cube_number.parse::<u32>().unwrap() > green_min {
                            green_min = cube_number.parse::<u32>().unwrap();
                        }
                    }
                    "blue" => {
                        if cube_number.parse::<u32>().unwrap() > blue_min {
                            blue_min = cube_number.parse::<u32>().unwrap();
                        }
                    }
                    _ => println!("No match"),
                }
            } // cubes
        } // section

        total_sum +=  red_min * green_min * blue_min;
        red_min = 0;
        blue_min = 0;
        green_min = 0;
    } // line
    println!("Total Sum: {}", total_sum);
}
