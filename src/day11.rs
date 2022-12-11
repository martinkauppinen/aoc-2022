use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, not_line_ending, u64},
    multi::{many0, many_till},
    sequence, IResult,
};

type MonkeyId = usize;
type ModularStressLevel = usize;

enum Operand {
    Number(ModularStressLevel),
    Old,
}

impl Operand {
    fn evaluate(&self, item: ModularStressLevel) -> ModularStressLevel {
        match self {
            Operand::Number(n) => *n,
            Operand::Old => item,
        }
    }
}

enum Operation {
    Addition(Operand, Operand),
    Multiplication(Operand, Operand),
}

impl Operation {
    fn evaluate(&self, item: ModularStressLevel) -> ModularStressLevel {
        match self {
            Operation::Addition(op1, op2) => op1.evaluate(item) + op2.evaluate(item),
            Operation::Multiplication(op1, op2) => op1.evaluate(item) * op2.evaluate(item),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let op1 = match words.next().unwrap() {
            "old" => Operand::Old,
            number => Operand::Number(number.parse().unwrap()),
        };
        let operation = words.next().unwrap();
        let op2 = match words.next().unwrap() {
            "old" => Operand::Old,
            number => Operand::Number(number.parse().unwrap()),
        };

        match operation {
            "+" => Ok(Operation::Addition(op1, op2)),
            "*" => Ok(Operation::Multiplication(op1, op2)),
            _ => Err(()),
        }
    }
}

struct Monkey {
    items: Vec<ModularStressLevel>,
    inspected: usize,
    operation: Operation,
    modulus: ModularStressLevel,
    worry_divisor: ModularStressLevel,
    true_monkey: MonkeyId,
    false_monkey: MonkeyId,
}

impl Monkey {
    fn inspect(&mut self) -> Option<(MonkeyId, ModularStressLevel)> {
        let item = self.items.pop()?;
        self.inspected += 1;
        let item = self.operation.evaluate(item);
        let item = item / self.worry_divisor;
        if item % self.modulus == 0 {
            Some((self.true_monkey, item))
        } else {
            Some((self.false_monkey, item))
        }
    }
}

pub struct Monkeys {
    monkeys: Vec<Monkey>,
    modulus: ModularStressLevel,
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Monkeys {
        let mut modulus = 1;
        for monkey in &monkeys {
            modulus *= monkey.modulus;
        }
        Monkeys { monkeys, modulus }
    }

    fn do_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((id, item)) = self.monkeys[i].inspect() {
                self.monkeys[id].items.push(item % self.modulus);
            }
        }
    }

    fn get_monkey_business(&mut self) -> usize {
        self.monkeys
            .sort_by(|monkey1, monkey2| monkey1.inspected.cmp(&monkey2.inspected));

        self.monkeys.pop().unwrap().inspected * self.monkeys.pop().unwrap().inspected
    }
}

fn next_number(input: &str) -> IResult<&str, ModularStressLevel> {
    let (input, (_, number)) = many_till(anychar, u64)(input)?;
    Ok((input, number as ModularStressLevel))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = next_number(input)?;
    let (input, (items, _)) = many_till(next_number, newline)(input)?;
    let (input, operation) =
        sequence::preceded(many_till(anychar, tag("= ")), not_line_ending)(input)?;
    let (input, modulus) = next_number(input)?;
    let (input, true_monkey) = next_number(input)?;
    let (input, false_monkey) = next_number(input)?;
    let monkey = Monkey {
        items,
        inspected: 0,
        operation: operation.parse().unwrap(),
        modulus,
        worry_divisor: 0,
        true_monkey: true_monkey as MonkeyId,
        false_monkey: false_monkey as MonkeyId,
    };

    Ok((input, monkey))
}

fn input_generator(input: &str, worry_divisor: ModularStressLevel) -> Monkeys {
    let (_, mut monkeys) = many0(parse_monkey)(input).unwrap();
    for monkey in monkeys.iter_mut() {
        monkey.worry_divisor = worry_divisor;
    }
    Monkeys::new(monkeys)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut monkeys = input_generator(input, 3);
    for _ in 0..20 {
        monkeys.do_round();
    }
    monkeys.get_monkey_business()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut monkeys = input_generator(input, 1);
    for _ in 0..10000 {
        monkeys.do_round();
    }
    monkeys.get_monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day11/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 10605);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 2713310158);
    }
}
