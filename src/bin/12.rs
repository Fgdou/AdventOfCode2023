use indicatif::ProgressIterator;

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

#[derive(Debug, PartialEq)]
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
    let seqs: Vec<i32> = seq.split(|e| *e == State::Operational)
        .filter(|s| s.len() != 0)
        .map(|s| s.len() as i32).collect();
    &seqs == groups
}

fn is_good_part(sequence: &[State], groups: &Vec<i32>) -> bool {
    let cnts: Vec<i32> = sequence.to_vec()
        .split(|s| *s == State::Operational)
        .filter(|s| s.len() != 0)
        .map(|s| s.len() as i32).collect();

    if cnts.len() > groups.len() {
        return false
    }

    if cnts.is_empty() {
        return true
    }

    let p1 = cnts[0..cnts.len()-1].iter().enumerate().all(|(i, e)| {
            *e == groups[i]
        });
    let p2 = cnts[cnts.len()-1] <= groups[cnts.len()-1];

    p1 && p2
}

fn visit(sequence: Vec<State>, groups: &Vec<i32>) -> i32 {
    let first_unknown = sequence.iter().position(|e| *e == State::Unknown);

    match first_unknown {
        None => if is_good(&sequence, groups) { 1 } else { 0 },
        Some(index) => {
            if !is_good_part(&sequence[0..index], groups) {
                return 0
            }

            let s1 = sequence.iter().enumerate()
                .map(|(i, n)| if i == index {State::Damaged} else {n.clone()}).collect();
            let s2 = sequence.into_iter().enumerate()
                .map(|(i, n)| if i == index {State::Operational} else {n}).collect();
            visit(s1, groups) + visit(s2, groups)
        }
    }
}
fn visit2(sequence: Vec<State>, mut groups: Vec<i32>, i: usize) -> i32 {
    if i == sequence.len() {
        if groups.len() == 0 || groups[0] == 0 {
            return 1
        } else {
            return 0
        }
    }

    let c = &sequence[i];

    match c {
        State::Damaged => {
            if groups[0] < 1 {
                return 0
            }
            groups[0] -= 1;

            visit2(sequence, groups, i+1)
        },
        State::Operational => {
            if groups[0] != 0 {
                return 0
            }
            groups.remove(0);
            visit2(sequence, groups, i+1)
        },
        State::Unknown => {
            let mut s1 = sequence.clone();
            s1[i] = State::Damaged;
            let mut s2 = sequence;
            s2[i] = State::Operational;

            visit2(s1, groups.clone(), i) + visit2(s2, groups, i)
        },
    }

}

fn expand(seq: Seq, n: i32) -> Seq {
    Seq {
        groups: (0..n).into_iter().map(|_| seq.groups.clone()).flatten().collect(),
        sequence: (0..n).into_iter()
            .map(|_| seq.sequence.clone())
            .reduce(|mut acc, mut e| {
                acc.push(State::Unknown);
                acc.append(&mut e);
                acc
            }).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let cnts = input.into_iter().map(|s| {
        visit2(s.sequence, s.groups, 0)
    });
    Some(cnts.sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    // let input: Vec<Seq> = parse(input).into_iter().map(|r| expand(r, 5)).collect();

    // let cnts = input.into_iter().progress().map(|s| {
    //     visit2(s.sequence, s.groups, 0)
    // });
    // Some(cnts.sum());
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
        assert_eq!(result, Some(525152));
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

    #[test]
    fn test_expand(){
        let vec = Seq{
            groups: vec!(1, 2),
            sequence: vec!(State::Operational, State::Damaged)
        };
        let expected = Seq {
            groups: vec!(1, 2, 1, 2),
            sequence: vec!(State::Operational, State::Damaged, State::Unknown, State::Operational, State::Damaged)
        };
        assert_eq!(expected, expand(vec, 2));
    }
}
