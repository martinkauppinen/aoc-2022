use std::collections::VecDeque;

use grid::{grid, Grid};

type HeightMap = Grid<u8>;
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Point(usize, usize);
type Start = Point;
type End = Point;
type Input = (HeightMap, Start, End);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let mut grid: HeightMap = grid![];
    for (y, line) in input.lines().enumerate() {
        let mut elevations = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Point(x, y));
                elevations.push(0);
            } else if c == 'E' {
                end = Some(Point(x, y));
                elevations.push(25);
            } else {
                elevations.push(c.to_digit(36).expect("bad input character") as u8 - 10);
            }
        }
        grid.push_row(elevations);
    }
    (
        grid,
        start.expect("no start point"),
        end.expect("no end point"),
    )
}

fn valid_neighbours(heightmap: &HeightMap, point: Point) -> Vec<Point> {
    let point_value = *heightmap.get(point.1, point.0).unwrap() as isize;
    let mut neighbours = Vec::new();
    for i in -1..=1isize {
        for j in -1..=1isize {
            let x = point.0 as isize - j;
            let y = point.1 as isize - i;
            if x < 0 || y < 0 || i == j || i == -j {
                continue;
            }
            if let Some(neighbour) = heightmap.get(y as usize, x as usize) {
                if *neighbour as isize - point_value <= 1 {
                    neighbours.push(Point(x as usize, y as usize));
                }
            }
        }
    }
    neighbours
}

fn bfs<F>(heightmap: &HeightMap, start: Start, end_condition: F) -> usize
where
    F: FnOnce(Point) -> bool + Copy,
{
    let mut q = VecDeque::new();
    let mut explored = Grid::new(heightmap.rows(), heightmap.cols());
    explored.fill(false);
    *explored.get_mut(start.1, start.0).unwrap() = true;

    let mut parents = Grid::new(heightmap.rows(), heightmap.cols());
    parents.fill(None);

    let mut end = None;

    q.push_back(start);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        if end_condition(v) {
            end = Some(v);
            break;
        }
        for w in valid_neighbours(heightmap, v) {
            if !explored.get(w.1, w.0).unwrap() {
                *explored.get_mut(w.1, w.0).unwrap() = true;
                *parents.get_mut(w.1, w.0).unwrap() = Some(v);
                q.push_back(w);
            }
        }
    }

    let mut path_length = 0;
    let mut path_node = end;
    while let Some(node) = path_node {
        path_length += 1;
        path_node = *parents.get(node.1, node.0).unwrap();
    }

    path_length - 1
}

#[aoc(day12, part1)]
fn solve_part1(input: &Input) -> usize {
    let (heightmap, start, end) = input;
    bfs(heightmap, *start, |p| p == *end)
}

#[aoc(day12, part2)]
fn solve_part2(input: &Input) -> usize {
    let (heightmap, _start, end) = input;
    let mut heightmap = heightmap.clone();
    for height in heightmap.iter_mut() {
        *height = 25u8 - *height;
    }
    bfs(&heightmap, *end, |p| {
        heightmap.get(p.1, p.0).unwrap() == &25u8
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day12/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 29);
    }
}
