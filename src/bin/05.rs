type num = u32;

type Seeds = Vec<num>;

struct Range {
    pub from: num,
    pub to: num,
    pub size: num
}

type map = Vec<Range>;
struct Input {
    pub seeds: Seeds,
    pub maps: Vec<map>,
}

fn parse_seeds(input: &str) -> Seeds {
    let input = &input[7..];
    input.split(" ").map(|s| s.parse::<num>().unwrap()).collect()
}

fn parse_map(input: &str) -> map {
    let lines = input.lines();

    lines.map(|l| {
        let nums = l.split(" ").map(|n| n.parse::<num>().unwrap()).collect::<Vec<num>>();
        assert!(nums.len() == 3);
        Range {
            from: nums[1],
            to: nums[0],
            size: nums[2]
        }
    }).collect::<map>()
}

fn parse(input: &str) -> Input {
    let mut lines = input.split("\n\n");
    let seeds = parse_seeds(lines.next().unwrap());
    lines.next().unwrap();

    let maps = lines.map(|l| parse_map(l)).collect();
    Input{seeds, maps}
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, None);
    }
}
