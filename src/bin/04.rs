struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

type CardList = Vec<Card>;

fn parse(input: &str) -> CardList {
    input.replace("  ", " ").lines().map(|line| {
        let card = line.split(": ").nth(1).unwrap();
        let mut it = card.split(" | ");
        let [left, right] = [it.next().unwrap(), it.next().unwrap()];
        let winning = left.split(" ").map(|n| n.parse::<u32>().unwrap()).collect();
        let numbers = right.split(" ").map(|n| n.parse::<u32>().unwrap()).collect();

        Card {
            winning, numbers
        }
    }).collect()
}

fn n_to_win(number_winning: &Vec<u32>) -> Vec<u32> {
    let mut cards = number_winning.iter().map(|_| 1).collect::<Vec<u32>>();

    for (pos, n) in number_winning.iter().enumerate() {
        let factor = cards[pos];
        for i in 1..=*n {
            if let Some(num) = cards.get_mut(pos+i as usize) {
                *num += factor;
            }
        }
    }

    cards
}

fn get_numbers_in(list: &Vec<u32>, to_search: &Vec<u32>) -> Vec<u32> {
    to_search.iter().filter(|n| list.contains(n)).copied().collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let winning = input.iter().map(|card| get_numbers_in(&card.winning, &card.numbers));

    Some(
        winning
            .filter(|card| card.len() != 0)
            .map(|card| (2 as u32).pow(card.len() as u32 - 1)).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let winning = input.iter().map(|card| get_numbers_in(&card.winning, &card.numbers));
    let number_of_cards = winning.map(|card| card.len() as u32).collect::<Vec<u32>>();
    
    Some(
        n_to_win(&number_of_cards).iter().sum()
    )
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, Some(30));
    }
}
