use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("19.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State(Vec<usize>);

impl State {
    fn new(first_register: usize) -> State {
        State(vec![first_register, 0, 0, 0, 0, 0])
    }

    fn get(&self, r: usize) -> usize {
        self.0.get(r).cloned().unwrap_or(0)
    }

    fn set(&mut self, r: usize, v: usize) {
        self.0.get_mut(r).map(|x| *x = v);
    }

    fn op(&mut self, op: Operation) {
        let v = match op.0 {
            "addr" => self.get(op.1) + self.get(op.2),
            "addi" => self.get(op.1) + op.2,
            "mulr" => self.get(op.1) * self.get(op.2),
            "muli" => self.get(op.1) * op.2,
            "banr" => self.get(op.1) & self.get(op.2),
            "bani" => self.get(op.1) & op.2,
            "borr" => self.get(op.1) | self.get(op.2),
            "bori" => self.get(op.1) | op.2,
            "setr" => self.get(op.1),
            "seti" => op.1,
            "gtir" => if op.1 > self.get(op.2) { 1 } else { 0 },
            "gtri" => if self.get(op.1) > op.2 { 1 } else { 0 },
            "gtrr" => if self.get(op.1) > self.get(op.2) { 1 } else { 0 },
            "eqir" => if op.1 == self.get(op.2) { 1 } else { 0 },
            "eqri" => if self.get(op.1) == op.2 { 1 } else { 0 },
            "eqrr" => if self.get(op.1) == self.get(op.2) { 1 } else { 0 },
            _ => return
        };

        self.set(op.3, v);
    }
}

type Operation<'a> = (&'a str, usize, usize, usize);

fn parse(input: &str) -> (usize, Vec<Operation>) {
    let mut ip_register = 0;
    let program = input.lines()
        .filter_map(|line| {
            if line.starts_with("#ip ") {
                line[4..].parse::<usize>().ok()
                .map(|x| ip_register = x);
            }

            let mut tokens = line.split(' ');
            let op_type = tokens.next();

            op_type.map(|op_type| (
                op_type,
                tokens
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            ))
        })
        .filter(|(_, v)| v.len() == 3)
        .map(|(op_type, v)| (op_type, v[0], v[1], v[2]))
        .collect();

    (ip_register, program)
}

fn run_program(mut state: State, ip_register: usize, program: &[Operation]) -> State {
    if ip_register >= state.0.len() {
        return state;
    }

    while let Some(&operation) = program.get(state.0[ip_register]) {
        state.op(operation);
        state.0[ip_register] += 1;
    }

    state
}

fn main() {
    let input = get_input().unwrap();
    let (ip_register, program) = parse(&input);
    let state = run_program(State::new(0), ip_register, &program);

    println!("Part 1: {}", state.0[0]);

    // Part 2 doesn't work for arbitrary input

    let r5 = 10551350;
    let result = (1..r5 + 1)
        .filter(|&r2| r5 % r2 == 0)
        .fold(0, |acc, x| acc + x);

    println!("Part 2: {}", result);
}
