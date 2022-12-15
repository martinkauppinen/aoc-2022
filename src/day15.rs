use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, parse_display::FromStr)]
#[display("x={x}, y={y}")]
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, parse_display::FromStr, PartialEq, Clone, Copy)]
#[display("Sensor at {position}: closest beacon is at {beacon}")]
struct Sensor {
    position: Point,
    beacon: Point,
}

struct Line {
    gradient: isize,
    intercept: isize,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        if self.gradient == other.gradient {
            return None;
        }

        let x = (other.intercept - self.intercept) / (self.gradient - other.gradient);
        let y = self.gradient * x + self.intercept;
        Some(Point { x, y })
    }
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Sensor {
    fn covered_x_coords(&self, y: isize) -> RangeInclusive<isize> {
        let radius = self.position.distance(&self.beacon) as isize;
        let y_offset = (self.position.y - y).abs();
        let covered_distance = 2 * (radius - y_offset);
        let x_offset = self.position.x - radius + y_offset;
        x_offset..=x_offset + covered_distance
    }

    fn covers(&self, point: &Point) -> bool {
        self.position.distance(point) <= self.position.distance(&self.beacon)
    }

    fn bounding_lines(&self) -> [Line; 4] {
        let radius = 1 + self.position.distance(&self.beacon) as isize;

        let top_corner = Point {
            x: self.position.x,
            y: self.position.y - radius,
        };
        let bottom_corner = Point {
            x: self.position.x,
            y: self.position.y + radius,
        };

        let c1 = top_corner.y - top_corner.x; // Positive gradient
        let c2 = top_corner.y + top_corner.x; // Negative gradient
        let c3 = bottom_corner.y - bottom_corner.x; // Positive gradient
        let c4 = bottom_corner.y + bottom_corner.x; // Negative gradient

        [
            Line {
                gradient: 1,
                intercept: c1,
            },
            Line {
                gradient: -1,
                intercept: c2,
            },
            Line {
                gradient: 1,
                intercept: c3,
            },
            Line {
                gradient: -1,
                intercept: c4,
            },
        ]
    }

    fn intersections(&self, other: &Sensor) -> [Option<Point>; 8] {
        let [self_pos1, self_neg1, self_pos2, self_neg2] = self.bounding_lines();
        let [other_pos1, other_neg1, other_pos2, other_neg2] = other.bounding_lines();

        [
            self_pos1.intersection(&other_neg1),
            self_pos1.intersection(&other_neg2),
            self_neg1.intersection(&other_pos1),
            self_neg1.intersection(&other_pos2),
            self_pos2.intersection(&other_neg1),
            self_pos2.intersection(&other_neg2),
            self_neg2.intersection(&other_pos1),
            self_neg2.intersection(&other_pos2),
        ]
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Sensor> {
    let mut sensors = vec![];
    for line in input.lines() {
        sensors.push(line.parse().unwrap());
    }
    sensors
}

#[aoc(day15, part1)]
fn solve_part1(sensors: &[Sensor]) -> isize {
    #[cfg(not(test))]
    let y = 2_000_000;
    #[cfg(test)]
    let y = 10;

    let mut beacons: HashSet<Point> = HashSet::new();

    sensors.iter().for_each(|sensor| {
        beacons.insert(sensor.beacon);
    });

    let beacons_on_row = beacons.iter().filter(|b| b.y == y).count() as isize;

    let mut ranges: Vec<_> = sensors
        .iter()
        .map(|s| s.covered_x_coords(y))
        .filter(|r| r.start() <= r.end())
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut merged_ranges = vec![ranges[0].start()..=ranges[0].end()];
    ranges.iter().skip(1).for_each(|r| {
        if r.start() <= &(*merged_ranges.last().unwrap().end() + 1) {
            *merged_ranges.last_mut().unwrap() = *merged_ranges.last().unwrap().start()
                ..=r.end().max(merged_ranges.last().unwrap().end());
        } else {
            merged_ranges.push(r.start()..=r.end());
        }
    });

    merged_ranges
        .iter()
        .map(|r| {
            println!("{} -> {} (-{})", r.start(), r.end(), beacons_on_row);
            **r.end() - **r.start() + 1
        })
        .sum::<isize>()
        - beacons_on_row
}

#[aoc(day15, part2)]
fn solve_part2(sensors: &[Sensor]) -> isize {
    #[cfg(not(test))]
    let max: isize = 4_000_000;
    #[cfg(test)]
    let max: isize = 20;

    sensors
        .iter()
        .find_map(|sensor| {
            sensors.iter().find_map(|other| {
                sensor
                    .intersections(other)
                    .iter()
                    .flatten()
                    .filter(|intersection| {
                        (0..=max).contains(&intersection.x) && (0..=max).contains(&intersection.y)
                    })
                    .find_map(|intersection| {
                        sensors
                            .iter()
                            .all(|sensor| !sensor.covers(intersection))
                            .then_some(4_000_000 * intersection.x + intersection.y)
                    })
            })
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../examples/day15/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 26);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 56_000_011);
    }
}
