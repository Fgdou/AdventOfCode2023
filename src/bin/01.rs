fn get_numbers(s: &str) -> Vec<u32> {
    s.chars()
    .into_iter()
    .filter_map(|c| c.to_digit(10))
    .collect::<Vec<u32>>()
}

fn get_line(nums: Vec<u32>) -> u32 {
    let first = nums.first().unwrap();
    let last = nums.last().unwrap();
    first*10 + last
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| get_numbers(line))
            .map(|line| get_line(line))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use crate::get_numbers;

    use super::*;

    #[test]
    fn test_get_numbers() {
        let s = "abc16597asdsfasdf150df0";
        let expected = vec!(1,6,5,9,7,1,5,0,0);

        assert_eq!(expected, get_numbers(s))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, None);
    }
}
