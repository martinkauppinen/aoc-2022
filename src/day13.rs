use serde::Deserialize;
use std::cmp::Ordering;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged)]
enum Packet {
    Integer(isize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Packet::Integer(a) => match other {
                Packet::Integer(b) => Some(a.cmp(b)),
                Packet::List(_) => {
                    let list_value = Packet::List(vec![Packet::Integer(*a)]);
                    list_value.partial_cmp(other)
                }
            },
            Packet::List(left) => match other {
                Packet::Integer(b) => {
                    let list_value = Packet::List(vec![Packet::Integer(*b)]);
                    self.partial_cmp(&list_value)
                }
                Packet::List(right) => {
                    if !left.is_empty() && right.is_empty() {
                        return Some(Ordering::Greater);
                    }

                    let mut it_left = left.iter();
                    let mut it_right = right.iter();

                    for (l, r) in it_left.by_ref().zip(it_right.by_ref()) {
                        let cmp = match l.partial_cmp(r) {
                            Some(Ordering::Equal) => None,
                            None => None,
                            cmp => cmp,
                        };
                        if cmp.is_some() {
                            return cmp;
                        }
                    }

                    let last_left = it_left.next();
                    let last_right = it_right.next();
                    if last_left.is_none() && last_right.is_some() {
                        Some(Ordering::Less)
                    } else if last_left.is_some() && last_right.is_none() {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    }
                }
            },
        }
    }
}

#[aoc_generator(day13, part1)]
fn input_generator(input: &str) -> Vec<Packet> {
    let mut packets = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        packets.push(serde_json::from_str(line).unwrap());
    }
    packets
}

#[aoc(day13, part1)]
fn solve_part1(packets: &[Packet]) -> usize {
    let mut correct_indices = vec![];

    for (i, packet) in packets.chunks(2).enumerate() {
        if packet[0] < packet[1] {
            correct_indices.push(i + 1);
        }
    }
    correct_indices.iter().sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    let mut packets = input_generator(input);
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut div1 = None;
    let mut div2 = None;

    for (i, packet) in packets.iter().enumerate() {
        if packet.partial_cmp(&Packet::Integer(2)) != Some(Ordering::Less) && div1.is_none() {
            div1 = Some(i + 1);
        }
        if packet.partial_cmp(&Packet::Integer(6)) != Some(Ordering::Less) && div2.is_none() {
            div2 = Some(i + 2);
            break;
        }
    }
    div1.unwrap() * div2.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day13/example.txt");

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 140);
    }

    #[test]
    fn input() {
        let packets = input_generator(INPUT);
        let expected = [true, true, false, true, false, true, false, false];
        for (packet, correct) in packets.chunks(2).zip(expected) {
            assert_eq!(packet[0] < packet[1], correct);
        }
    }
}
