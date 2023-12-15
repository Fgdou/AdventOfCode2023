use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Set {
        hash: u64,
        value: u64,
        name: String
    },
    Remove {
        hash: u64,
        name: String
    }
}

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

fn parse2(input: &str) -> Vec<Instruction>{
    parse(input).into_iter().map(|e| {
        if let Some(i) = e.chars().position(|c| c == '=') {
            let name = &e[0..i];
            let value = &e[i+1..].parse::<u64>().unwrap();
            Instruction::Set { 
                hash: hash(name), 
                value: value.clone(), 
                name: name.to_string() 
            }
        } else {
            let name = &e[..e.len()-1];
            Instruction::Remove { 
                hash: hash(name),
                name: name.to_string()
            }
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let res = input.iter().map(|i| hash(i)).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse2(input);
    let mut hashmap: HashMap<u64, Vec<(String, u64)>> = HashMap::new();

    for i in &input {
        match i {
            Instruction::Set { hash, value, name } => {
                if !hashmap.contains_key(hash) {
                    hashmap.insert(hash.clone(), Vec::new());
                }
                let vec = hashmap.get_mut(&hash).unwrap();
                if let Some(x) = vec.iter_mut().find(|e| &e.0 == name) {
                    x.1 = *value;
                } else {
                    vec.push((name.clone(), value.clone()));
                }
            },
            Instruction::Remove { hash, name } => {
                if let Some(e) = hashmap.get_mut(hash) {
                    e.retain(|e| &e.0 != name)
                }
            },
        }
    }

    let res = hashmap.iter()
    .map(|(hash, instructions)| {
        instructions.iter().enumerate().map(|(j, instruction)| {
            (*hash+1)*(j as u64+1)*instruction.1
        }).sum::<u64>()
    }).sum();

    Some(res)
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
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        assert_eq!(30, hash("rn=1"))
    }
}

// 506805
// 506891