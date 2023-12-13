use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use indicatif::ProgressIterator;

type Input = Vec<Vec<Vec<bool>>>;

fn parse(input: &str) -> Input {
    input.split("\n\n").map(|tab| {
        tab.lines().map(|l| l.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => unreachable!()
            }
        }).collect()).collect()
    }).collect()
}

fn get_hash_line(v: &Vec<Vec<bool>>, i: usize) -> u64 {
    let mut hash = DefaultHasher::new();
    v[i].hash(&mut hash);
    hash.finish()
}

fn get_hash_column(v: &Vec<Vec<bool>>, i: usize) -> u64 {
    let mut hash = DefaultHasher::new();
    v.iter().map(|v| v[i]).collect::<Vec<bool>>().hash(&mut hash);
    hash.finish()
}

// 1 0 0 1
// 0 1 2 3

fn is_mirror(hash: &Vec<u64>, i: usize) -> bool {
    let dist = if i < hash.len()-2 {hash.len()-2-i} else {0};

    if i >= hash.len()-1 {
        return false
    }

    for j in 0..=i.min(dist) {
        if hash[i-j] != hash[i+j+1] {
            return false
        }
    }
    return true
}

fn find_mirror(hash: &Vec<u64>) -> Option<usize> {
    for i in 0..=hash.len() {
        if is_mirror(hash, i) {
            return Some(i)
        }
    }
    None
}

fn print(input: &Vec<Vec<bool>>) {
    for i in input {
        for j in i {
            print!("{}", if *j {"#"} else {"."})
        }
        println!()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let res = input.iter().progress().map(|grid| {
        let lines: Vec<u64> = (0..grid.len()).into_iter().map(|i| get_hash_line(&grid, i)).collect();
        let cols: Vec<u64> = (0..grid[0].len()).into_iter().map(|i| get_hash_column(&grid, i)).collect();

        let line = find_mirror(&lines);
        let col = find_mirror(&cols);

        match (line, col) {
            (None, None) => {
                print(grid);
                println!("{:?}", cols);
                unreachable!()
            },
            (None, Some(x)) => x+1,
            (Some(x), None) => 100*(x+1),
            (Some(_), Some(_)) => unreachable!(),
        }
    })
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, None);
    }

    #[test]
    fn test_is_mirror() {
        assert_eq!(true, is_mirror(&vec!(0, 0), 0));
        assert_eq!(false, is_mirror(&vec!(0, 0), 1));
        assert_eq!(false, is_mirror(&vec!(0, 1), 0));
        assert_eq!(true, is_mirror(&vec!(1, 0, 0, 1), 1));
        assert_eq!(false, is_mirror(&vec!(1, 0, 0, 1), 2));
        assert_eq!(false, is_mirror(&vec!(1, 1, 0, 0, 1, 1), 1));
        assert_eq!(true, is_mirror(&vec!(1, 1, 0, 0, 1, 1), 2));
        assert_eq!(false, is_mirror(&vec!(1, 1, 0, 0, 1, 1), 3));
        assert_eq!(false, is_mirror(&vec!(1, 1, 1, 0, 0, 1, 1), 2));
        assert_eq!(true, is_mirror(&vec!(1, 1, 1, 0, 0, 1, 1), 3));
        assert_eq!(false, is_mirror(&vec!(1, 1, 1, 0, 0, 1, 1), 4));
        assert_eq!(false, is_mirror(&vec!(1, 0, 1, 0, 0, 1, 1), 3));
        assert_eq!(true, is_mirror(&vec!(1, 0, 1, 0, 0, 2, 2), 5));
        assert_eq!(true, is_mirror(&vec!(5, 5, 1, 0, 0, 2, 2), 0));
        assert_eq!(true, is_mirror(&vec!(5, 5, 1, 0, 0, 2), 0));
        assert_eq!(true, is_mirror(&vec!(5, 5, 0, 0, 2), 0));
        assert_eq!(true, is_mirror(&vec!(5, 5, 0, 2), 0));
        assert_eq!(true, is_mirror(&vec!(5, 5, 2), 0));
        assert_eq!(true, is_mirror(&vec!(5, 5), 0));
        assert_eq!(true, is_mirror(&vec!(5, 6, 6, 5, 0, 2, 2), 1));
        assert_eq!(true, is_mirror(&vec!(5, 5, 6, 6, 5, 5, 2), 2));
        assert_eq!(true, is_mirror(&vec!(2, 5, 5, 6, 6, 5, 5), 3));
        assert_eq!(true, is_mirror(&vec!(8478844302057451079, 8478844302057451079, 15134898337082185605, 14152414884309465293, 14451582079670384677, 6391166416506382924, 16063916804375491602, 12091216157700848071, 17418744119211472666, 15134898337082185605, 7855126870682793489), 0));
    }
}

// 28453