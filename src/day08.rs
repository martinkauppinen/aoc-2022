use grid::{grid, Grid};

type Trees = Grid<u8>;

#[aoc_generator(day08)]
fn input_generator(input: &str) -> Trees {
    let mut grid: Grid<u8> = grid![];
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push_row(row);
    }
    grid
}

fn visible_trees(direction: &mut dyn Iterator<Item = &u8>) -> Vec<usize> {
    let mut visible = Vec::new();
    let mut tallest = -1;
    for (i, tree) in direction.enumerate() {
        if tallest < *tree as i8 {
            tallest = *tree as i8;
            visible.push(i);
        }
    }
    visible
}

#[aoc(day08, part1)]
fn solve_part1(trees: &Trees) -> usize {
    let mut seen = Grid::init(trees.rows(), trees.cols(), 0);

    for row in 0..trees.rows() {
        let forward = visible_trees(&mut trees.iter_row(row));
        let reverse = visible_trees(&mut trees.iter_row(row).rev());
        for col in forward {
            *seen.get_mut(row, col).unwrap() = 1;
        }
        for col in reverse {
            *seen.get_mut(row, trees.cols() - 1 - col).unwrap() = 1;
        }
    }

    for col in 0..trees.cols() {
        let forward = visible_trees(&mut trees.iter_col(col));
        let reverse = visible_trees(&mut trees.iter_col(col).rev());
        for row in forward {
            *seen.get_mut(row, col).unwrap() = 1;
        }
        for row in reverse {
            *seen.get_mut(trees.rows() - 1 - row, col).unwrap() = 1;
        }
    }

    seen.iter().sum()
}

fn viewing_direction(direction: &mut dyn Iterator<Item = &u8>) -> usize {
    let height = direction.next().unwrap();
    let mut viewing_distance = 0;
    for tree in direction {
        viewing_distance += 1;
        if tree >= height {
            break;
        }
    }
    viewing_distance
}

#[aoc(day08, part2)]
fn solve_part2(trees: &Trees) -> usize {
    let mut best_score = 0;
    for row in 0..trees.rows() {
        for col in 0..trees.cols() {
            let up_score =
                viewing_direction(&mut trees.iter_col(col).rev().skip(trees.rows() - 1 - row));
            let left_score =
                viewing_direction(&mut trees.iter_row(row).rev().skip(trees.cols() - 1 - col));
            let down_score = viewing_direction(&mut trees.iter_col(col).skip(row));
            let right_score = viewing_direction(&mut trees.iter_row(row).skip(col));
            let view_score = left_score * right_score * up_score * down_score;
            if view_score > best_score {
                best_score = view_score;
            }
        }
    }
    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day08/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 8);
    }
}
