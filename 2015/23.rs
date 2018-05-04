use std::fs::File;
use std::io::Read;
use RegisterName::*;
use Instruction::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("23.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone)]
struct Registers(u32, u32);

#[derive(Debug, Copy, Clone, PartialEq)]
enum RegisterName {
    RegisterA,
    RegisterB
}

#[derive(Debug)]
enum Instruction {
    HLF(RegisterName),
    TPL(RegisterName),
    INC(RegisterName),
    JMP(i32),
    JIE(RegisterName, i32),
    JIO(RegisterName, i32)
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|line| {
        if line.len() < 4 {
            return None;
        }

        let action = &line[0..3];
        let args = &line[4..].split(", ").collect::<Vec<_>>();

        if args.len() < 1 {
            return None;
        }

        let register_name = match args[0] {
            "a" => Some(RegisterA),
            "b" => Some(RegisterB),
            _ => None
        };

        if let Some(register_name) = register_name {
            match (action, args.get(1)) {
                ("hlf", _) => Some(HLF(register_name)),
                ("tpl", _) => Some(TPL(register_name)),
                ("inc", _) => Some(INC(register_name)),
                (_, Some(arg)) => match arg.parse::<i32>() {
                    Ok(step) => match action {
                        "jie" => Some(JIE(register_name, step)),
                        "jio" => Some(JIO(register_name, step)),
                        _ => None
                    },
                    Err(_) => None
                },
                _ => None
            }
        } else {
            let step = match args[0].parse::<i32>() {
                Ok(x) => x,
                Err(_) => return None
            };

            match action {
                "jmp" => Some(JMP(step)),
                _ => None
            }
        }
    }).collect()
}

fn apply<F: Fn(u32) -> u32>(registers: Registers, name: RegisterName, change: F) -> Registers {
    match name {
        RegisterA => Registers(change(registers.0), registers.1),
        RegisterB => Registers(registers.0, change(registers.1))
    }
}

fn get(registers: Registers, name: RegisterName) -> u32 {
    match name {
        RegisterA => registers.0,
        RegisterB => registers.1
    }
}

fn run(start: Registers, instructions: &Vec<Instruction>) -> Registers {
    let mut registers = start;
    let mut pointer = 0i32;

    while pointer >= 0 && pointer < instructions.len() as i32 {
        match instructions[pointer as usize] {
            HLF(name) => registers = apply(registers, name, |x| x / 2),
            TPL(name) => registers = apply(registers, name, |x| x * 3),
            INC(name) => registers = apply(registers, name, |x| x + 1),
            JMP(step) => pointer += step - 1,
            JIE(name, step) if get(registers, name) % 2 == 0 => pointer += step - 1,
            JIO(name, step) if get(registers, name) == 1 => pointer += step - 1,
            _ => {}
        };

        pointer += 1;
    }

    registers
}

fn main() {
    let input = get_input().unwrap();
    let instructions = parse(&input);

    println!("Part 1: {}", run(Registers(0, 0), &instructions).1);
    println!("Part 2: {}", run(Registers(1, 0), &instructions).1);
}
