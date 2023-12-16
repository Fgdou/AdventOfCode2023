use std::collections::HashSet;

use cgmath::{Vector2, Zero};

type Input = Vec<Vec<char>>;
#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}
#[derive(Clone, PartialEq, Eq, Hash)]
struct Beam {
    pos: Vector2<i32>,
    dir: Direction
}

impl Beam {
    fn next(&self, c: char) -> Vec<Beam> {
        let dirs = self.dir.next(c);

        dirs.into_iter().map(|dir| {
            Beam {
                pos: self.pos,
                dir
            }.move_dir()
        })
        .collect()
    }

    fn move_dir(mut self) -> Self {
        match self.dir {
            Direction::Up => self.pos.y -= 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Right => self.pos.x += 1,
        }
        self
    }
}

impl Direction {
    fn next(&self, c: char) -> Vec<Direction> {
        match (c, self) {
            ('/', Direction::Right) => vec!(Direction::Up),
            ('/', Direction::Down) => vec!(Direction::Left),
            ('/', Direction::Up) => vec!(Direction::Right),
            ('/', Direction::Left) => vec!(Direction::Down),
            ('\\', Direction::Right) => vec!(Direction::Down),
            ('\\', Direction::Left) => vec!(Direction::Up),
            ('\\', Direction::Up) => vec!(Direction::Left),
            ('\\', Direction::Down) => vec!(Direction::Right),
            ('|', d) if d == &Direction::Left || d == &Direction::Right => vec!(Direction::Up, Direction::Down),
            ('-', d) if d == &Direction::Up || d == &Direction::Down => vec!(Direction::Left, Direction::Right),
            (_, d) => vec!(d.clone())
        }
    }
}
fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn count(input: &Input, start_beam: Beam) -> usize {
    let mut stack = Vec::<Beam>::new();
    let mut cache = HashSet::<Beam>::new();

    stack.push(start_beam);

    while let Some(b) = stack.pop() {
        if cache.contains(&b) {
            continue;
        }
        if b.pos.x < 0 || b.pos.y < 0 || b.pos.x >= input[0].len() as i32 || b.pos.y >= input.len() as i32 {
            continue;
        }
        
        stack.extend(b.next(input[b.pos.y as usize][b.pos.x as usize]));
        cache.insert(b.clone());
    }

    let poses = cache.into_iter().map(|b| b.pos).collect::<HashSet<_>>();
    poses.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let start_beam = Beam{
        dir: Direction::Right,
        pos: Vector2::zero()
    };


    Some(count(&input, start_beam))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let mut starts = Vec::new();

    for i in 0..input[0].len() {
        starts.push(Beam{
            pos: Vector2::new(i as i32, 0),
            dir: Direction::Down
        });
        starts.push(Beam{
            pos: Vector2::new(i as i32, input.len() as i32-1),
            dir: Direction::Up
        });
    }
    for i in 0..input.len() {
        starts.push(Beam{
            pos: Vector2::new(0, i as i32),
            dir: Direction::Right
        });
        starts.push(Beam{
            pos: Vector2::new(input[0].len() as i32-1, i as i32),
            dir: Direction::Up
        });
    }

    let res = starts.into_iter().map(|start| count(&input, start)).max().unwrap();

    Some(res)
}

advent_of_code::main!(16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(51));
    }
}
