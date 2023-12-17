use std::collections::HashSet;

use cgmath::Vector2;

type Input = Vec<Vec<u32>>;

#[derive(Clone, Debug)]
enum Direction {
    Up, Down, Left, Right
}

fn move_vec(v: &Vector2<i32>, dir: &Direction) -> Vector2<i32> {
    match dir {
        Direction::Up => Vector2::new(v.x, v.y-1),
        Direction::Down => Vector2::new(v.x, v.y+1),
        Direction::Left => Vector2::new(v.x-1, v.y),
        Direction::Right => Vector2::new(v.x+1, v.y),
    }
}

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect()
}

fn visit(input: &Input, pos: Vector2<i32>, straight_count: u32, dir: &Direction, mut visited: HashSet<Vector2<i32>>) -> Option<u32> {
    if pos.x == input[0].len() as i32-1 && pos.y == input.len() as i32-1 {
        return Some(0);
    }
    if pos.x < 0 || pos.x >= input[0].len() as i32 || pos.y < 0 || pos.y >= input.len() as i32 {
        return None;
    }
    if visited.contains(&pos) {
        return None;
    }
    visited.insert(pos.clone());

    let mut moves = vec!();

    if straight_count < 3 {
        moves.push(visit(input, move_vec(&pos, dir), straight_count+1, dir, visited.clone()));
    }

    match dir {
        Direction::Up | Direction::Down => {
            moves.push(visit(input, move_vec(&pos, &Direction::Left), 0, &Direction::Left, visited.clone()));
            moves.push(visit(input, move_vec(&pos, &Direction::Right), 0, &Direction::Right, visited.clone()));
        },
        Direction::Left | Direction::Right => {
            moves.push(visit(input, move_vec(&pos, &Direction::Up), 0, &Direction::Up, visited.clone()));
            moves.push(visit(input, move_vec(&pos, &Direction::Down), 0, &Direction::Down, visited.clone()));
        },
    }

    Some(moves.into_iter().filter_map(|e| e).min()? + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    visit(&input, Vector2::new(0, 0), 0, &Direction::Right, HashSet::new())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, None);
    }
}
