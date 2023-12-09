pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()
    }).collect()
}

pub fn derive(nums: &Vec<i64>) -> Vec<i64> {
    let mut res = Vec::new();

    for i in 1..nums.len() {
        res.push(nums[i]-nums[i-1]);
    }

    res
}

pub fn full_derive(nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut res = Vec::new();
    res.push(nums);

    while !res.last().unwrap().iter().all(|n| *n==0) {
        res.push(derive(res.last().unwrap()))
    }

    res
}

pub fn primitive(nums: &Vec<i64>, mut other: Vec<i64>) -> Vec<i64> {
    let last = other.last().unwrap();
    other.push(last + nums.last().unwrap());
    other
}
pub fn full_primitive(mut nums: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let l = nums.len();
    for i in 1..l {
        let values = nums[l-i-1].clone();
        nums[l-i-1] = primitive(&nums[l-i], values)
    }
    nums
}

pub fn primitive2(nums: &Vec<i64>, mut other: Vec<i64>) -> Vec<i64> {
    let first = other.first().unwrap();
    other.insert(0, first - nums.first().unwrap());
    other
}
pub fn full_primitive2(mut nums: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let l = nums.len();
    for i in 1..l {
        let values = nums[l-i-1].clone();
        nums[l-i-1] = primitive2(&nums[l-i], values)
    }
    nums
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);

    let sum = input.into_iter()
        .map(|line| {
            let der = full_derive(line);
            full_primitive(der)
        })
        .map(|tabs| *tabs[0].last().unwrap())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);

    let sum = input.into_iter()
        .map(|line| {
            let der = full_derive(line);
            full_primitive2(der)
        })
        .map(|tabs| *tabs[0].first().unwrap())
        .sum();

    Some(sum)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_derivative() {
        assert_eq!(vec!(3, 3, 3, 3, 3), derive(&vec!(0, 3, 6,  9, 12,  15)));
        assert_eq!(vec!(0, 0, 0, 0), derive(&vec!(3, 3, 3, 3, 3)));
    }

    #[test]
    fn test_full_derivative() {
        assert_eq!(
            vec!(
                vec!(0, 3, 6, 9, 12, 15),
                vec!(3, 3, 3, 3, 3),
                vec!(0, 0, 0, 0)
            ),
            full_derive(vec!(0, 3, 6, 9, 12, 15))
        )
    }

    #[test]
    fn test_primitive(){
        assert_eq!(vec!(3, 3, 3, 3, 3, 3), primitive(&vec!(0, 0, 0, 0, 0), vec!(3, 3, 3, 3, 3)));
        assert_eq!(vec!(0, 3, 6, 9, 12, 15, 18), primitive(&vec!(3, 3, 3, 3, 3, 3), vec!(0, 3, 6, 9, 12, 15)));
    }

    #[test]
    fn test_primitive2() {
        assert_eq!(vec!(2, 2, 2, 2), primitive2(&vec!(0, 0, 0), vec!(2, 2, 2)));
        assert_eq!(vec!(-2, 0, 2, 4, 6), primitive2(&vec!(2, 2, 2, 2), vec!(0, 2, 4, 6)));
    }
}
