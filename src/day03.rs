use itertools::Itertools;

fn to_priority(c: char) -> usize {
    assert!(c.is_ascii_alphabetic());

    let ascii = c as usize;
    if c.is_lowercase() {
        ascii - ('a' as usize) + 1
    } else {
        ascii - ('A' as usize) + 1 + 26
    }
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &str) -> usize {
    let rucksacks = input.lines().map(|line| line.split_at(line.len() / 2));

    let mut sum = 0;
    for (a, b) in rucksacks {
        for c in a.chars() {
            if b.contains(c) {
                sum += to_priority(c);
                break;
            }
        }
    }
    sum
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut sum = 0;
    for (a, b, c) in input.lines().tuples() {
        for item in a.chars() {
            if b.contains(item) && c.contains(item) {
                sum += to_priority(item);
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day03/example.txt");

    #[test]
    fn part1() {
        let solution = solve_part1(INPUT);
        assert_eq!(solution, 157);
    }

    #[test]
    fn part2() {
        let solution = solve_part2(INPUT);
        assert_eq!(solution, 70);
    }
}
