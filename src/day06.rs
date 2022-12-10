fn find_offset_after_distinct(input: &str, distinct_count: usize) -> Option<usize> {
    'windows: for (i, window) in input.as_bytes().windows(distinct_count).enumerate() {
        for j in 0..window.len() - 1 {
            for k in j + 1..window.len() {
                if window[j] == window[k] {
                    continue 'windows;
                }
            }
        }
        return Some(i + window.len());
    }
    None
}

#[aoc(day06, part1)]
fn solve_part1(input: &str) -> usize {
    find_offset_after_distinct(input, 4).unwrap()
}

#[aoc(day06, part2)]
fn solve_part2(input: &str) -> usize {
    find_offset_after_distinct(input, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day06/example.txt");

    #[test]
    fn part1() {
        let answers = [7, 5, 6, 10, 11];
        for (i, line) in INPUT.lines().enumerate() {
            assert_eq!(solve_part1(line), answers[i]);
        }
    }

    #[test]
    fn part2() {
        let answers = [19, 23, 23, 29, 26];
        for (i, line) in INPUT.lines().enumerate() {
            assert_eq!(solve_part2(line), answers[i]);
        }
    }
}
