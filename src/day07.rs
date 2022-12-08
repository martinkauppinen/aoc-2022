use std::collections::HashMap;

#[derive(Debug)]
pub enum DirEnt {
    Directory(String, Vec<String>),
    File(String, usize),
}

fn input_generator(input: &str) -> HashMap<String, DirEnt> {
    let mut tree = HashMap::new();
    tree.insert(
        String::from("/"),
        DirEnt::Directory(String::from("/"), Vec::new()),
    );
    let mut current_path = String::from("/");
    for line in input.lines().skip(1) {
        if line.starts_with('$') {
            // Command
            let mut words = line.split_whitespace();
            let command = words.nth(1).unwrap();
            let arg = words.next();
            if command == "cd" {
                if let Some("..") = arg {
                    let mut path_elements: Vec<String> =
                        current_path.split('/').map(|s| s.to_string()).collect();
                    path_elements.pop();
                    path_elements.pop();
                    current_path = path_elements.join("/") + "/";
                } else if let Some(dir) = arg {
                    current_path += dir;
                    current_path += "/";
                    tree.insert(
                        current_path.clone(),
                        DirEnt::Directory(current_path.clone(), Vec::new()),
                    );
                }
            }
        } else {
            // Command output
            let (size_or_dir, name) = line.split_once(' ').unwrap();
            let path = current_path.clone() + name;
            let entries =
                if let DirEnt::Directory(_, entries) = tree.get_mut(&current_path).unwrap() {
                    entries
                } else {
                    unreachable!()
                };
            if size_or_dir == "dir" {
                entries.push(path.clone() + "/");
                tree.insert(
                    path.clone() + "/",
                    DirEnt::Directory(path.clone(), Vec::new()),
                );
            } else {
                entries.push(path.clone());
                tree.insert(
                    path.clone(),
                    DirEnt::File(path, size_or_dir.parse().unwrap()),
                );
            }
        }
    }
    tree
}

fn part1_dir_traverser(dir: &DirEnt, dirs: &HashMap<String, DirEnt>) -> usize {
    let ret = match dir {
        DirEnt::Directory(_name, entries) => entries
            .iter()
            .map(|e| part1_dir_traverser(dirs.get(e).unwrap(), dirs))
            .sum(),
        DirEnt::File(_name, size) => *size,
    };
    ret
}

#[aoc(day07, part1)]
fn solve_part1(input: &str) -> usize {
    let tree = input_generator(input);
    let mut sizes = Vec::new();

    for key in tree.keys() {
        if key == "/" {
            continue;
        }

        if let dir @ DirEnt::Directory(..) = tree.get(key).unwrap() {
            sizes.push(part1_dir_traverser(dir, &tree));
        }
    }

    sizes.iter().filter(|size| size <= &&100000).sum()
}

#[aoc(day07, part2)]
fn solve_part2(input: &str) -> usize {
    let tree = input_generator(input);
    let used = part1_dir_traverser(tree.get("/").unwrap(), &tree);
    let max = 70000000;
    let needed = 30000000;
    let left = max - used;

    let mut smallest = max;

    for key in tree.keys() {
        if key == "/" {
            continue;
        }

        if let dir @ DirEnt::Directory(..) = tree.get(key).unwrap() {
            let dir_size = part1_dir_traverser(dir, &tree);
            if dir_size < smallest && dir_size >= needed - left {
                smallest = dir_size;
            }
        }
    }
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 24933642);
    }
}
