use std::{collections::HashMap, ops::Range};

#[derive(Debug)]
struct Workflow {
    conds: Vec<WorkflowPart>,
    last: String
}
#[derive(Debug)]
struct WorkflowPart {
    part: char,
    comparator: char,
    value: usize,
    then: String
}
type Part = HashMap<char, usize>;
type PartRange = HashMap<char, Range<usize>>;

#[derive(Debug)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>
}

fn parse(input: &str) -> Input {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(|line| {
        let (name, parts) = line[..line.len()-1].split_once('{').unwrap();
        let parts = parts.split(",").collect::<Vec<&str>>();
        let last = parts[parts.len()-1].to_string();
        let parts = parts[0..parts.len()-1].into_iter().map(|part| {
            let p = part.chars().nth(0).unwrap();
            let cond = part.chars().nth(1).unwrap();
            let (number, to) = part[2..].split_once(":").unwrap();
            WorkflowPart {
                comparator: cond,
                part: p,
                then: to.to_string(),
                value: number.parse().unwrap()
            }
        }).collect();
        (name.to_string(), Workflow {
            conds: parts,
            last
        })
    }).collect();
    let parts = parts.lines().map(|line| {
        line[1..line.len()-1].split(",").map(|e| {
            (e.chars().next().unwrap(), e[2..].parse::<usize>().unwrap())
        }).collect()
    }).collect();
    Input {
        parts,
        workflows
    }
}

fn run_condition(workflows: &HashMap<String, Workflow>, part: &Part, condition_name: &str) -> bool {
    if condition_name == "A" {
        return true;
    }
    if condition_name == "R" {
        return false;
    }
    for cond in &workflows[condition_name].conds {
        match cond.comparator {
            '<' => {
                if part[&cond.part] < cond.value {
                    return run_condition(workflows, part, cond.then.as_str());
                }
            },
            '>' => {
                if part[&cond.part] > cond.value {
                    return run_condition(workflows, part, cond.then.as_str());
                }
            },
            _ => unreachable!()
        }
    }
    run_condition(workflows, part, workflows[condition_name].last.as_str())
}
fn run_condition_count(workflows: &HashMap<String, Workflow>, part: &PartRange, condition_name: &str) -> usize {
    if condition_name == "A" {
        return part.values().map(|v| v.len()).product();
    }
    if condition_name == "R" {
        return 0;
    }
    let mut rest = part.clone();
    let mut sum = 0;

    for cond in &workflows[condition_name].conds {
        match cond.comparator {
            '<' => {
                let mut left = rest.clone();
                left.get_mut(&cond.part).unwrap().end = cond.value;
                rest.get_mut(&cond.part).unwrap().start = cond.value;
                sum += run_condition_count(workflows, &left, cond.then.as_str());
            },
            '>' => {
                let mut left = rest.clone();
                left.get_mut(&cond.part).unwrap().start = cond.value+1;
                rest.get_mut(&cond.part).unwrap().end = cond.value+1;
                sum += run_condition_count(workflows, &left, cond.then.as_str());
            },
            _ => unreachable!()
        }
    }
    sum += run_condition_count(workflows, &rest, &workflows[condition_name].last.as_str());
    sum

}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let res = input.parts.iter().filter(|part| {
        run_condition(&input.workflows, &part, "in")
    })
    .map(|e| e.values().sum::<usize>())
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let res = run_condition_count(&input.workflows, &[
        ('x', 1..4001),
        ('m', 1..4001),
        ('a', 1..4001),
        ('s', 1..4001),
    ].into_iter().collect(), "in");

    Some(res)
}

advent_of_code::main!(19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn test_run_count() {
        assert_eq!(
            49,
            run_condition_count(
                &[
                    ("in".to_string(), Workflow{ conds: [WorkflowPart{ part: 'a', comparator: '<', value: 50, then: "A".to_string() }].into_iter().collect(), last: "R".to_string() })
                ].into_iter().collect(),
                &[
                    ('a', 1..101)
                ].into_iter().collect(),
                "in"
            )
        );
        assert_eq!(
            50,
            run_condition_count(
                &[
                    ("in".to_string(), Workflow{ conds: [WorkflowPart{ part: 'a', comparator: '>', value: 50, then: "A".to_string() }].into_iter().collect(), last: "R".to_string() })
                ].into_iter().collect(),
                &[
                    ('a', 1..101)
                ].into_iter().collect(),
                "in"
            )
        );
        assert_eq!(
            50*2,
            run_condition_count(
                &[
                    ("in".to_string(), Workflow{ conds: [WorkflowPart{ part: 'a', comparator: '>', value: 50, then: "A".to_string() }].into_iter().collect(), last: "R".to_string() })
                ].into_iter().collect(),
                &[
                    ('a', 1..101), ('b', 1..3)
                ].into_iter().collect(),
                "in"
            )
        );
    }
}

// < 25063492333491600