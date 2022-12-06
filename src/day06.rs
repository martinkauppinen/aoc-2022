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

    #[test]
    fn part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
