use std::fs;

const LIMIT_SET: Set = Set {
    red: 12,
    green: 13,
    blue: 14
};

pub fn task_one() {
    let text = fs::read_to_string("inputs/2.txt").expect("Not able to read the file");
    let mut sum: i32 = 0;

    for line in text.lines() {
        let game = parse_game(String::from(line));
        if game.1.is_possible() {
            sum += game.0;
        }
    }

    println!("{sum}");
}

pub fn task_two() {
    let text = fs::read_to_string("inputs/2.txt").expect("Not able to read the file");
    let mut sum: i32 = 0;

    for line in text.lines() {
        let game = parse_game(String::from(line));
        sum += game.1.get_power();
    }

    println!("{sum}");
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}
impl Set {
    fn new() -> Self {
        Self {
            red: 0, green: 0, blue: 0
        }
    }

    fn parse(&mut self, text: &str) {
        text
            .split(", ")
            .for_each(|d| {
                let (count_str, color) = d.split_once(" ").unwrap();
                let count = count_str.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        if self.red < count {
                            self.red = count;
                        }
                    }
                    "green" => {
                        if self.green < count {
                            self.green = count;
                        }
                    }
                    "blue" => {
                        if self.blue < count {
                            self.blue = count;
                        }
                    }
                    _ => {}
                }
            })
    }

    fn is_possible(&self) -> bool {
        self.red <= LIMIT_SET.red && self.green <= LIMIT_SET.green && self.blue <= LIMIT_SET.blue
    }

    fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn parse_game(line: String) -> (i32, Set) {
    let test = line.clone();
    let id = test.replace("Game ", "").split_once(" ").unwrap().0.replace(":", "").parse::<i32>().unwrap();
    let mut set: Set = Set::new();

    line
        .split_once(": ")
        .unwrap()
        .1
        .split("; ")
        .for_each(|set_str| {
            set.parse(set_str);
        });

    (id, set)
}