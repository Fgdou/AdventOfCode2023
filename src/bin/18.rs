use std::collections::HashSet;

use cgmath::{Vector2, Zero};

#[derive(Clone, PartialEq, Debug)]
struct Instruction {
    dir: Vector2<i32>,
    count: usize,
    color: String
}
type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        let infos: Vec<&str> = line.split(" ").collect();
        let dir = match infos[0] {
            "R" => Vector2::new(1, 0),
            "L" => Vector2::new(-1, 0),
            "U" => Vector2::new(0, -1),
            "D" => Vector2::new(0, 1),
            _ => unreachable!()
        };
        let count = infos[1].parse().unwrap();
        let color = &infos[2][1..infos.len()-2];
        Instruction {
            color: color.to_string(),
            count,
            dir
        }
    }).collect()
}

fn run_border(input: &Input) -> HashSet<Vector2<i32>> {
    let mut set = HashSet::new();

    let mut pos = Vector2::zero();

    for instruction in input {
        for _ in 0..instruction.count {
            set.insert(pos.clone());
            pos += instruction.dir
        }
    }

    set
}

fn dig(set: HashSet<Vector2<i32>>) -> usize {
    let x_min = set.iter().map(|e| e.x).min().unwrap()-1;
    let y_min = set.iter().map(|e| e.y).min().unwrap()-1;
    let x_max = set.iter().map(|e| e.x).max().unwrap()+1;
    let y_max = set.iter().map(|e| e.y).max().unwrap()+1;

    let mut visited = HashSet::new();

    let mut stack = Vec::new();
    stack.push(Vector2::new(x_min, y_min));

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        if visited.contains(&pos) || set.contains(&pos) {
            continue;
        }
        if pos.x < x_min || pos.x > x_max || pos.y < y_min || pos.y > y_max {
            continue;
        }

        stack.push(pos + Vector2::new(1, 0));
        stack.push(pos + Vector2::new(-1, 0));
        stack.push(pos + Vector2::new(0, 1));
        stack.push(pos + Vector2::new(0, -1));

        visited.insert(pos);
    }

    ((y_max-y_min+1)*(x_max-x_min+1) - visited.len() as i32) as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let border = run_border(&input);
    let area = dig(border);

    Some(area)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(18);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, None);
    }
}
