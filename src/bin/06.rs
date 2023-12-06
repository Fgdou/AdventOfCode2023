use std::ops::Range;

/// Equations to solve :
///     speed = t_button
///     distance = speed*(t_max - t_button)
///     t_button < t_max
///     distance > target
/// 
/// => t_button*t_max - t_button^2 - target > 0
/// 
/// solution => (t_max - sqrt(t_max^2 - 4*target))/2 < b < (t_max + sqrt(t_max^2 - 4*target))/2
fn solve_equation(t_max: u32, target: u32) -> Range<u32> {
    let t_max = t_max as f32;
    let n = target as f32;

    let delta = t_max.powi(2)-4.0*n;
    let detla_sqrt = delta.sqrt();

    let left = (t_max - detla_sqrt)/2.0+0.01;
    let right = (t_max + detla_sqrt)/2.0-0.01;

    left.ceil().max(0.0) as u32..right.ceil().min(t_max) as u32
}

#[derive(Debug)]
struct Pair {
    time: u32,
    distance: u32
}

fn parse(input: &str) -> Vec<Pair> {
    let lines = input.lines()
        .map(|l| l.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>();

    (0..lines[0].len()).into_iter()
        .map(|i| Pair{time: lines[0][i], distance: lines[1][i]})
        .collect()
}


pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let ranges = input.iter()
        .map(|race| solve_equation(race.time, race.distance));
    let count = ranges.map(|r| r.len() as u32).product();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, None);
    }
}

// 800280