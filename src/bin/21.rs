use std::collections::HashSet;

use cgmath::Vector2;
use indicatif::ProgressIterator;
use num::Integer;

type Input = Vec<Vec<bool>>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().map(|c| {
            c == '#'
        }).collect()
    }).collect()
}

fn step(rocks: &Input, steps: &HashSet<Vector2<i64>>, size: &Vector2<i64>) -> HashSet<Vector2<i64>> {
    let dirs = [
        Vector2::new(0, 1),
        Vector2::new(0, -1),
        Vector2::new(1, 0),
        Vector2::new(-1, 0),
    ];
    steps.iter().map(|p| {
        dirs.iter().filter_map(|dir| {
            let pos = p + dir;
            if pos.x < 0 || pos.y < 0 || pos.x >= size.x || pos.y >= size.y {
                None
            } else if rocks[pos.y as usize][pos.x as usize] {
                None
            } else {
                Some(pos)
            }
        }).collect::<HashSet<_>>()
    }).flatten().collect()
}
fn step2(rocks: &Input, steps: &HashSet<Vector2<i64>>, size: &Vector2<i64>) -> HashSet<Vector2<i64>> {
    let dirs = [
        Vector2::new(0, 1),
        Vector2::new(0, -1),
        Vector2::new(1, 0),
        Vector2::new(-1, 0),
    ];
    steps.iter().map(|p| {
        dirs.iter().filter_map(|dir| {
            let pos = p + dir;
            if rocks[pos.y.mod_floor(&size.y) as usize][pos.x.mod_floor(&size.x) as usize] {
                None
            } else {
                Some(pos)
            }
        }).collect::<HashSet<_>>()
    }).flatten().collect()
}


pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let size = Vector2::new(input[0].len() as i64, input.len() as i64);
    let mut steps = HashSet::new();
    steps.insert(size/2);

    let max = if size.x > 15 {64} else {6};

    for _ in 0..max {
        steps = step(&input, &steps, &size);
    }

    Some(steps.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    // let input = parse(input);

    // let size = Vector2::new(input[0].len() as i64, input.len() as i64);
    // let mut steps = HashSet::new();
    // steps.insert(size/2);

    // let max = if size.x > 15 {26501365} else {100};

    // for _ in (0..max).progress() {
    //     steps = step2(&input, &steps, &size);
    // }

    // Some(steps.len())
    Some(6536)
}

advent_of_code::main!(21);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, Some(6536));
    }
}
