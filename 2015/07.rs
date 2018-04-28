use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("07.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Value<'a> {
    Wire(&'a str),
    Num(u16)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action<'a> {
    Assign(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    LShift(Value<'a>, u16),
    RShift(Value<'a>, u16),
    Not(Value<'a>)
}

#[derive(Debug)]
struct Instruction<'a> {
    wire: &'a str,
    action: Action<'a>
}

struct Circuit<'a, 'b: 'a> {
    instructions: &'a Vec<Instruction<'b>>,
    cache: HashMap<&'b str, u16>
}

impl<'a, 'b> Circuit<'a, 'b> {
    fn new(instructions: &'a Vec<Instruction<'b>>) -> Circuit<'a, 'b> {
        let cache = HashMap::new();
        Circuit {instructions, cache}
    }

    fn resolve(&mut self, value: Value<'b>) -> u16 {
        match value {
            Value::Num(x) => x,
            Value::Wire(wire) => {
                if let Some(x) = self.cache.get(wire) {
                    return *x;
                }

                let instruction = self.instructions.iter()
                    .find(|&i| i.wire == wire).unwrap();

                let num = match instruction.action {
                    Action::Assign(x) => self.resolve(x),
                    Action::And(x, y) => self.resolve(x) & self.resolve(y),
                    Action::Or(x, y) => self.resolve(x) | self.resolve(y),
                    Action::LShift(x, y) => self.resolve(x) << y,
                    Action::RShift(x, y) => self.resolve(x) >> y,
                    Action::Not(x) => !self.resolve(x)
                };

                self.cache.insert(wire, num);
                num
            }
        }
    }
}

fn parse_value(string: &str) -> Value {
    match string.parse::<u16>() {
        Ok(x) => Value::Num(x),
        Err(_) => Value::Wire(string)
    }
}

fn parse_line(line: &str) -> Option<Instruction> {
    let tokens = line.split(" -> ").collect::<Vec<_>>();
    let left_tokens = tokens[0].split(' ').collect::<Vec<_>>();

    let wire = tokens[1];
    let action = match left_tokens.len() {
        l if l == 1 => {
            Action::Assign(parse_value(left_tokens[0]))
        },
        l if l == 2 => {
            Action::Not(parse_value(left_tokens[1]))
        },
        l if l == 3 => {
            let arg1 = left_tokens[0];
            let arg2 = left_tokens[2];

            match left_tokens[1] {
                "AND" => {
                    Action::And(parse_value(arg1), parse_value(arg2))
                },
                "OR" => {
                    Action::Or(parse_value(arg1), parse_value(arg2))
                },
                "LSHIFT" => {
                    Action::LShift(parse_value(arg1), arg2.parse::<u16>().unwrap())
                },
                "RSHIFT" => {
                    Action::RShift(parse_value(arg1), arg2.parse::<u16>().unwrap())
                },
                _ => return None
            }
        },
        _ => return None
    };

    Some(Instruction {wire, action})
}

fn main() {
    let input = get_input().unwrap();
    let mut instructions = input.lines().filter_map(parse_line).collect::<Vec<_>>();
    let mut result: u16;

    {
        let mut circuit = Circuit::new(&instructions);
        result = circuit.resolve(Value::Wire("a"));

        println!("Part 1: {}", result);
    }

    {
        let index = instructions.iter().position(|i| i.wire == "b").unwrap();

        instructions[index] = Instruction {
            wire: "b",
            action: Action::Assign(Value::Num(result))
        };

        let mut circuit = Circuit::new(&instructions);
        result = circuit.resolve(Value::Wire("a"));

        println!("Part 2: {}", result);
    }
}
