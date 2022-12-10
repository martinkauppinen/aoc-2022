#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn from_str(s: &str) -> Throw {
        match s {
            "A" | "X" => Throw::Rock,
            "B" | "Y" => Throw::Paper,
            "C" | "Z" => Throw::Scissors,
            _ => unreachable!("{}", s),
        }
    }

    fn score(&self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn play(&self, other: &Throw) -> Outcome {
        if *other == self.get_better() {
            Outcome::Loss
        } else if *other == self.get_worse() {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    fn get_better(&self) -> Throw {
        match self {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        }
    }

    fn get_worse(&self) -> Throw {
        match self {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from_str(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!("{}", s),
        }
    }

    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }

    fn force_from(&self, throw: &Throw) -> Throw {
        match self {
            Outcome::Win => throw.get_better(),
            Outcome::Loss => throw.get_worse(),
            Outcome::Draw => *throw,
        }
    }
}

pub type Round = (Throw, Throw);
pub type ForcedRound = (Throw, Outcome);

#[aoc_generator(day02, part1)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (Throw::from_str(a), Throw::from_str(b)))
        .collect()
}

#[aoc_generator(day02, part2)]
pub fn input_generator2(input: &str) -> Vec<ForcedRound> {
    input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (Throw::from_str(a), Outcome::from_str(b)))
        .collect()
}

#[aoc(day02, part1)]
pub fn solve_part1(rounds: &[Round]) -> usize {
    rounds.iter().fold(0, |acc, (opponent, me)| {
        acc + me.play(opponent).score() + me.score()
    })
}

#[aoc(day02, part2)]
pub fn solve_part2(rounds: &[ForcedRound]) -> usize {
    rounds.iter().fold(0, |acc, (opponent, outcome)| {
        let me = outcome.force_from(opponent);
        acc + me.play(opponent).score() + me.score()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day02/example.txt");

    #[test]
    fn part1() {
        let solution = solve_part1(&input_generator(INPUT));
        assert_eq!(solution, 15);
    }

    #[test]
    fn part2() {
        let solution = solve_part2(&input_generator2(INPUT));
        assert_eq!(solution, 12);
    }
}
