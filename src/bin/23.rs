use std::collections::{HashMap, HashSet};

use cgmath::Vector2;

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn find_free(line: &Vec<char>) -> isize {
    line.iter().position(|e| e == &'.').unwrap() as isize
}

fn run(input: &Input, start: &Vector2<isize>) -> HashMap<Vector2<isize>, isize> {
    let mut visited = HashMap::new();
    let mut to_visit = vec!((start.clone(), start + Vector2::new(0, 1), 1));

    while !to_visit.is_empty() {
        let (prev_pos, pos, value) = to_visit.pop().unwrap();

        if pos.x < 0 || pos.y < 0 || pos.x >= input[0].len() as isize || pos.y >= input.len() as isize {
            continue;
        }

        let c = input[pos.y as usize][pos.x as usize];

        let diff = pos-prev_pos;
        let can_move = match (diff, c) {
            (_, '.') => true,
            (Vector2{x: 1, y: 0}, '>') => true,
            (Vector2{x: 0, y: 1}, 'v') => true,
            (Vector2{x: -1, y: 0}, '<') => true,
            (Vector2{x: 0, y: -1}, '^') => true,
            _ => false
        };

        if !can_move {
            continue;
        }

        if let Some(pred_value) = visited.get(&pos) {
            if pred_value >= &value {
                continue;
            }
        }

        visited.insert(pos.clone(), value);
        for d in [Vector2::new(0, 1),Vector2::new(0, -1),Vector2::new(1, 0),Vector2::new(-1, 0)] {
            if d == -diff {
                continue;
            }
            to_visit.push((pos, pos+d, value+1));
        }
    }

    visited
}

fn print(input: &Input, map: &HashMap<Vector2<isize>, isize>) {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let pos = Vector2::new(j as isize, i as isize);
            if let Some(v) = map.get(&pos) {
                print!("{}", v.to_string().chars().last().unwrap());
            } else {
                print!("{}", input[i][j])
            }
        }
        println!();
    }
    println!();
}

fn run2(input: &Input, pos: Vector2<isize>, end: &Vector2<isize>, mut history: HashSet<Vector2<isize>>) -> Option<isize> {
    if pos.x < 0 || pos.y < 0 || pos.x >= input[0].len() as isize || pos.y >= input.len() as isize {
        return None;
    }
    if history.contains(&pos) {
        return None;
    }
    if &pos == end {
        return Some(history.len() as isize)
    }

    let c = input[pos.y as usize][pos.x as usize];
    if c == '#' {
        return None;
    }

    history.insert(pos);

    let max = [Vector2::new(0, 1),Vector2::new(0, -1),Vector2::new(1, 0),Vector2::new(-1, 0)].iter()
        .filter_map(|d| {
            let pos = pos + d;
            run2(input, pos, end, history.clone())
        }).max();

    max
}

pub fn part_one(input: &str) -> Option<isize> {
    let input = parse(input);

    let start = Vector2::new(find_free(&input[0]), 0);
    let end = Vector2::new(find_free(&input[input.len()-1]), input.len() as isize-1);

    let res = run(&input, &start);

    Some(res[&end])
}

pub fn part_two(input: &str) -> Option<isize> {
    let input = parse(input);

    let start = Vector2::new(find_free(&input[0]), 0);
    let end = Vector2::new(find_free(&input[input.len()-1]), input.len() as isize-1);

    let res = run2(&input, start, &end, Default::default());

    res
}

advent_of_code::main!(23);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(154));
    }
}
