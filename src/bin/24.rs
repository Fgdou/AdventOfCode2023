use std::collections::{HashMap, HashSet};

use cgmath::{Vector3, Vector2, InnerSpace, Zero};
use num::ToPrimitive;
use num_bigint::{BigInt, Sign, ToBigInt};

type Input = Vec<Stone>;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Stone {
    pos: Vector3<isize>,
    vel: Vector3<isize>
}

impl Stone {
    fn intersect2(&self, other: &Stone) -> Option<Vector2<BigInt>> {
        let x1 = BigInt::from(self.pos.x);
        let y1 = BigInt::from(self.pos.y);
        let x2 = BigInt::from(self.pos.x+self.vel.x);
        let y2 = BigInt::from(self.pos.y+self.vel.y);
        let x3 = BigInt::from(other.pos.x);
        let y3 = BigInt::from(other.pos.y);
        let x4 = BigInt::from(other.pos.x+other.vel.x);
        let y4 = BigInt::from(other.pos.y+other.vel.y);

        let npx = (&x1*&y2-&y1*&x2)*(&x3-&x4)-(&x1-&x2)*(&x3*&y4-&y3*&x4);
        let npy = (&x1*&y2-&y1*&x2)*(&y3-&y4)-(&y1-&y2)*(&x3*&y4-&y3*&x4);
        let d = (&x1-&x2)*(&y3-&y4)-(&y1-&y2)*(&x3-&x4);

        if d == BigInt::zero() {
            return None;
        }

        let px = npx/&d;
        let py = npy/&d;

        Some(Vector2::new(px, py))
    }
    fn intersect2_forward(&self, other: &Stone) -> Option<Vector2<BigInt>> {
        let i = self.intersect2(other)?;
        let dir1 = Vector2::new(self.vel.x.to_bigint().unwrap(), self.vel.y.to_bigint().unwrap());
        let dir2 = Vector2::new(&i.x - self.pos.x.to_bigint().unwrap(), &i.y - self.pos.y.to_bigint().unwrap());

        if dir1.x*dir2.x+dir1.y*dir2.y > BigInt::zero() {
            Some(i)
        } else {
            None
        }
    }
    fn intersect2_boundaries(&self, other: &Stone, start: isize, end: isize) -> Option<Vector2<BigInt>> {
        let i = self.intersect2_forward(other)?;
        let start = start.to_bigint().unwrap();
        let end = end.to_bigint().unwrap();

        if i.x >= start && i.x <= end && i.y >= start && i.y <= end {
            Some(i)
        } else {
            None
        }
    }
}

fn get_vec(input: &str) -> Vector3<isize> {
    let input = input.replace(" ", "");
    let mut values = input.split(",");
    let x = values.next().unwrap().parse().unwrap();
    let y = values.next().unwrap().parse().unwrap();
    let z = values.next().unwrap().parse().unwrap();
    Vector3::new(x, y, z)
}

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let pos = get_vec(pos);
        let vel = get_vec(vel);
        Stone{
            pos, vel
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let boundaries = if input.len() > 10 {
        (200000000000000, 400000000000000)
    } else {
        (7, 27)
    };

    let intersections = input.iter().map(|v1| {
        let mut res = (v1.clone(), input.iter().filter_map(|v2| {
            if v1 == v2 {
                None
            } else {
                if let Some(i) = v1.intersect2_boundaries(v2, boundaries.0, boundaries.1) {
                    let x = (i.x-v1.pos.x).to_bigint().unwrap();
                    let y = (i.y-v1.pos.y).to_bigint().unwrap();
                    Some((v2.clone(), &x*&x+&y*&y))
                } else {
                    None
                }
            }
        }).collect::<Vec<(Stone, BigInt)>>());
        res.1.sort_by_key(|e| e.1.clone());
        res
    }).collect::<Vec<(Stone,Vec<(Stone, BigInt)>)>>();

    let mut used: HashSet<Stone> = HashSet::new();

    let mut cnt = 0;

    for (v1, map) in intersections {
        if used.contains(&v1) {
            continue;
        }
        let v2 = map.iter().find(|e| !used.contains(&e.0));

        if let Some(v2) = v2 {
            used.insert(v1.clone());
            used.insert(v2.0.clone());
            cnt += 1;
        } else {
            continue;
        }
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, None);
    }
}

// 150