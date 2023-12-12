#[derive(PartialEq, Clone, Debug)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '#' => State::Damaged,
            '.' => State::Operational,
            '?' => State::Unknown,
            _ => panic!()
        }
    }
}

struct Seq {
    sequence: Vec<State>,
    groups: Vec<i32>
}
type Input = Vec<Seq>;

fn parse(input: &str) -> Input {
    input.lines().map(|l| {
        let mut parts = l.split(" ");
        Seq {
            sequence: parts.next().unwrap().chars().map(|e| e.into()).collect(),
            groups: parts.next().unwrap().split(",").map(|e| e.parse().unwrap()).collect()
        }
    }).collect()
}
fn is_good(seq: &Vec<State>, groups: &Vec<i32>) -> bool {
    let seqs = seq.split(|e| *e == State::Operational)
        .filter(|s| s.len() != 0);
    if seqs.clone().count() != groups.len() {
        return false
    }
    seqs.enumerate()
        .all(|(e, seq)| {
            e < groups.len() && seq.len() as i32 == groups[e]
        })
}

fn visit(sequence: &Vec<State>, groups: &Vec<i32>) -> i32 {
    let first_unknown = sequence.iter().position(|e| *e == State::Unknown);

    match first_unknown {
        None => if is_good(sequence, groups) { 1 } else { 0 },
        Some(index) => {
            let s1 = sequence.iter().enumerate()
                .map(|(i, n)| if i == index {State::Damaged} else {n.clone()}).collect();
            let s2 = sequence.iter().enumerate()
                .map(|(i, n)| if i == index {State::Operational} else {n.clone()}).collect();
            visit(&s1, groups) + visit(&s2, groups)
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let cnts = input.iter().map(|s| {
        visit(&s.sequence, &s.groups)
    });
    Some(cnts.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;
    use State::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, None);
    }

    #[test]
    fn test_is_good() {
        assert_eq!(true, is_good(&vec!(), &vec!()));
        assert_eq!(false, is_good(&vec!(Damaged), &vec!()));
        assert_eq!(true, is_good(&vec!(Damaged), &vec!(1)));
        assert_eq!(false, is_good(&vec!(Operational), &vec!(1)));
        assert_eq!(false, is_good(&vec!(), &vec!(1)));
        assert_eq!(false, is_good(&vec!(Damaged), &vec!(1, 2)));
        assert_eq!(true, is_good(&vec!(Operational, Damaged), &vec!(1)));
        assert_eq!(true, is_good(&vec!(Damaged, Operational), &vec!(1)));
        assert_eq!(true, is_good(&vec!(Operational, Damaged, Operational), &vec!(1)));
        assert_eq!(true, is_good(&vec!(Operational, Damaged, Operational, Damaged), &vec!(1, 1)));
        assert_eq!(false, is_good(&vec!(Operational, Damaged, Operational, Damaged), &vec!(1, 2)));
        assert_eq!(true, is_good(&vec!(Operational, Damaged, Damaged, Operational), &vec!(2)));
    }
}
