#[derive(Copy, Clone)]
pub struct Section(usize, usize);

impl Section {
    pub fn contained_by(&self, other: &Section) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    pub fn overlaps(&self, other: &Section) -> bool {
        self.contained_by(other)
            || other.contained_by(self)
            || self.0 >= other.0 && self.0 <= other.1
            || self.1 <= other.1 && self.1 >= other.0
    }
}

pub type Assignment = (Section, Section);

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(|pair| {
            let mut assignment: [Section; 2] = [Section(0, 0), Section(0, 0)];
            for (i, elf) in pair.split(',').enumerate() {
                let section = elf.split_once('-').unwrap();
                assignment[i] = Section(section.0.parse().unwrap(), section.1.parse().unwrap());
            }
            assert!(assignment.len() == 2);
            (assignment[0], assignment[1])
        })
        .collect()
}

#[aoc(day04, part1)]
pub fn solve_part1(assignments: &[Assignment]) -> usize {
    assignments.iter().fold(0, |acc, assignment| {
        let (elf1, elf2) = assignment;
        if elf1.contained_by(elf2) || elf2.contained_by(elf1) {
            return acc + 1;
        }
        acc
    })
}

#[aoc(day04, part2)]
pub fn solve_part2(assignments: &[Assignment]) -> usize {
    assignments.iter().fold(0, |acc, (elf1, elf2)| {
        if elf1.overlaps(elf2) {
            return acc + 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day04/example.txt");

    #[test]
    fn part1() {
        let solution = solve_part1(&input_generator(INPUT));
        assert_eq!(solution, 2);
    }

    #[test]
    fn part2() {
        let solution = solve_part2(&input_generator(INPUT));
        assert_eq!(solution, 4);
    }
}
