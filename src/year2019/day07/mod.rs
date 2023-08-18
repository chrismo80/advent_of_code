pub fn solve() -> (i64, i64)
{
    let input: Vec<i64> = include_str!("input.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let result1 = permute::permutations_of(&"01234".chars().collect::<Vec<char>>())
        .map(|setting| amplifier_chain(input.clone(), setting.copied().collect()))
        .max()
        .unwrap();

    let result2 = 0;

    println!("5\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn amplifier_chain(memory: Vec<i64>, phases: Vec<char>) -> i64
{
    let mut input = 0;

    for phase in phases {
        let mut computer = IntCodeComputer::new(memory.clone(), phase.to_digit(10).unwrap() as i64, false);
        computer.inputs.push_front(input);
        computer.start();
        input = computer.get_output();
    }

    input
}

struct IntCodeComputer
{
    program: Option<std::thread::JoinHandle<()>>,
    memory: std::collections::HashMap<i64, i64>,
    inputs: std::collections::VecDeque<i64>,
    outputs: std::collections::VecDeque<i64>,
    pointer: i64,
    phase: i64,
    relative_base: i64,
    running: bool,
}

impl IntCodeComputer
{
    fn new(memory: Vec<i64>, input: i64, wait: bool) -> Self
    {
        let mut mem = std::collections::HashMap::new();

        for (i, v) in memory.iter().enumerate() {
            mem.insert(i as i64, *v);
        }

        let mut inp = std::collections::VecDeque::new();

        inp.push_front(input);

        Self {
            program: None,
            memory: mem,
            inputs: inp,
            outputs: std::collections::VecDeque::new(),
            pointer: 0,
            phase: input,
            relative_base: 0,
            running: false,
        }
    }

    fn start(&mut self)
    {
        self.running = true;

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
                    if !self.inputs.is_empty() {
                        let input = self.inputs.pop_front().unwrap();
                        self.write(1, input);
                        self.pointer += 2;
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

        self.running = false;
    }

    fn get_output(&mut self) -> i64
    {
        while self.outputs.is_empty() {
            println!("waiting for output");
        }

        self.outputs.pop_front().unwrap()
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
