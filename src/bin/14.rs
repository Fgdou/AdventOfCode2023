type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Input {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn roll_up(tab: &mut Input) -> usize {
    let mut count = 0;
    for i in 1..tab.len() {
        for j in 0..tab[0].len() {
            let c = tab[i][j];
            if let Some(line) = tab.get(i-1) {
                let other_c = line[j];

                match (c, other_c) {
                    ('O', '.') => {
                        tab[i-1][j] = 'O';
                        tab[i][j] = '.';
                        count += 1;
                    }
                    _ => ()
                }
            }
        }
    }
    count
}

fn weights(tab: &Input) -> usize {
    let mut cnt = 0;
    for i in 0..tab.len() {
        for j in 0..tab[0].len() {
            let y = tab.len()-i;
            if tab[i][j] == 'O' {
                cnt += y;
            }
        }
    }
    cnt
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);

    while roll_up(&mut input) != 0 {}

    Some(weights(&input))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, None);
    }
}
