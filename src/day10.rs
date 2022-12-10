use std::str::FromStr;

pub enum Instruction {
    Noop,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(4) {
            ("noop", "") => Ok(Self::Noop),
            ("addx", arg) => Ok(Self::AddX(
                arg.trim()
                    .parse()
                    .map_err(|_| format!("couldn't parse number: {arg}"))?,
            )),
            _ => Err(format!("invalid instruction: {s}")),
        }
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct Cpu<'a> {
    pub x: isize,
    cycle_counter: usize,
    instruction_pointer: usize,
    program: &'a [Instruction],
    addition_progress: u8,
}

impl Cpu<'_> {
    fn new(program: &[Instruction]) -> Cpu {
        Cpu {
            x: 1,
            cycle_counter: 1,
            instruction_pointer: 0,
            program,
            addition_progress: 0,
        }
    }

    fn run_until(&mut self, cycle: usize) {
        while self.cycle_counter < cycle {
            match self.program[self.instruction_pointer] {
                Instruction::Noop => {
                    self.cycle_counter += 1;
                    self.instruction_pointer += 1;
                    self.addition_progress = 0;
                }
                Instruction::AddX(arg) => {
                    self.cycle_counter += 1;
                    self.addition_progress += 1;
                    if self.addition_progress == 2 {
                        self.x += arg;
                        self.instruction_pointer += 1;
                        self.addition_progress = 0;
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        self.run_until(self.cycle_counter + 1);
    }

    fn total_cycles(&self) -> usize {
        let mut total = 0;
        for instruction in self.program {
            total += instruction.cycles();
        }
        total
    }
}

struct Crt {
    current_pixel: usize,
}

impl Crt {
    fn new() -> Crt {
        Crt { current_pixel: 0 }
    }

    fn draw(&mut self, sprite: usize) -> String {
        let mut pixel = String::new();
        if self.current_pixel % 40 == 0 {
            pixel += "\n";
        }
        if sprite.abs_diff(self.current_pixel % 40) <= 1 {
            pixel += "█";
        } else {
            pixel += "░";
        }
        self.current_pixel += 1;
        pixel
    }
}

struct Device<'a> {
    cpu: Cpu<'a>,
    crt: Crt,
}

impl Device<'_> {
    fn new(cpu: Cpu, crt: Crt) -> Device {
        Device { cpu, crt }
    }

    fn run(&mut self) -> String {
        let mut rendered = String::new();
        for _ in 0..self.cpu.total_cycles() {
            rendered += &self.crt.draw((self.cpu.x) as usize);
            self.cpu.tick();
        }
        rendered
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(program: &[Instruction]) -> isize {
    let mut cpu = Cpu::new(program);
    let mut signal_sum = 0;
    for i in (20..=220).step_by(40) {
        cpu.run_until(i);
        signal_sum += cpu.x * i as isize;
    }
    signal_sum
}

#[aoc(day10, part2)]
pub fn solve_part2(program: &[Instruction]) -> String {
    let cpu = Cpu::new(program);
    let crt = Crt::new();
    let mut device = Device::new(cpu, crt);
    device.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day10/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 13140);
    }

    #[test]
    fn part2() {
        println!("{}", solve_part2(&input_generator(INPUT)));
        // panic!();
    }
}
