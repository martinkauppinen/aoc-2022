use std::collections::BinaryHeap;

type InventoryTotal = usize;

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Vec<InventoryTotal> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}

#[aoc(day01, part1)]
pub fn solve_part1(inventories: &[InventoryTotal]) -> usize {
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    for inventory in inventories {
        heap.push(*inventory);
    }
    heap.pop().unwrap()
}

#[aoc(day01, part2)]
pub fn solve_part2(inventories: &[InventoryTotal]) -> usize {
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    for inventory in inventories {
        heap.push(*inventory);
    }
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day01/example.txt");

    #[test]
    fn part1() {
        let solution = solve_part1(&input_generator(INPUT));
        assert_eq!(solution, 24000);
    }

    #[test]
    fn part2() {
        let solution = solve_part2(&input_generator(INPUT));
        assert_eq!(solution, 45000);
    }
}
