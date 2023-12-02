use std::fs;

pub fn task_one() {
    let text = fs::read_to_string("inputs/1.txt").expect("Not able to read the file");
    let mut calibration_values: Vec<i32> = vec![];

    for line in text.lines() {
        let digits: Vec<_> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        let calibration_value = (format!("{}{}", digits.first().unwrap(), digits.last().unwrap())).parse::<i32>().unwrap();
        calibration_values.push(calibration_value)
    }

    let sum: i32 = calibration_values.iter().sum();
    println!("{}", sum);
}

pub fn task_two() {
    let text = fs::read_to_string("inputs/1.txt").expect("Not able to read the file");
    let mut calibration_values: Vec<i32> = vec![];
    let valid_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in text.lines() {
        let mut digits = line.chars().enumerate().filter_map(|(index, c)| {
            if let Some(digit) = c.to_digit(10) {
                Some(digit)
            } else {
                let substring = &line[index..];
                valid_nums
                    .iter()
                    .enumerate()
                    .filter_map(|(digit, digit_str)| {
                        substring.starts_with(digit_str).then_some(digit as u32 + 1)
                    })
                    .next()
            }
        });
        let first = digits.next().expect("No first digit");
        let last = digits.last().unwrap_or(first);
        let calibration_value = (format!("{}{}", first, last)).parse::<i32>().unwrap();
        calibration_values.push(calibration_value);
    };

    let sum: i32 = calibration_values.iter().sum();
    println!("{}", sum);
}