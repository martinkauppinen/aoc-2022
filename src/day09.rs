use itertools::Itertools;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    steps: isize,
}

#[aoc_generator(day09)]
fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (direction, steps) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            Move {
                direction,
                steps: steps.parse().unwrap(),
            }
        })
        .collect()
}

fn new_tail_position(head: &(isize, isize), tail: &(isize, isize)) -> (isize, isize) {
    if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
        (
            tail.0 + (head.0 - tail.0).signum(),
            tail.1 + (head.1 - tail.1).signum(),
        )
    } else {
        *tail
    }
}

#[aoc(day09, part1)]
fn solve_part1(moves: &[Move]) -> usize {
    let mut tail_positions = vec![(0, 0)];
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    for Move { direction, steps } in moves {
        for _ in 0..*steps {
            match direction {
                Direction::Up => head_position.1 -= 1,
                Direction::Down => head_position.1 += 1,
                Direction::Left => head_position.0 -= 1,
                Direction::Right => head_position.0 += 1,
            }
            tail_position = new_tail_position(&head_position, &tail_position);
            tail_positions.push(tail_position);
        }
    }

    tail_positions.iter().unique().count()
}

#[aoc(day09, part2)]
fn solve_part2(moves: &[Move]) -> usize {
    let mut tail_positions = vec![(0, 0)];
    let mut knots = vec![(0, 0); 10];
    for Move { direction, steps } in moves {
        for _ in 0..*steps {
            match direction {
                Direction::Up => knots[0].1 -= 1,
                Direction::Down => knots[0].1 += 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            }

            for i in 1..knots.len() {
                knots[i] = new_tail_position(&knots[i - 1], &knots[i]);
            }
            tail_positions.push(*knots.last().unwrap());
        }
    }
    tail_positions.iter().unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("../examples/day09/example.txt");
    const INPUT2: &str = include_str!("../examples/day09/example2.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 36);
    }
}
