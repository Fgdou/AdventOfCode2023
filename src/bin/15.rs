fn hash(string: &str) -> u64 {
    string.chars().fold(0, |total, c| {
        if c == '\n' {return total}
        if !c.is_ascii() {
            panic!("Not ascii")
        }
        let n = c as u64;
        ((total + n)* 17) % 256
    })
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let res = input.iter().map(|i| hash(i)).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, None);
    }

    #[test]
    fn test_hash() {
        assert_eq!(30, hash("rn=1"))
    }
}

// 506805
// 506891