use std::collections::HashSet;

use cgmath::Vector2;

type Input = Vec<Vec<bool>>;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    }).collect()
}

fn expand2(mut points: Vec<Vector2<usize>>, input: &Input, factor: usize) -> Vec<Vector2<usize>> {
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
    
    points.iter_mut().for_each(|p| {
        let count_lines = lines.iter().filter(|l| p.y >= **l).count();
        let count_cols = cols.iter().filter(|c| p.x >= **c).count();

        p.x += factor*count_cols;
        p.y += factor*count_lines;
    });

    points
}

fn get_paires(input: &Vec<Vector2<usize>>) -> HashSet<(Vector2<usize>, Vector2<usize>)> {
    let mut res = HashSet::new();

    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            res.insert((input[i], input[j]));
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let points = input.iter().enumerate().map(|(y, l)| {
        l.iter().enumerate().filter_map(|(x, c)| {
            if *c {
                Some(Vector2::new(x as usize, y as usize))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();

    let points2 = expand2(points, &input, 1);

    let paires = get_paires(&points2);

    let distances = paires.iter().map(|p| {
        let diff = Vector2::new(p.1.x.abs_diff(p.0.x), p.1.y.abs_diff(p.0.y));
        diff.x + diff.y
    });

    Some(distances.sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let points = input.iter().enumerate().map(|(y, l)| {
        l.iter().enumerate().filter_map(|(x, c)| {
            if *c {
                Some(Vector2::new(x as usize, y as usize))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();

    let points2 = expand2(points, &input, 1000000-1);

    let paires = get_paires(&points2);

    let distances = paires.iter().map(|p| {
        let diff = Vector2::new(p.1.x.abs_diff(p.0.x), p.1.y.abs_diff(p.0.y));
        diff.x + diff.y
    });

    Some(distances.sum())
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
        assert_eq!(result, Some(82000210));
    }
}


// < 702771271959