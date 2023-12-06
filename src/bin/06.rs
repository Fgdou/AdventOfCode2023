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
fn solve_equation(t_max: u64, target: u64) -> Range<u64> {
    let t_max = t_max as f64;
    let n = target as f64;

    let delta = t_max.powi(2)-4.0*n;
    let detla_sqrt = delta.sqrt();

    let left = (t_max - detla_sqrt)/2.0+0.01;
    let right = (t_max + detla_sqrt)/2.0-0.01;

    left.ceil().max(0.0) as u64..right.ceil().min(t_max) as u64
}

#[derive(Debug)]
struct Pair {
    time: u64,
    distance: u64
}

fn parse(input: &str) -> Vec<Pair> {
    let lines = input.lines()
        .map(|l| l.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>();

    (0..lines[0].len()).into_iter()
        .map(|i| Pair{time: lines[0][i], distance: lines[1][i]})
        .collect()
}
fn parse2(input: &str) -> Pair {
    let lines = input.lines()
        .map(|l| l.split_whitespace()
            .skip(1)
            .fold("".to_string(), |s1, s2| s1+s2)
        )
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    Pair{time: lines[0], distance: lines[1]}
}


pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let ranges = input.iter()
        .map(|race| solve_equation(race.time, race.distance));
    let count = ranges.map(|r| r.end - r.start).product();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse2(input);
    let range = solve_equation(input.time, input.distance);
    Some(range.end - range.start)
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
        assert_eq!(result, Some(71503));
    }
}

// 800280

// 45128024