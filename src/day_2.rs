use crate::utils::read_lines;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Submarine {
    h_pos: u32,
    depth: u32,
    aim: u32,
    commands: Vec<Command>,
}

impl Submarine {
    fn new(commands: Vec<Command>) -> Self {
        Submarine {
            h_pos: 0,
            depth: 0,
            aim: 0,
            commands,
        }
    }

    fn reset(&mut self) {
        self.h_pos = 0;
        self.depth = 0;
        self.aim = 0;
    }

    fn calculate_course(&mut self) {
        for command in &self.commands {
            use Command::*;
            match command {
                Forward(num) => self.h_pos += num,
                Down(num) => self.depth += num,
                Up(num) => self.depth -= num,
            }
        }
    }

    fn calculate_course_with_aim(&mut self) {
        for command in &self.commands {
            use Command::*;
            match command {
                Forward(num) => {
                    self.h_pos += num;
                    if self.aim != 0 {
                        self.depth += self.aim * num;
                    }
                }
                Down(num) => self.aim += num,
                Up(num) => self.aim -= num,
            }
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<Command> {
    let commands: Vec<Command> = lines
        .iter()
        .map(|line| {
            let line: Vec<&str> = line.split(" ").collect();
            let num: u32 = line[1].parse().unwrap();
            let command = match line[0] {
                "forward" => Command::Forward(num),
                "down" => Command::Down(num),
                "up" => Command::Up(num),
                _ => Command::Forward(32), // fix this
            };
            command
        })
        .collect();
    commands
}

pub fn part_1() {
    let lines = read_lines("input/day_2.txt").unwrap();
    let commands = parse_lines(&lines);
    let mut submarine = Submarine::new(commands);
    submarine.calculate_course();
    println!("   Part 1: {}", submarine.h_pos * submarine.depth);
    submarine.reset();
    submarine.calculate_course_with_aim();
    println!("   Part 2: {}", submarine.h_pos * submarine.depth);
}
