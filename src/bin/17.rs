use cgmath::{Vector2, Zero};
use pathfinding::directed::dijkstra::dijkstra;

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
    dir: Option<Direction>,
    straight: u32
}


pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let success = |p: &Node| {
        p.pos.x == input[0].len() as i32-1 && p.pos.y == input.len() as i32-1
    };
    let successors = |p: &Node| {
        let mut l = Vec::new();

        match &p.dir {
            None => {
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Right),
                    dir: Some(Direction::Right)
                });
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Down),
                    dir: Some(Direction::Down)
                });
            },
            Some(Direction::Down) | Some(Direction::Up) => {
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Right),
                    dir: Some(Direction::Right)
                });
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Left),
                    dir: Some(Direction::Left)
                });
            },
            Some(Direction::Left) | Some(Direction::Right) => {
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Up),
                    dir: Some(Direction::Up)
                });
                l.push(Node {
                    straight: 1,
                    pos: move_vec(&p.pos, &Direction::Down),
                    dir: Some(Direction::Down)
                });
            },
        }

        if let Some(dir) = &p.dir  {
            if p.straight < 3 {
                l.push(Node {
                    straight: p.straight+1,
                    pos: move_vec(&p.pos, &dir),
                    dir: Some(dir.clone())
                })
            }
        }

        l.into_iter().filter(|e| {
            e.pos.x >= 0 && e.pos.y >= 0 && e.pos.x < input[0].len() as i32 && e.pos.y < input.len() as i32
        })
        .map(|e| {
            let v = input[e.pos.y as usize][e.pos.x as usize];
            (e, v)
        })
        .collect::<Vec<(_,_)>>()
    };

    let res = dijkstra(&Node {
        dir: None,
        pos: Vector2::zero(),
        straight: 1
    }, successors, success);

    res.map(|e| e.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let success = |p: &Node| {
        p.pos.x == input[0].len() as i32-1 && p.pos.y == input.len() as i32-1
    };
    let successors = |p: &Node| {
        let mut l = Vec::new();

        if p.straight > 3 || p.dir.is_none() {
            match &p.dir {
                None => {
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Right),
                        dir: Some(Direction::Right)
                    });
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Down),
                        dir: Some(Direction::Down)
                    });
                },
                Some(Direction::Down) | Some(Direction::Up) => {
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Right),
                        dir: Some(Direction::Right)
                    });
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Left),
                        dir: Some(Direction::Left)
                    });
                },
                Some(Direction::Left) | Some(Direction::Right) => {
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Up),
                        dir: Some(Direction::Up)
                    });
                    l.push(Node {
                        straight: 1,
                        pos: move_vec(&p.pos, &Direction::Down),
                        dir: Some(Direction::Down)
                    });
                },
            }
        }

        if let Some(dir) = &p.dir  {
            if p.straight < 10 {
                l.push(Node {
                    straight: p.straight+1,
                    pos: move_vec(&p.pos, &dir),
                    dir: Some(dir.clone())
                })
            }
        }

        l.into_iter().filter(|e| {
            e.pos.x >= 0 && e.pos.y >= 0 && e.pos.x < input[0].len() as i32 && e.pos.y < input.len() as i32
        })
        .map(|e| {
            let v = input[e.pos.y as usize][e.pos.x as usize];
            (e, v)
        })
        .collect::<Vec<(_,_)>>()
    };

    let res = dijkstra(&Node {
        dir: None,
        pos: Vector2::zero(),
        straight: 1
    }, successors, success);

    res.map(|e| e.1)
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
        assert_eq!(result, Some(94));
    }
}

// > 973