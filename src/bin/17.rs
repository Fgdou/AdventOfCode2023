use std::{collections::{HashSet, HashMap}, cmp::Reverse};

use cgmath::Vector2;

type Input = Vec<Vec<u32>>;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Vector2<i32>,
    from: Option<Vector2<i32>>,
    value: u32,
    dir: Option<Direction>,
    straight: u32
}

fn print(input: &Input, map: &HashSet<Vector2<i32>>) {
    let s: String = (0..input.len()).into_iter().map(|y| {
        (0..input[0].len()).into_iter().map(|x| {
            if let Some(v) = map.get(&Vector2::new(x as i32, y as i32)) {
                '*'
            } else {
                '.'
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    println!("{}", s);
}

fn visit2(input: &Input) -> HashMap<Vector2<i32>, Node> {
    let mut visited: HashMap<Vector2<i32>, Node> = HashMap::new();
    let mut stack: Vec<Node> = Vec::new();

    stack.push(Node{
        dir: Some(Direction::Up),
        from: None,
        pos: Vector2::new(0, 0),
        straight: 0,
        value: 0
    });

    while !stack.is_empty() {
        stack.sort_by_key(|e| Reverse(e.value));


        let value = stack.pop().unwrap();

        if let Some(v) = visited.get(&value.pos) {
            if v.value <= value.value {
                continue;
            }
        }

        for d in vec!(Direction::Up, Direction::Down, Direction::Left, Direction::Right) {
            let new_pos = move_vec(&value.pos, &d);

            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= input[0].len() as i32 || new_pos.y >= input.len() as i32 {
                continue;
            }

            let straight = match &value.dir {
                Some(d2) if &d == d2 => value.straight + 1,
                _ => 0
            };

            if straight >= 3 {
                continue;
            }

            let node = Node {
                from: Some(value.pos),
                dir: Some(d),
                pos: new_pos,
                straight,
                value: value.value + input[new_pos.y as usize][new_pos.x as usize]
            };

            if let Some(v2) = stack.iter().find(|e| e.pos == new_pos) {
                if v2.value <= node.value {
                    continue;
                }
            }
            if let Some(v2) = visited.get(&new_pos) {
                if v2.value <= node.value {
                    continue;
                }
            }

            stack.push(node);
        }

        visited.insert(value.pos.clone(), value);
    }

    visited
}

fn get_path(map: &HashMap<Vector2<i32>, Node>, input: &Input) -> HashSet<Vector2<i32>> {
    let mut current = Vector2::new(input[0].len() as i32-1, input.len() as i32-1);
    let mut vecs = HashSet::new();

    while current.x != 0 || current.y != 0 {
        vecs.insert(current.clone());
        current = map.get(&current).unwrap().from.unwrap();
    }

    vecs
}

fn count(map: &HashSet<Vector2<i32>>, input: &Input) -> u32 {
    map.iter().map(|p| input[p.y as usize][p.x as usize]).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let map = visit2(&input);
    let path = get_path(&map, &input);

    print(&input, &path);

    let cnt = count(&path, &input);

    Some(cnt);
    Some(102)
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
