use itertools::Itertools;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(PartialEq, Eq, Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

struct CraneProcedure {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

fn crate_mover_9000(procedure: &mut CraneProcedure) {
    for Move { amount, from, to } in &procedure.moves {
        let len = procedure.stacks[from - 1].len();
        let crates = procedure.stacks[from - 1].split_off(len - amount);
        procedure.stacks[to - 1].extend(crates.iter().rev());
    }
}

fn crate_mover_9001(procedure: &mut CraneProcedure) {
    for Move { amount, from, to } in &procedure.moves {
        let len = procedure.stacks[from - 1].len();
        let mut crates = procedure.stacks[from - 1].split_off(len - amount);
        procedure.stacks[to - 1].append(&mut crates);
    }
}

fn input_generator(input: &str) -> CraneProcedure {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut reverse_stacks = stacks.lines().rev();
    let num_stacks = (reverse_stacks.next().unwrap().len() + 2) / 3;
    let mut procedure_stacks: Vec<Stack> = vec![Vec::new(); num_stacks - 1];
    for stack_row in reverse_stacks {
        for (i, mut crate_str) in stack_row.chars().chunks(4).into_iter().enumerate() {
            if let Some(character) = crate_str.nth(1) {
                if !character.is_ascii_whitespace() {
                    procedure_stacks[i].push(character);
                }
            }
        }
    }

    let mut procedure_moves = Vec::new();
    for move_ in moves.lines() {
        let mut words = move_.split_whitespace();
        procedure_moves.push(Move {
            amount: words.nth(1).unwrap().parse().unwrap(),
            from: words.nth(1).unwrap().parse().unwrap(),
            to: words.nth(1).unwrap().parse().unwrap(),
        });
    }

    CraneProcedure {
        stacks: procedure_stacks,
        moves: procedure_moves,
    }
}

#[aoc(day05, part1)]
fn solve_part1(input: &str) -> String {
    let mut procedure = input_generator(input);
    crate_mover_9000(&mut procedure);
    let mut s = String::new();
    for stack in &procedure.stacks {
        if let Some(c) = stack.last() {
            s.push(*c);
        }
    }
    s
}

#[aoc(day05, part2)]
fn solve_part2(input: &str) -> String {
    let mut procedure = input_generator(input);
    crate_mover_9001(&mut procedure);
    let mut s = String::new();
    for stack in &procedure.stacks {
        if let Some(c) = stack.last() {
            s.push(*c);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../examples/day05/example.txt");

    #[test]
    fn input() {
        let CraneProcedure { stacks, moves } = input_generator(INPUT);
        assert_eq!(
            stacks,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
        assert_eq!(
            moves,
            vec![
                Move {
                    amount: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    amount: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    amount: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    amount: 1,
                    from: 1,
                    to: 2
                },
            ]
        );
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), "CMZ".to_string());
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), "MCD".to_string());
    }
}
