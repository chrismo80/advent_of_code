use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main()
{
    solve();
}

fn solve()
{
    let program: Vec<i64> = fs::read_to_string("AdventOfCode/2019/17/Input.txt")
        .expect("Failed to read input file")
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let mut scaffold: HashSet<(i64, i64)> = HashSet::new();
    let mut robot = IntCodeComputer::run(program.clone(), false);

    let commands = [
        "A,B,A,C,A,B,C,B,C,B\n",
        "R,8,L,10,L,12,R,4\n",
        "R,8,L,12,R,4,R,4\n",
        "R,8,L,10,R,8\n",
        "n\n",
    ];

    for command in &commands {
        for c in command.chars() {
            robot.inputs.lock().unwrap().push_back(c as i64);
        }
    }

    let mut x = 0;
    let mut y = 0;

    while robot.running.load() {
        let value = robot.outputs.try_recv();
        match value {
            Ok(value) => {
                print!("{}", (value as u8) as char);
                match value {
                    10 => {
                        x = -1;
                        y += 1;
                    }
                    35 => {
                        if y < 50 {
                            scaffold.insert((x, y));
                        }
                    }
                    _ => {}
                }
                x += 1;
            }
            Err(_) => {}
        }
    }

    let mut intersections: Vec<(i64, i64)> = Vec::new();
    for &(x, y) in scaffold.iter() {
        if is_intersection((x, y), &scaffold) {
            intersections.push((x, y));
        }
    }

    let part_1: i64 = intersections.iter().map(|(x, y)| x * y).sum();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", robot.outputs.recv().unwrap());
}

fn is_intersection(position: (i64, i64), scaffold: &HashSet<(i64, i64)>) -> bool
{
    let (x, y) = position;
    scaffold.contains(&(x + 1, y))
        && scaffold.contains(&(x - 1, y))
        && scaffold.contains(&(x, y + 1))
        && scaffold.contains(&(x, y - 1))
}

struct IntCodeComputer
{
    program: HashMap<i64, i64>,
    pointer: i64,
    relative_base: i64,
    running: Arc<std::sync::atomic::AtomicBool>,
    inputs: Arc<Mutex<VecDeque<i64>>>,
    outputs: Receiver<i64>,
}

impl IntCodeComputer
{
    fn run(program: Vec<i64>, wait: bool) -> IntCodeComputer
    {
        let mut memory = HashMap::new();
        for (i, value) in program.iter().enumerate() {
            memory.insert(i as i64, *value);
        }

        let (input_sender, input_receiver) = channel();
        let (output_sender, output_receiver) = channel::<i64>();

        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let inputs = Arc::new(Mutex::new(VecDeque::new()));
        let running_clone = running.clone();
        let inputs_clone = inputs.clone();

        let program_thread = thread::spawn(move || {
            let mut computer = IntCodeComputer {
                program: memory,
                pointer: 0,
                relative_base: 0,
                running: running_clone,
                inputs: inputs_clone,
                outputs: output_sender,
            };

            computer.start();
        });

        if wait {
            program_thread.join().unwrap();
        }

        IntCodeComputer {
            program: memory,
            pointer: 0,
            relative_base: 0,
            running,
            inputs,
            outputs: input_receiver,
        }
    }

    fn start(&mut self)
    {
        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            match self.program[&self.pointer] % 100 {
                1 => {
                    let value = self.read(1) + self.read(2);
                    self.write(3, value);
                    self.pointer += 4;
                }
                2 => {
                    let value = self.read(1) * self.read(2);
                    self.write(3, value);
                    self.pointer += 4;
                }
                3 => {
                    let input = self.inputs.lock().unwrap().pop_front().unwrap();
                    self.write(1, input);
                    self.pointer += 2;
                }
                4 => {
                    let value = self.read(1);
                    self.outputs.send(value).unwrap();
                    self.pointer += 2;
                }
                5 => {
                    if self.read(1) != 0 {
                        self.pointer = self.read(2);
                    }
                    else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    if self.read(1) == 0 {
                        self.pointer = self.read(2);
                    }
                    else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let value = if self.read(1) < self.read(2) { 1 } else { 0 };
                    self.write(3, value);
                    self.pointer += 4;
                }
                8 => {
                    let value = if self.read(1) == self.read(2) { 1 } else { 0 };
                    self.write(3, value);
                    self.pointer += 4;
                }
                9 => {
                    self.relative_base += self.read(1);
                    self.pointer += 2;
                }
                99 => {
                    self.running.store(false, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
                _ => {
                    self.pointer += 1;
                }
            }
        }
    }

    fn read(&mut self, offset: i64) -> i64
    {
        let param_mode = (self.program[&self.pointer] / 10i64.pow((offset + 1) as u32)) % 10;
        match param_mode {
            0 => {
                let address = self.program[&(self.pointer + offset)];
                *self.program.entry(address).or_insert(0)
            }
            1 => self.program[&(self.pointer + offset)],
            2 => {
                let address = self.program[&(self.pointer + offset)] + self.relative_base;
                *self.program.entry(address).or_insert(0)
            }
            _ => unreachable!(),
        }
    }

    fn write(&mut self, offset: i64, value: i64)
    {
        let param_mode = (self.program[&self.pointer] / 10i64.pow((offset + 1) as u32)) % 10;
        let address = match param_mode {
            0 => self.program[&(self.pointer + offset)],
            2 => self.program[&(self.pointer + offset)] + self.relative_base,
            _ => unreachable!(),
        };
        self.program.insert(address, value);
    }
}
