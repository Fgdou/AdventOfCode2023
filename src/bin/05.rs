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


#[derive(Debug, PartialEq)]
struct InputRange {
    pub from: Num,
    pub to: Num,
}
#[derive(Debug)]
struct Input2 {
    pub seeds: Vec<InputRange>,
    pub maps: Vec<RangeMap>
}
trait RangeNextRange {
    fn next_range(&self, n: InputRange) -> Used;
}

#[derive(Debug, PartialEq)]
struct Used {
    used: Vec<InputRange>,
    not_used: Vec<InputRange>,
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

fn parse2(input: &str) -> Input2 {
    let mut lines = input.split("\n\n");
    let seeds = parse_seeds2(lines.next().unwrap());

    let maps = lines.map(|l| parse_map(l)).collect();
    Input2{seeds, maps}
}
fn parse_seeds2(input: &str) -> Vec<InputRange> {
    let input = &input[7..];
    input.split(" ")
        .map(|s| s.parse::<Num>().unwrap()).collect::<Vec<Num>>()
        .chunks(2)
        .map(|chunk| InputRange{from: chunk[0], to: chunk[0]+chunk[1]-1})
        .collect()
}

impl RangeNextRange for Range {
    fn next_range(&self, n: InputRange) -> Used {
        let left1 = self.from;
        let right1 = self.from + self.size-1;
        let left2 = n.from;
        let right2 = n.to;

        let mut res = Used{used: vec!(), not_used: vec!()};

        if left2 < left1 {
            res.not_used.push(InputRange { from: left2, to: right2.min(left1-1) });
        }
        if !(right2 < left1 || left2 > right1) {
            let from = left1.max(left2);
            let to = right1.min(right2);
            res.used.push(InputRange { from: from-self.from+self.to, to: to-self.from+self.to})
        }
        if right2 > right1 {
            res.not_used.push(InputRange { from: left2.max(right1+1), to: right2 });
        }

        res
    }
}
impl RangeNextRange for RangeMap {
    fn next_range(&self, n: InputRange) -> Used {
        let mut used = Used{used: vec!(), not_used: vec!(n)};

        for map in self {
            let mut other_used = Used{used: used.used, not_used: vec!()};

            used.not_used.into_iter().for_each(|n| {
                let mut res = map.next_range(n);
                other_used.used.append(&mut res.used);
                other_used.not_used.append(&mut res.not_used)
            });

            used = other_used;
        }

        used
    }
}

pub fn part_one(input: &str) -> Option<Num> {
    let input = parse(input);

    let seeds_end = input.seeds.iter()
        .map(|seed| {
            input.maps.iter()
                .fold(*seed, |n, map| map.next(n).unwrap_or(n))  
        });

    Some(
        seeds_end.min().unwrap()
    )
}

pub fn part_two(input: &str) -> Option<Num> {
    let input = parse2(input);

    let seeds_end = input.seeds.into_iter()
        .map(|seed| {
            input.maps.iter()
                .fold(vec!(seed), |n, map| {
                    n.into_iter()
                        .map(|n| {
                            let mut res = map.next_range(n);
                            let mut v = vec!();
                            v.append(&mut res.used);
                            v.append(&mut res.not_used);
                            v
                        })
                        .flatten()
                        .collect()
                }) 
        }).flatten();

    Some(
        seeds_end.map(|e| e.from).min().unwrap()
    )
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
    fn next_range() {
        let range = Range{from: 4, size: 4, to: 20};

        let input = InputRange{from: 0, to: 3};
        let expected = Used{not_used: vec!(InputRange{from: 0, to: 3}), used: vec!()};
        assert_eq!(expected, range.next_range(input));

        let input = InputRange{from: 8, to: 10};
        let expected = Used{not_used: vec!(InputRange{from: 8, to: 10}), used: vec!()};
        assert_eq!(expected, range.next_range(input));

        let input = InputRange{from: 4, to: 7};
        let expected = Used{used: vec!(InputRange{from: 20, to: 23}), not_used: vec!()};
        assert_eq!(expected, range.next_range(input));

        let input = InputRange{from: 0, to: 7};
        let expected = Used{not_used: vec!(InputRange{from: 0, to: 3}), used: vec!(InputRange{from: 20, to: 23})};
        assert_eq!(expected, range.next_range(input));

        let input = InputRange{from: 4, to: 10};
        let expected = Used{used: vec!(InputRange{from: 20, to: 23}), not_used: vec!(InputRange{from: 8, to: 10})};
        assert_eq!(expected, range.next_range(input));

        let input = InputRange{from: 0, to: 10};
        let expected = Used{not_used: vec!(InputRange{from: 0, to: 3}, InputRange{from: 8, to: 10}), used: vec!(InputRange{from: 20, to: 23})};
        assert_eq!(expected, range.next_range(input));
    }

    #[test]
    fn test_part2_simple() {
        let input = "seeds: 0 10

aaa:
20 0 11";
        assert_eq!(part_two(input), Some(20));
    }

    #[test]
    fn test_range_vec() {
        let vec = vec!(Range{from: 4, to: 20, size: 4});
        let expected = Used{not_used: vec!(InputRange{from: 0, to: 3}, InputRange{from: 8, to: 10}), used: vec!(InputRange{from: 20, to: 23})};
        let input = InputRange{from: 0, to: 10};

        assert_eq!(expected, vec.next_range(input));
    }

    #[test]
    fn test_range_vec_double() {
        let vec = vec!(Range{from: 4, to: 20, size: 4}, Range{from: 9, to: 30, size: 1});
        let expected = Used {
            not_used: vec!(
                InputRange{from: 0, to: 3}, 
                InputRange{from: 8, to: 8},
                InputRange{from: 10, to: 10}),
            used: vec!(
                InputRange{from: 20, to: 23}, 
                InputRange{from: 30, to: 30}, 
            )
        };
        let input = InputRange{from: 0, to: 10};

        assert_eq!(expected, vec.next_range(input));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}


// 286952859
// 278755257

// 26829166