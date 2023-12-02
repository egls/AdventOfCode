fn main() {
    // day 01 part 01
    part_one();

    // day 01 part 02
    part_two();
}

fn part_two() {

    let input = std::fs::read_to_string("rsc/input.txt").unwrap();
    let mut sum = 0;

    let digits = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let replacement_digits = vec![
        "ze0o", "o1e", "t2o", "thr3e", "fo4r", "fi5e", "s6x", "se7en", "eig8t", "ni9e",
    ];

    for line in input.lines() {
        let mut digits_string = String::new();
        let mut search = line.to_string();

        for d in &digits {
            match search.contains(d) {
                true => {
                    // replace the matched word with the one from replacement_digits
                    let result = search.replace(
                        d,
                        &replacement_digits[digits.iter().position(|&x| x == *d).unwrap()],
                    );
                    search = result;
                }

                false => (),
            }
        }
        for c in search.chars() {
            if c.is_digit(10) {
                digits_string.push(c);
            }
        }

        let sum_string = format!(
            "{}{}",
            digits_string.chars().next().unwrap(),
            digits_string.chars().last().unwrap()
        );
        //println!("calibration value: {}", sum_string);

        sum += sum_string.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum);
}

fn part_one() {
    let input = std::fs::read_to_string("rsc/input.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let mut digits_string = String::new();
        let chars = line.chars();

        // get all digits
        for c in chars {
            if c.is_digit(10) {
                println!("c: {}", c);
                digits_string.push(c);
            }
        }
        println!("digits_string: {}", digits_string);

        // pick the first and last digit and concatenate them
        let sum_string = format!(
            "{}{}",
            digits_string.chars().next().unwrap(),
            digits_string.chars().last().unwrap()
        );
        println!("sum_string: {}", sum_string);

        sum += sum_string.parse::<i32>().unwrap();
    }

    println!("sum: {}", sum);
}
