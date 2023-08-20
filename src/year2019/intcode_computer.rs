use std::collections::*;

#[derive(Debug, PartialEq)]
pub enum State
{
    Done,
    Waiting,
}

#[derive(Clone, Default)]
pub struct IntCodeComputer
{
    memory: HashMap<i64, i64>,
    pointer: i64,
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    relative_base: i64,
}

impl IntCodeComputer
{
    pub fn new(memory: HashMap<i64, i64>) -> Self
    {
        Self {
            memory,
            ..Default::default()
        }
    }

    pub fn add_input(&mut self, input: i64)
    {
        self.inputs.push_back(input);
    }

    pub fn get_output(&mut self) -> Option<i64>
    {
        self.outputs.pop_front()
    }

    pub fn run(&mut self) -> State
    {
        while self.memory[&self.pointer] != 99 {
            let opcode = self.memory[&self.pointer] % 100;

            match opcode {
                1 => self.write(3, self.read(1) + self.read(2)),
                2 => self.write(3, self.read(1) * self.read(2)),
                7 => self.write(3, (self.read(1) < self.read(2)) as i64),
                8 => self.write(3, (self.read(1) == self.read(2)) as i64),
                9 => self.relative_base += self.read(1),
                3 => {
                    if let Some(input) = self.inputs.pop_front() {
                        self.write(1, input);
                    }
                    else {
                        return State::Waiting;
                    }
                }
                4 => self.outputs.push_back(self.read(1)),
                5 | 6 => {}
                _ => panic!("Invalid opcode"),
            }

            match opcode {
                1 | 2 | 7 | 8 => self.pointer += 4,
                3 | 4 | 9 => self.pointer += 2,
                5 => self.pointer = if self.read(1) != 0 { self.read(2) } else { self.pointer + 3 },
                6 => self.pointer = if self.read(1) == 0 { self.read(2) } else { self.pointer + 3 },
                _ => panic!("Invalid opcode"),
            }
        }

        State::Done
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
        let left_pad = format!("{:0>5}", self.memory[&self.pointer]);
        let mode: u32 = left_pad.chars().nth(3 - offset as usize).unwrap().to_digit(10).unwrap();

        match mode {
            0 => *self.memory.get(&(self.pointer + offset)).unwrap_or(&0),
            1 => self.pointer + offset,
            2 => *self.memory.get(&(self.pointer + offset)).unwrap_or(&0) + self.relative_base,
            _ => panic!("Invalid parameter mode"),
        }
    }
}
