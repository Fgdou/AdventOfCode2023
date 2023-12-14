use std::collections::HashMap;

use indicatif::ProgressIterator;

type Input = Vec<Vec<char>>;

enum Direction {
    North,
    South,
    East,
    West
}

fn parse(input: &str) -> Input {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn roll(tab: &mut Input, dir: Direction) -> usize {
    let mut count = 0;
    for i in 0..tab.len() {
        for j in 0..tab[0].len() {
            let c = tab[i][j];

            if c != 'O' {
                continue;
            }
            
            let new_pos = match dir {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };
            let new_pos = (new_pos.0 + j as i32, new_pos.1 + i as i32);

            if new_pos.0 < 0 || new_pos.0 >= tab[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= tab.len() as i32 {
                continue;
            }

            if tab[new_pos.1 as usize][new_pos.0 as usize] == '.' {
                tab[new_pos.1 as usize][new_pos.0 as usize] = 'O';
                tab[i][j] = '.';
                count += 1;
            }
        }
    }
    count
}

fn weights(tab: &Input) -> usize {
    let mut cnt = 0;
    for i in 0..tab.len() {
        for j in 0..tab[0].len() {
            let y = tab.len()-i;
            if tab[i][j] == 'O' {
                cnt += y;
            }
        }
    }
    cnt
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);

    while roll(&mut input, Direction::North) != 0 {}

    Some(weights(&input))
}

fn cycle(input: &mut Input) {
    while roll(input, Direction::North) != 0 {};
    while roll(input, Direction::West) != 0 {};
    while roll(input, Direction::South) != 0 {};
    while roll(input, Direction::East) != 0 {};
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = parse(input);
    let mut cache = HashMap::new();

    for i in (0..1000000000).into_iter().progress() {
        if let Some(n) = cache.get(&input) {
            let diff = n - i;

            let rest = 1000000000 - i;
            let count = rest % diff;

            for _ in 0..count {
                cycle(&mut input);
            }

            break;
        }

        cache.insert(input.clone(), i);

        cycle(&mut input);
    }

    Some(weights(&input))
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(64));
    }
}
