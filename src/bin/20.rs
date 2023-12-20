use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum State {
    High,
    Low
}
#[derive(Clone, Debug)]
enum Module {
    Conjunction {
        inputs: HashMap<String, State>,
        outputs: Vec<String>,
    },
    FlipFlop {
        outputs: Vec<String>,
        state: State,
    },
    Broadcaster {
        outputs: Vec<String>
    }
}
struct Pulse {
    state: State,
    destination: String,
    from: String
}
impl State {
    fn reverse(&self) -> Self {
        match self {
            State::High => State::Low,
            State::Low => State::High
        }
    }
}

type Input = HashMap<String, Module>;

fn parse(input: &str) -> Input {
    let mut res: Input = input.lines().map(|line| {
        let (name, list) = line.split_once(" -> ").unwrap();
        let list = list.split(", ").map(str::to_string).collect();

        match name.chars().next().unwrap() {
            '%' => (name[1..].to_string(), Module::FlipFlop { outputs: list, state: State::Low }),
            '&' => (name[1..].to_string(), Module::Conjunction { inputs: Default::default(), outputs: list }),
            _ => (name.to_string(), Module::Broadcaster { outputs: list })
        }
    }).collect();

    let outputs: HashMap<String, Vec<String>> = res.iter().map(|e| (e.0.clone(), match e.1 {
        Module::Conjunction {outputs, ..} => outputs.clone(),
        Module::FlipFlop { outputs, ..} => outputs.clone(),
        Module::Broadcaster { outputs } => outputs.clone(),
    })).collect();

    outputs.iter().for_each(|(name, list)| {
        list.iter().for_each(|module| {
            if let Some(m) = res.get_mut(module) {
                match m {
                    Module::Conjunction { inputs, ..} => {inputs.insert((*name).clone(), State::Low);},
                    _ => ()
                };
            }
        });
    });

    res
}

impl Module {
    fn pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self {
            Module::Conjunction { inputs, outputs } => {
                *inputs.get_mut(&pulse.from).unwrap() = pulse.state;
                let state = if inputs.iter().all(|i| i.1 == &State::High) {
                    State::Low
                } else {
                    State::High
                };
                Module::send_all(&state, outputs, &pulse.destination)
            },
            Module::FlipFlop { outputs, state } => {
                if pulse.state == State::Low {
                    *state = state.reverse();
                    Module::send_all(state, outputs, &pulse.destination)
                } else {
                    vec!()
                }
            },
            Module::Broadcaster { outputs } => {
                Module::send_all(&pulse.state, outputs, &pulse.destination)
            },
        }
    }
    fn send_all(state: &State, outputs: &Vec<String>, from: &String) -> Vec<Pulse> {
        outputs.iter().map(|output| {
            Pulse { state: state.clone(), destination: output.clone(), from: from.clone()}
        }).collect()
    }
}

fn push_button(input: &mut Input) -> (usize, usize) {
    let mut cnt = (0usize, 0usize);
    let mut cache: Vec<Pulse> = Vec::new();

    cache.push(Pulse {
        state: State::Low,
        destination: "broadcaster".to_string(),
        from: "button".to_string(),
    });

    while !cache.is_empty() {
        let pulse = cache.remove(0);

        match &pulse.state {
            State::High => cnt.1 += 1,
            State::Low => cnt.0 += 1,
        };

        if let Some(m) = input.get_mut(&pulse.destination) {
            let res = m.pulse(pulse);
            cache.extend(res);
        }
    }

    cnt
}
fn push_button2(input: &mut Input) -> bool {
    let mut cache: Vec<Pulse> = Vec::new();

    cache.push(Pulse {
        state: State::Low,
        destination: "broadcaster".to_string(),
        from: "button".to_string(),
    });

    while !cache.is_empty() {
        let pulse = cache.remove(0);

        if pulse.destination.as_str() == "rx" && pulse.state == State::Low {
            return true;
        }

        if let Some(m) = input.get_mut(&pulse.destination) {
            let res = m.pulse(pulse);
            cache.extend(res);
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);

    let res = (0..1000).into_iter()
        .map(|_| push_button(&mut input))
        .reduce(|i1, i2| (i1.0+i2.0, i1.1+i2.1))
        .unwrap();
    Some(res.0*res.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    // let mut input = parse(input);

    // let mut cnt = 0;

    // loop {
    //     cnt += 1;
    //     if push_button2(&mut input) {
    //         break;
    //     }
    // }

    // Some(cnt)
    None
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, Some(11687500));
    }
}
