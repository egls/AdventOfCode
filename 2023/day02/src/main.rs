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
    const MAX_GREEN: u32 =  13;
    const MAX_BLUE: u32  = 14;

    //let mut sum_red = 0;
    //let mut sum_green = 0;
    //let mut sum_blue = 0;
    let mut total_sum = 0;

    //let path = "rsc/test.txt";
    let path = "rsc/input.txt";
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        //  get game number, separate the string via : first string is the game number, second string is the game data
        let line = line.unwrap();
        
        // refactor with let(game_number, game)
        let split: Vec<&str> = line.split(":").collect();
        let game_number = split[0];
        let game_data = split[1];

        let mut valid = true;

        println!("-----------------------------------");
        println!("Game Number: {}", game_number);
        println!("Game Data: {}", game_data);

        // split into sections (;)
        let sections: Vec<&str> = game_data.split(";").collect();

        for section in sections {
            
            if valid == false {
                break;
            }
            
            let cubes: Vec<&str> = section.split(",").collect();

            for cube in cubes {
                //println!("Cube: {}", cube.trim());

                // split cube, first string is the number of cubes, second is the string 
                let cube_split: Vec<&str> = cube.split(" ").collect();
                let cube_number = cube_split[1];
                let cube_color = cube_split[2];

                match cube_color {
                    "red" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_RED {
                            valid = false;
                        }
                    },
                    "green" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_GREEN {
                            valid = false;
                        }
                    },
                    "blue" => {
                        if cube_number.parse::<u32>().unwrap() > MAX_BLUE {
                            valid = false;
                        }
                    },
                    _ => println!("No match"),
                }


                //println!("Cube Number: {}", cube_number);
                //println!("Cube Color: {}", cube_color);
                //match cube_color {
                //    "red" => sum_red += cube_number.parse::<i32>().unwrap(),
                //    "green" => sum_green += cube_number.parse::<i32>().unwrap(),
                //    "blue" => sum_blue += cube_number.parse::<i32>().unwrap(),
                //    _ => println!("No match"),
                //}
            } // cubes
        } // sections
        
        if valid {
            let gamenbr = game_number.split(" ").collect::<Vec<&str>>()[1];
            total_sum += gamenbr.parse::<i32>().unwrap();
            println!("Total Sum: {}", total_sum);
        }
        //println!("Sum Red: {}", sum_red);
        //println!("Sum Green: {}", sum_green);
        //println!("Sum Blue: {}", sum_blue);
        //if sum_red <= red_cubes && sum_green <= green_cubes && sum_blue <= blue_cubes {
        //    let gamenbr = game_number.split(" ").collect::<Vec<&str>>()[1];
        //    println!("Game Number: {}", gamenbr);
        //    total_sum += gamenbr.parse::<i32>().unwrap();
        //    println!("Total Sum: {}", total_sum);
        //} else {
        //    println!("No match");
        // }
        //println!("-----------------------------------");
        // println!("{}", line);
        //sum_red = 0;
        //sum_blue = 0;
        //sum_green = 0;
    }
    println!("Total Sum: {}", total_sum);
}

fn part_two() {}
