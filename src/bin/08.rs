use std::collections::HashMap;

use lcm::Lcm;


#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}
#[derive(Debug)]
struct Input {
    network: HashMap<String, Node>,
    instructions: String,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().to_string();
    lines.next().unwrap();

    let map = lines.fold(HashMap::new(), |mut map, line| {
        let from = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        map.insert(from, Node{left, right});
        map
    });

    Input {
        network: map,
        instructions
    }
}

fn network_move<'a>(network: &'a HashMap<String, Node>, position: &str, direction: char) -> &'a str {
    let node = network.get(position).unwrap();
    match direction {
        'L' => &node.left,
        'R' => &node.right,
        _ => panic!()
    }
}
fn network_round<'a>(input: &'a Input, pos: &'a str, count: u32) -> (u32, &'a str) {
    input.instructions.chars()
        .fold((count, pos), |(i, pos), dir| {
            let pos = network_move(&input.network, pos, dir);
            if pos == "ZZZ" {
                return (i+1, pos)
            }
            (i+1, pos)
        })
}

fn count(input: &Input, start: &str) -> u32 {
    let mut pos = start;
    let mut i = 0;

    while pos.chars().nth(2).unwrap() != 'Z' {
        pos = network_move(&input.network, pos, input.instructions.chars().nth(i%input.instructions.len()).unwrap());
        i = i+1;
    }

    i as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut pos = (0, "AAA");

    while pos.1 != "ZZZ" {
        pos = network_round(&input, pos.1, pos.0);
    }

    Some(pos.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let starts = input.network.keys().filter(|k| k.chars().nth(2).unwrap() == 'A');

    let numbers = starts.map(|s| count(&input, s));

    let lcms = numbers.fold(1, |l, n| {
        
    })
    

    Some(i as u32)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one2() {
        let result = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one() {
        let result = part_one("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, Some(6));
    }
}
