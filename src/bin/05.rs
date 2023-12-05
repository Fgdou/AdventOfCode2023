type Num = u64;

type Seeds = Vec<Num>;

trait NextRange {
    fn next(&self, n: Num) -> Option<Num>;
}

#[derive(Debug)]
struct Range {
    pub from: Num,
    pub to: Num,
    pub size: Num
}

type RangeMap = Vec<Range>;

#[derive(Debug)]
struct Input {
    pub seeds: Seeds,
    pub maps: Vec<RangeMap>,
}

impl NextRange for Range {
    fn next(&self, n: Num) -> Option<Num> {
        if n < self.from {
            return None;
        }
        let diff = n - self.from;
        if diff < self.size {
            Some(self.to + (n-self.from))
        } else {
            None
        }
    }
}
impl NextRange for Vec<Range> {
    fn next(&self, n: Num) -> Option<Num> {
        self.iter().find_map(|e| e.next(n))
    }
}

fn parse_seeds(input: &str) -> Seeds {
    let input = &input[7..];
    input.split(" ").map(|s| s.parse::<Num>().unwrap()).collect()
}

fn parse_map(input: &str) -> RangeMap {
    let mut lines = input.lines();
    lines.next().unwrap();

    lines.map(|l| {
        let nums = l.split(" ").map(|n| n.parse::<Num>().unwrap()).collect::<Vec<Num>>();
        assert!(nums.len() == 3);
        Range {
            from: nums[1],
            to: nums[0],
            size: nums[2]
        }
    }).collect::<RangeMap>()
}

fn parse(input: &str) -> Input {
    let mut lines = input.split("\n\n");
    let seeds = parse_seeds(lines.next().unwrap());

    let maps = lines.map(|l| parse_map(l)).collect();
    Input{seeds, maps}
}

pub fn part_one(input: &str) -> Option<Num> {
    let input = parse(input);

    dbg!(&input);

    let seeds_end = input.seeds.iter()
        .map(|seed| {
            input.maps.iter()
                .fold(*seed, |n, map| map.next(n).unwrap_or(n))  
        });

    Some(
        seeds_end.min().unwrap()
    )
}

pub fn part_two(_input: &str) -> Option<Num> {
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


// 286952859
// 278755257