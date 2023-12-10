use std::collections::{HashMap, HashSet};

use cgmath::{Vector2, Zero};

#[derive(Debug)]
struct Pipe {
    one:  Vector2<i32>,
    two: Vector2<i32>,
    letter: char
}
#[derive(Debug)]
struct Maze {
    maze: HashMap<Vector2<i32>, Pipe>,
    start: Vector2<i32>,
    size: Vector2<i32>,
}

impl Maze {
    fn print(&self) -> String {
        (0..self.size.y).map(|y| {
            (0..self.size.x).map(|x| {
                match self.maze.get(&Vector2::new(x, y)) {
                    Some(n) => n.letter,
                    None => '.'
                }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n")
    }
}

fn parse(input: &str) -> Maze {        
    let mut res = Maze{
        start: Vector2::zero(), 
        maze: HashMap::new(),
        size: Vector2::new(input.lines().next().unwrap().len() as i32,input.lines().count() as i32)
    };

    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i32;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i32;
            let pos = Vector2::new(x, y);
            match c {
                '|' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x, y-1), 
                    two: Vector2::new(x, y+1),
                    letter: c,
                });},
                '-' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x-1, y), 
                    two: Vector2::new(x+1, y),
                    letter: c,
                });},
                'F' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x+1, y), 
                    two: Vector2::new(x, y+1),
                    letter: c,
                });},
                '7' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x-1, y), 
                    two: Vector2::new(x, y+1),
                    letter: c,
                });},
                'L' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x+1, y), 
                    two: Vector2::new(x, y-1),
                    letter: c,
                });},
                'J' => {res.maze.insert(pos, Pipe { 
                    one: Vector2::new(x-1, y), 
                    two: Vector2::new(x, y-1),
                    letter: c,
                });},
                'S' => {
                    res.start = pos;
                },
                _ => (),
            };
        })
    });

    let start = res.start;
    let iter = [
        Vector2::new(start.x, start.y-1), 
        Vector2::new(start.x, start.y+1), 
        Vector2::new(start.x-1, start.y),
        Vector2::new(start.x+1, start.y)
    ];
    let mut poses = iter.into_iter()
        .filter_map(|i| Some((res.maze.get(&i)?, i)))
        .filter_map(|(i,origin)| {
            if i.one == start || i.two == start {
                Some(origin)
            } else {
                None
            }
        });
    assert!(poses.clone().count() == 2);
    res.maze.insert(start, Pipe { one: poses.next().unwrap(), two: poses.next().unwrap(), letter: 'S'});

    res
}

fn next_pos(pos: &Vector2<i32>, next: &Pipe) -> Vector2<i32> {
    if &next.one == pos {
        next.two.clone()
    } else {
        next.one.clone()
    }
}
fn get_pipe_pos(maze: &Maze) -> Vec<Vector2<i32>> {
    let mut pos = maze.start;
    let mut next = maze.maze.get(&pos).unwrap().one;

    let mut set = Vec::new();

    loop {
        set.push(pos);
        let next_pipe = maze.maze.get(&next).unwrap();
        let next_pipe_pos = next_pos(&pos, next_pipe);
        pos = next;
        next = next_pipe_pos;

        if pos == maze.start {
            break;
        }
    }

    set
}

fn run_outside(pos: Vector2<i32>, size: &Vector2<i32>, border: &HashSet<Vector2<i32>>, visited: &mut HashSet<Vector2<i32>>) {
    if pos.x < -1 || pos.y < -1 || pos.x > size.x || pos.y > size.y {
        return
    }
    if visited.contains(&pos) {
        return
    }
    if border.contains(&pos) {
        return
    }
    visited.insert(pos.clone());
    
    let nexts = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ].into_iter()
        .map(|(x, y)| Vector2::new(x, y))
        .map(|v| pos + v);

    nexts.into_iter().for_each(|v| {
        run_outside(v, size, border, visited);
    });
}
fn double_maze(pipes: &Vec<Vector2<i32>>) -> Vec<Vector2<i32>> {
    let mut res = Vec::new();
    
    (0..pipes.len()).for_each(|i| {
        let p1 = pipes[i]*2;
        let p2 = pipes[(i+1) % pipes.len()]*2;
        let mid = (p1+p2)/2;
        res.push(p1);
        res.push(mid);
    });

    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let cnt = get_pipe_pos(&input).len() as u32;

    Some(cnt/2)
}

fn print_set(vec: &HashSet<Vector2<i32>>, size: &Vector2<i32>) -> String {
    (0..size.y).map(|y| {
        (0..size.x).map(|x| {
            match vec.contains(&Vector2::new(x, y)) {
                true => '*',
                false => '.'
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n")
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let pipes = get_pipe_pos(&input);
    let doubled_pipes = double_maze(&pipes);
    let doubled_size = input.size*2;

    let mut visited = HashSet::new();
    run_outside(Vector2::new(-1, -1), &doubled_size, &doubled_pipes.into_iter().collect(), &mut visited);

    let divided = visited.into_iter()
        .filter_map(|v| {
            if v.x%2 == 0 && v.y%2 == 0 {
                Some(v/2)
            }else{
                None
            }
        })
        .collect::<HashSet<_>>();

    let all = (0..input.size.y).into_iter().map(|y| {
        (0..input.size.x).into_iter().map(|x| {
            Vector2::new(x, y)
        }).collect::<HashSet<_>>()
    }).flatten()
    .filter(|v| !divided.contains(v))
    .filter(|v| !pipes.contains(v))
    .collect::<HashSet<_>>();

    Some(all.len() as u32)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(".....
.S-7.
.|.|.
.L-J.
.....");
        assert_eq!(result, Some(4));
        let result = part_one("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");
        assert_eq!(result, Some(4));
        let result = part_two("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
        assert_eq!(result, Some(10));
    }
}
