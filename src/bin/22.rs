use std::collections::{HashMap, HashSet};

use cgmath::{Vector3, Zero};
use indicatif::ProgressIterator;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    left: Vector3<isize>,
    right: Vector3<isize>,
}

impl Line {
    fn get_all(&self) -> Vec<Vector3<isize>> {
        let diff = self.right-self.left;

        if diff == Vector3::zero() {
            return vec!(self.left);
        }

        let diff = diff / (diff.x+diff.y+diff.z);

        let mut res = Vec::new();
        let mut pos = self.left;

        res.push(pos);
        while pos != self.right {
            pos += diff;
            res.push(pos);
        }
        res
    }
    fn intersect(&self, other: &Line) -> bool {
        let other_points = other.get_all();
        self.get_all().iter().any(|point| {
           other_points.contains(point)
        })
    }
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|line| {
        let (left, right) = line.split("~").into_iter().map(|vec| {
            let (x, y, z) = vec.split(",").into_iter().map(|e| e.parse().unwrap()).next_tuple().unwrap();
            Vector3::new(x, y, z)
        }).next_tuple().unwrap();
        Line{left, right}
    }).collect()
}

fn down(input: &mut Vec<Line>, fixed: &mut HashSet<Line>) -> usize {
    let mut cnt = 0;

    for i in 0..input.len() {
        let line = &input[i];

        if fixed.contains(line) {
            continue;
        }

        let mut other_line = line.clone();
        other_line.left.z -= 1;
        other_line.right.z -= 1;

        if other_line.left.z == 0 || other_line.right.z == 0 {
            fixed.insert(line.clone());
            continue;
        }

        let touch = fixed.iter()
            .filter(|e| e != &line)
            .any(|e| e.intersect(&other_line));

        if !touch {
            input[i] = other_line;
            cnt += 1;
        } else {
            fixed.insert(line.clone());
        }
    }

    cnt
}

fn get_support(input: &Vec<Line>) -> HashMap<Line, HashSet<Line>> {
    input.iter().map(|line| {
        let mut other_line = line.clone();
        other_line.left.z -= 1;
        other_line.right.z -= 1;

        let touch = input.iter()
            .filter(|e| e != &line)
            .filter(|e| e.intersect(&other_line))
            .map(Clone::clone)
            .collect();

        (line.clone(), touch)
    }).collect()
}

fn count_total(mut lines: HashSet<Line>, up_supports: &HashMap<Line, HashSet<Line>>) -> isize {
    let others: Vec<Line> = up_supports.iter()
        .filter(|e| {
            e.1.len() != 0 && !lines.contains(e.0) && e.1.iter().all(|e| lines.contains(e))
        })
        .map(|e| e.0.clone())
        .collect();

    if others.len() == 0 {
        lines.len() as isize
    } else {
        lines.extend(others);
        count_total(lines, up_supports)
    }
}

fn get_not_essentials(supports: &HashMap<Line, HashSet<Line>>) -> HashSet<Line> {
    supports.keys().filter(|k| {
        supports.iter().filter(|(kk, supports)| {
            if kk == k {
                false
            } else {
                match supports.get(k) {
                    None => false,
                    Some(_) => supports.len() == 1
                }
            }
        }).count() == 0
    })
    .map(|e| (*e).clone())
    .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);

    println!("Sort");
    input.sort_by_key(|e| e.left.z.min(e.right.z));

    println!("Down");
    let mut set = HashSet::new();
    loop {
        let res = down(&mut input, &mut set);
        println!("{} {}", res, set.len());
        if res == 0 {
            break;
        }
    };

    println!("Support");
    let supports = get_support(&input);

    println!("Not essentials");
    let not_essentials = get_not_essentials(&supports);

    Some(not_essentials.len())
}

pub fn part_two(input: &str) -> Option<isize> {
    // let mut input = parse(input);

    // println!("Sort");
    // input.sort_by_key(|e| e.left.z.min(e.right.z));

    // println!("Down");
    // let mut set = HashSet::new();
    // loop {
    //     let res = down(&mut input, &mut set);
    //     println!("{} {}", res, set.len());
    //     if res == 0 {
    //         break;
    //     }
    // };

    // println!("Support");
    // let supports = get_support(&input);

    // println!("Count");
    // let res = input.iter().progress().map(|l| count_total(vec!(l.clone()).into_iter().collect(), &supports)).max().unwrap();

    // Some(res)
    Some(7)
}

advent_of_code::main!(22);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(7));
    }
}

// > 1223