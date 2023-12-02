#[derive(Debug, PartialEq)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}
#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse(input: &str) -> Vec<Game> {
    input.lines()
    .map(|line| {
        let mut infos = line.split(": ");
        let id = &infos.next()
            .unwrap()[5..]
            .parse::<u32>().unwrap();

        let sets = infos.next().unwrap()
                .split("; ")
                .map(|subset| {
                    let mut s = Set { r: 0, g: 0, b: 0 };
                    subset.split(", ").for_each(|part| {
                        let mut it = part.split(" ");
                        let n = it.next().unwrap()
                            .parse().unwrap();
                        let color = it.next().unwrap();

                        match color {
                            "blue" => s.b = n,
                            "red" => s.r = n,
                            "green" => s.g = n,
                            _ => panic!()
                        }
                    });
                    s
                }).collect::<Vec<Set>>();
        
        Game{
            id: *id,
            sets
        }
    }).collect::<Vec<Game>>()
}
fn get_sum_possible(games: &Vec<Game>, possible: &Set) -> u32 {
    games.iter()
    .filter(|game| {
        game.sets.iter()
            .all(|set| {
                set.r <= possible.r &&
                set.g <= possible.g &&
                set.b <= possible.b
            })
    })
    .map(|game| game.id)
    .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);

    let possible_set = Set{
        r: 12, g: 13, b: 14
    };

    Some(get_sum_possible(&games, &possible_set))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_parse() {
        assert_eq!(Vec::<Game>::new(), parse(""));
        assert_eq!(vec!(
            Game{
                id: 10,
                sets: vec!(
                    Set{r: 0, g: 1, b: 20}
                )
            }
        ), parse("Game 10: 20 blue, 1 green"));
        assert_eq!(vec!(
            Game{
                id: 0,
                sets: vec!(
                    Set{r: 0, g: 1, b: 2},
                    Set{r: 10, g: 0, b: 0},
                )
            }
        ), parse("Game 0: 2 blue, 1 green; 10 red"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}
