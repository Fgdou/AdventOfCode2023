use std::{collections::HashMap, cmp::Ordering};

type num = u32;
struct Card(char);
#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: num
}
type Input = Vec<Hand>;

impl Into<u32> for Card {
    fn into(self) -> u32 {
        match self.0.to_digit(10) {
            Some(n) => n,
            None => match self.0 {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Char not found")
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High
}

impl Hand {
    pub fn get_type(&self) -> Type {
        let map = self.cards.iter()
            .fold(HashMap::<u32, num>::new(), |mut map, card| {
                map.insert(*card, *map.get(card).unwrap_or_else(|| &0) + 1);
                map
            });

        let mut values = map.values().collect::<Vec<_>>();
        values.sort();
        values.reverse();

        dbg!(&values);

        if values.len() == 1 {
            return Type::Five;
        }

        match (values[0], values[1]) {
            (4, _) => Type::Four,
            (3, 2) => Type::Full,
            (3, _) => Type::Three,
            (2, 2) => Type::Two,
            (2, _) => Type::One,
            _ => Type::High
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cards.iter().enumerate()
            .find_map(|(i, n)| {
                let res = n.partial_cmp(&other.cards[i]).unwrap();
                if res != Ordering::Equal {
                    return Some(res);
                }
                None
            })
    }
}

fn parse(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Hand {
                cards: parts.next()
                    .unwrap()
                    .chars()
                    .map(|c| Card(c).into())
                    .collect::<Vec<num>>(),
                bid: parts.next().unwrap().parse::<num>().unwrap()
            }
        }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, None);
    }

    #[test]
    fn test_types() {
        assert_eq!(Type::Five, Hand{cards: vec!(1, 1, 1, 1, 1), bid: 0}.get_type());
        assert_eq!(Type::Four, Hand{cards: vec!(1, 1, 0, 1, 1), bid: 0}.get_type());
        assert_eq!(Type::Full, Hand{cards: vec!(1, 1, 0, 0, 1), bid: 0}.get_type());
        assert_eq!(Type::Three, Hand{cards: vec!(1, 1, 2, 0, 1), bid: 0}.get_type());
        assert_eq!(Type::Two, Hand{cards: vec!(1, 2, 2, 0, 1), bid: 0}.get_type());
        assert_eq!(Type::One, Hand{cards: vec!(1, 2, 6, 0, 1), bid: 0}.get_type());
        assert_eq!(Type::High, Hand{cards: vec!(5, 2, 6, 0, 1), bid: 0}.get_type());
    }

    #[test]
    fn test_ord(){
        assert!(
            Hand{cards: vec!(1, 1, 1, 1, 1), bid: 0} == Hand{cards: vec!(1, 1, 1, 1, 1), bid: 0}
        );
        assert!(
            Hand{cards: vec!(1, 0, 1, 1, 1), bid: 0} < Hand{cards: vec!(1, 1, 1, 1, 1), bid: 0}
        );
        assert!(
            Hand{cards: vec!(1, 1, 1, 1, 1), bid: 0} > Hand{cards: vec!(1, 1, 1, 1, 0), bid: 0}
        );
    }
}
