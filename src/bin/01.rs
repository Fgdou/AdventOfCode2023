const NUMBERS_STRING: [&str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];

fn parse_string_to_numbers(s: &str) -> Vec<u32> {
    s.chars()
    .filter_map(|c| c.to_digit(10))
    .collect::<Vec<u32>>()
}

fn parse_string_to_number(s: &str) -> u32 {
    let i = NUMBERS_STRING.iter().position(|n| n == &s).unwrap();
    (i as u32%9) + 1
}

fn parse_string_to_numbers_2(s: &str) -> Vec<u32> {
    let mut indices: Vec<(usize, &str)> = NUMBERS_STRING.iter()
        .map(|n| s.match_indices(n))
        .flatten()
        .collect();
    indices.sort_by_key(|e| e.0);
    indices.iter().map(|n| parse_string_to_number(n.1)).collect::<Vec<u32>>()
}

fn get_number_on_line(nums: Vec<u32>) -> u32 {
    let first = nums.first().unwrap();
    let last = nums.last().unwrap();
    first*10 + last
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| parse_string_to_numbers(line))
            .map(|line| get_number_on_line(line))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_string_to_numbers_2(line))
        .map(|num| get_number_on_line(num))
        .sum()
    )
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use crate::parse_string_to_numbers;

    use super::*;

    #[test]
    fn test_parse_string_to_numbers() {
        let s = "abc16597asdsfasdf150df0";
        let expected = vec!(1,6,5,9,7,1,5,0,0);

        assert_eq!(expected, parse_string_to_numbers(s))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#);
        assert_eq!(result, Some(281));
    }
}
