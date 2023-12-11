use std::collections::HashSet;

use cgmath::Vector2;

type Input = Vec<Vec<bool>>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    }).collect()
}

fn expand(mut input: Input) -> Input {
    let lines = input.iter().enumerate().filter_map(|(i, l)| {
        if l.iter().all(|c| !*c) {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<usize>>();
    let cols = (0..input[0].len()).filter_map(|i| {
        if input.iter().all(|l| !l[i]) {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<usize>>();
    
    cols.iter().rev().for_each(|c| {
        input.iter_mut().for_each(|l| {
            l.insert(*c, false)
        })
    });
    lines.iter().rev().for_each(|l| {
        input.insert(*l, vec![false; input[0].len()])
    });

    input
}

fn get_paires(input: &Vec<Vector2<i32>>) -> HashSet<(Vector2<i32>, Vector2<i32>)> {
    let mut res = HashSet::new();

    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            res.insert((input[i], input[j]));
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let expanded = expand(input);

    let points = expanded.iter().enumerate().map(|(y, l)| {
        l.iter().enumerate().filter_map(|(x, c)| {
            if *c {
                Some(Vector2::new(x as i32, y as i32))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();

    let paires = get_paires(&points);

    let distances = paires.iter().map(|p| {
        let diff = p.1 - p.0;
        diff.x.abs() + diff.y.abs()
    });

    Some(distances.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, None);
    }
}
