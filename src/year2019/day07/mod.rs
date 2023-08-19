use std::collections::*;

pub fn solve() -> (i64, i64)
{
    let input: HashMap<i64, i64> = include_str!("input.txt")
        .split(',')
        .enumerate()
        .map(|(i, x)| (i as i64, x.parse().unwrap()))
        .collect();

    let result1 = permute::permutations_of(&"01234".chars().collect::<Vec<char>>())
        .map(|setting| amplifier_chain(input.clone(), setting.copied().collect()))
        .max()
        .unwrap();

    let result2 = permute::permutations_of(&"56789".chars().collect::<Vec<char>>())
        .map(|setting| amplifier_chain(input.clone(), setting.copied().collect()))
        .max()
        .unwrap();

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn amplifier_chain(memory: HashMap<i64, i64>, phases: Vec<char>) -> i64
{
    let count = phases.len();

    let mut amps: Vec<IntCodeComputer> = phases
        .iter()
        .map(|phase| IntCodeComputer::new(memory.clone(), phase.to_digit(10).unwrap() as i64))
        .collect();

    amps[0].add_input(0);

    let mut states: Vec<State> = (0..count).map(|i| amps[i].run()).collect();

    while states.iter().any(|state| *state == State::Waiting) {
        for i in 0..count {
            if states[i] == State::Waiting {
                if let Some(input) = amps[(count + i - 1) % count].get_output() {
                    amps[i].add_input(input);
                    states[i] = amps[i].run();
                }
            }
        }
    }

    amps.last_mut().unwrap().get_output().unwrap()
}

#[derive(PartialEq)]
enum State
{
    Finished,
    Waiting,
}

#[derive(Clone)]
struct IntCodeComputer
{
    memory: HashMap<i64, i64>,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    pointer: i64,
    phase: i64,
    relative_base: i64,
}

impl IntCodeComputer
{
    fn new(memory: HashMap<i64, i64>, input: i64) -> Self
    {
        Self {
            memory,
            inputs: VecDeque::from(vec![input]),
            outputs: VecDeque::new(),
            pointer: 0,
            phase: input,
            relative_base: 0,
        }
    }

    fn add_input(&mut self, input: i64)
    {
        self.inputs.push_back(input);
    }

    fn get_output(&mut self) -> Option<i64>
    {
        self.outputs.pop_front()
    }

    fn run(&mut self) -> State
    {
        while self.memory[&self.pointer] != 99 {
            let m = self.memory[&self.pointer] % 100;

            match self.memory[&self.pointer] % 100 {
                1 => {
                    self.write(3, self.read(1) + self.read(2));
                    self.pointer += 4;
                }
                2 => {
                    self.write(3, self.read(1) * self.read(2));
                    self.pointer += 4;
                }
                3 => {
                    if let Some(input) = self.inputs.pop_front() {
                        self.write(1, input);
                        self.pointer += 2;
                    }
                    else {
                        return State::Waiting;
                    }
                }
                4 => {
                    self.outputs.push_back(self.read(1));
                    self.pointer += 2;
                }
                5 => {
                    if self.read(1) != 0 {
                        self.pointer = self.read(2);
                    }
                    else {
                        self.pointer += 3;
                    };
                }
                6 => {
                    if self.read(1) == 0 {
                        self.pointer = self.read(2);
                    }
                    else {
                        self.pointer += 3;
                    };
                }
                7 => {
                    self.write(3, (self.read(1) < self.read(2)) as i64);
                    self.pointer += 4;
                }
                8 => {
                    self.write(3, (self.read(1) == self.read(2)) as i64);
                    self.pointer += 4;
                }
                9 => {
                    self.relative_base += self.read(1);
                    self.pointer += 2;
                }
                _ => panic!("Invalid opcode"),
            }
        }

        State::Finished
    }

    fn read(&self, offset: i64) -> i64
    {
        *self.memory.get(&self.parameter(offset)).unwrap_or(&0)
    }

    fn write(&mut self, offset: i64, value: i64)
    {
        self.memory.insert(self.parameter(offset), value);
    }

    fn parameter(&self, offset: i64) -> i64
    {
        let mode = format!("{:0>5}", self.memory.get(&(self.pointer)).unwrap())
            .chars()
            .nth(3 - offset as usize)
            .unwrap();

        match mode {
            '0' => *self.memory.get(&(self.pointer + offset)).unwrap_or(&0),
            '1' => self.pointer + offset,
            '2' => *self.memory.get(&(self.pointer + offset)).unwrap_or(&0) + self.relative_base,
            _ => panic!("Invalid parameter mode"),
        }
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (17440, 27561242));
    }
}
