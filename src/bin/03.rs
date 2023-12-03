use std::collections::HashSet;

type Map = Vec<Vec<char>>;

#[derive(PartialEq, Debug, Hash, Eq)]
struct Pair {
    x: usize,
    y: usize
}

pub fn parse(input: &str) -> Map {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited = HashSet::<Pair>::new();
    let input = parse(input);

    let mut connected_num = Vec::<u32>::new();
    
    for i in input.iter().enumerate() {
        for j in 0..i.1.len() {
            let mut n = j;
            let mut number = 0;

            let mut connected = false;

            loop {
                let pos = Pair{x: n, y: i.0};
                let char = i.1[n];

                if visited.contains(&pos) {
                    break;
                }

                if let Some(num) = char.to_digit(10) {
                    number *= 10;
                    number += num;
                } else {
                    break;
                }

                for k in i.0 as i32-1..=i.0 as i32+1 {
                    for l in n as i32-1..=n as i32+1 {
                        if k == i.0 as i32 && l == n as i32 || k < 0 || l < 0 || k >= input.len() as i32 || l >= i.1.len() as i32 {
                            continue;
                        }

                        let c = input[k as usize][l as usize];
                        if !c.is_numeric() && c != '.' {
                            connected = true;
                        }
                    }
                }


                visited.insert(pos);



                if n+1 >= i.1.len() {
                    break;
                }else{
                    n += 1;
                }
            }

            if connected {
                connected_num.push(number);
            }
        }
    }

    Some(
        connected_num.iter().sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        assert_eq!(part_one("467"), Some(0));
        assert_eq!(part_one("*467."), Some(467));
        assert_eq!(part_one(".467*"), Some(467));
        assert_eq!(part_one("467\n..*"), Some(467));
        assert_eq!(part_one("467\n..*"), Some(467));
        assert_eq!(part_one("467\n.*."), Some(467));
        assert_eq!(part_one("467.\n*..."), Some(467));
        assert_eq!(part_one("467.\n...*"), Some(467));
        assert_eq!(part_one("*...\n467."), Some(467));
        assert_eq!(part_one("...*\n467."), Some(467));
        assert_eq!(part_one(".467\n*..."), Some(467));
        assert_eq!(part_one(".467\n...*"), Some(467));
        assert_eq!(part_one("*...\n.467"), Some(467));
        assert_eq!(part_one("...*\n.467"), Some(467));
        assert_eq!(part_one("....\n.467"), Some(0));
        assert_eq!(part_one("*467.555"), Some(467));
    }

    #[test]
    fn test_part_one() {
        let result = part_one("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, None);
    }
}
