use grid::Grid;
use std::io::{self, Write};

#[derive(parse_display::FromStr, Clone, Copy, PartialEq, Eq)]
#[display("{x},{y}")]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to(&self, other: &Point) -> Vec<Point> {
        let mut points = vec![];

        let from_x = self.x.min(other.x);
        let from_y = self.y.min(other.y);
        let to_x = self.x.max(other.x);
        let to_y = self.y.max(other.y);

        if from_x != to_x {
            for x in from_x..to_x {
                points.push(Point { x, y: self.y });
            }
        } else {
            for y in from_y..to_y {
                points.push(Point { x: self.x, y });
            }
        }
        points
    }
}

struct Path {
    key_points: Vec<Point>,
}

impl Path {
    fn new() -> Path {
        Path { key_points: vec![] }
    }

    fn push(&mut self, point: Point) {
        self.key_points.push(point);
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for pair in self.key_points.windows(2) {
            let a = pair[0];
            let b = pair[1];
            points.push(a);
            a.to(&b).iter().for_each(|p| points.push(*p));
            points.push(b);
        }
        points.push(*self.key_points.last().unwrap());
        points
    }
}

#[derive(Clone)]
pub enum Tile {
    Source,
    Air,
    Rock,
    Sand,
}

type Cave = Grid<Tile>;

struct Simulation {
    cave: Cave,
    sand_source: Point,
}

impl Simulation {
    fn spawn_grain(&mut self) -> Option<Point> {
        let mut path = Path::new();
        let mut check = self.sand_source;
        path.push(check);

        while self.cave.get(check.y, check.x).is_some() {
            // Down
            if let Some(Tile::Air) = self.cave.get(check.y + 1, check.x) {
                check.y += 1;
                path.push(check);
                continue;
            }

            // Make sure we have not hit left edge
            check.x.checked_sub(1)?;

            // Left
            if let Some(Tile::Air) = self.cave.get(check.y + 1, check.x - 1) {
                check.y += 1;
                check.x -= 1;
                path.push(check);
                continue;
            }

            // Make sure we have not hit right edge
            self.cave.get(check.y + 1, check.x + 1)?;

            // Right
            if let Some(Tile::Air) = self.cave.get(check.y + 1, check.x + 1) {
                check.y += 1;
                check.x += 1;
                path.push(check);
                continue;
            }

            break;
        }

        let grain = path.key_points.last().unwrap();
        *self.cave.get_mut(grain.y, grain.x).unwrap() = Tile::Sand;

        Some(*grain)
    }
}

fn input_generator(input: &str, pseudo_infinite: bool) -> Simulation {
    let mut rocks = vec![];

    let mut min_x = usize::max_value();
    let mut min_y = usize::max_value();
    let mut max_x = usize::min_value();
    let mut max_y = usize::min_value();

    for line in input.lines() {
        let mut path = Path::new();
        let points = line.split(" -> ");
        for point in points {
            let point: Point = point.parse().unwrap();
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            path.push(point);
        }
        rocks.push(path);
    }

    let mut source_offset = 500 - min_x;
    let cave_height = max_y + 1;
    let cave_width = if pseudo_infinite {
        // Hacky, but since the pile will be triangular, the cave basically
        // only has to be twice as wide as the height
        source_offset += cave_height;
        max_x - min_x + 1 + 2 * cave_height
    } else {
        max_x - min_x + 1
    };

    let mut cave = Cave::init(cave_height, cave_width, Tile::Air);
    *cave.get_mut(0, source_offset).unwrap() = Tile::Source;

    // Add floor
    if pseudo_infinite {
        let mut air = Vec::with_capacity(cave.cols());
        air.resize(cave.cols(), Tile::Air);
        cave.push_row(air);
        let mut rock = Vec::with_capacity(cave.cols());
        rock.resize(cave.cols(), Tile::Rock);
        cave.push_row(rock)
    }

    for rock in rocks {
        for Point { x, y } in rock.all_points() {
            if pseudo_infinite {
                *cave.get_mut(y, x - min_x + cave_height).unwrap() = Tile::Rock;
            } else {
                *cave.get_mut(y, x - min_x).unwrap() = Tile::Rock;
            }
        }
    }

    Simulation {
        cave,
        sand_source: Point {
            x: source_offset,
            y: 0,
        },
    }
}

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> usize {
    let mut sim = input_generator(input, false);
    let mut count = 0;
    while sim.spawn_grain().is_some() {
        count += 1;
        // Uncomment for visualization!
        //print_cave(&sim.cave);
    }
    count
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> usize {
    let mut sim = input_generator(input, true);
    let mut count = 0;
    while let Some(point) = sim.spawn_grain() {
        count += 1;
        if point == sim.sand_source {
            break;
        }
        // Uncomment for visualization!
        //print_cave(&sim.cave);
    }
    count
}

#[allow(dead_code)]
fn print_cave(cave: &Cave) {
    print!("{esc}[1;1H", esc = 27 as char);
    let mut lock = io::stdout().lock();
    for (i, tile) in cave.iter().enumerate() {
        if i % cave.cols() == 0 {
            writeln!(lock).unwrap();
        }
        match tile {
            Tile::Source => write!(lock, "🭦🭛").unwrap(),
            Tile::Air => write!(lock, "  ").unwrap(),
            Tile::Rock => write!(lock, "🪨").unwrap(),
            Tile::Sand => write!(lock, "🟡").unwrap(),
        }
    }
    writeln!(lock).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../examples/day14/example.txt");

    #[test]
    #[ignore]
    fn input() {
        let mut sim = input_generator(INPUT, false);
        print_cave(&sim.cave);
        while let Some(point) = sim.spawn_grain() {
            print_cave(&sim.cave);
            if point == sim.sand_source {
                break;
            }
        }
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 24);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 93);
    }
}
