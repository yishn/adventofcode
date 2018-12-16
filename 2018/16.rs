use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("16.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum OpType {
    AddR, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtIR, GtRI, GtRR,
    EqIR, EqRI, EqRR
}

const OP_TYPE_VALUES: [OpType; 16] = [
    OpType::AddR, OpType::AddI,
    OpType::MulR, OpType::MulI,
    OpType::BanR, OpType::BanI,
    OpType::BorR, OpType::BorI,
    OpType::SetR, OpType::SetI,
    OpType::GtIR, OpType::GtRI, OpType::GtRR,
    OpType::EqIR, OpType::EqRI, OpType::EqRR
];

impl OpType {
    fn iter() -> impl Iterator<Item = OpType> {
        OP_TYPE_VALUES.iter().cloned()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State(Vec<usize>);

impl State {
    fn new() -> Self {
        Self(vec![0, 0, 0, 0])
    }

    fn get(&self, r: usize) -> usize {
        self.0.get(r).cloned().unwrap_or(0)
    }

    fn set(&mut self, r: usize, v: usize) {
        self.0.get_mut(r).map(|x| *x = v);
    }

    fn op(&mut self, op_type: OpType, op: Operation) {
        let v = match op_type {
            OpType::AddR => self.get(op.1) + self.get(op.2),
            OpType::AddI => self.get(op.1) + op.2,
            OpType::MulR => self.get(op.1) * self.get(op.2),
            OpType::MulI => self.get(op.1) * op.2,
            OpType::BanR => self.get(op.1) & self.get(op.2),
            OpType::BanI => self.get(op.1) & op.2,
            OpType::BorR => self.get(op.1) | self.get(op.2),
            OpType::BorI => self.get(op.1) | op.2,
            OpType::SetR => self.get(op.1),
            OpType::SetI => op.1,
            OpType::GtIR => if op.1 > self.get(op.2) { 1 } else { 0 },
            OpType::GtRI => if self.get(op.1) > op.2 { 1 } else { 0 },
            OpType::GtRR => if self.get(op.1) > self.get(op.2) { 1 } else { 0 },
            OpType::EqIR => if op.1 == self.get(op.2) { 1 } else { 0 },
            OpType::EqRI => if self.get(op.1) == op.2 { 1 } else { 0 },
            OpType::EqRR => if self.get(op.1) == self.get(op.2) { 1 } else { 0 }
        };

        self.set(op.3, v);
    }
}

type Operation = (usize, usize, usize, usize);
type Sample = (State, Operation, State);

fn parse(input: &str) -> (Vec<Sample>, Vec<Operation>) {
    let input = input.replace('\r', "");
    let mut parts = input.split("\n\n\n\n");

    match (parts.next(), parts.next()) {
        (Some(samples_content), Some(program_content)) => {
            let samples = samples_content.split("\n\n")
                .map(|chunk| chunk.lines().collect::<Vec<_>>())
                .filter(|v| v.len() == 3)
                .map(|v| (
                    [v[0], v[2]],
                    v[1].split(' ')
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                ))
                .map(|(v, op)| (
                    v.iter()
                    .filter_map(|line| line.split(|c| c == '[' || c == ']').nth(1))
                    .map(|line| {
                        line.split(", ")
                        .map(|x| x.parse::<usize>().ok().unwrap_or(0))
                        .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
                    op
                ))
                .filter(|(v, op)| v.len() == 2 && op.len() == 4)
                .map(|(mut v, op)| (State(v.remove(0)), (op[0], op[1], op[2], op[3]), State(v.remove(0))))
                .collect();

            let program = program_content.lines()
                .map(|line| {
                    line.split(' ')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()
                })
                .filter(|v| v.len() == 4)
                .map(|v| (v[0], v[1], v[2], v[3]))
                .collect();

            (samples, program)
        },
        _ => (Vec::new(), Vec::new())
    }
}

fn possible_op_types(&(ref before, op, ref after): &Sample) -> HashSet<OpType> {
    OpType::iter()
    .filter(|&op_type| {
        let mut state = before.clone();
        state.op(op_type, op);
        state == *after
    })
    .collect()
}

fn main() {
    let input = get_input().unwrap();
    let (samples, program) = parse(&input);

    let result = samples.iter()
        .map(|sample| possible_op_types(sample))
        .filter(|types| types.len() >= 3)
        .count();

    println!("Part 1: {}", result);

    let mut opcode_map = samples.iter()
        .map(|sample| ((sample.1).0, possible_op_types(sample)))
        .fold(HashMap::new(), |mut acc, (code, types)| {
            if !acc.contains_key(&code) {
                acc.insert(code, types);
            } else {
                let values = {
                    let values = acc.get_mut(&code).unwrap();
                    values.intersection(&types).cloned().collect()
                };

                acc.insert(code, values);
            }

            acc
        });

    while opcode_map.values().any(|s| s.len() > 1) {
        let assigned = opcode_map.values()
            .filter(|s| s.len() == 1)
            .filter_map(|s| s.iter().next().cloned())
            .collect::<HashSet<_>>();

        for (_, values) in opcode_map.iter_mut() {
            if values.len() > 1 {
                *values = values.difference(&assigned).cloned().collect();
            }
        }
    }

    let State(registers) = program.into_iter()
        .fold(State::new(), |mut state, op| {
            opcode_map.get(&op.0)
            .and_then(|types| types.iter().next().cloned())
            .map(|op_type| state.op(op_type, op));

            state
        });

    println!("Part 2: {}", registers[0]);
}
