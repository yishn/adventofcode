use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate
}

#[derive(Debug)]
enum OperationType {
  Add,
  Multiply,
  Input,
  Output,
  JumpIfTrue,
  JumpIfFalse,
  LessThan,
  Equals,
  Halt
}

#[derive(Debug)]
struct Instruction {
  operation: OperationType,
  inputs: Vec<(ParameterMode, i32)>
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ProgramResult {
  Output(i32),
  WaitForInput,
  Halt,
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("07.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_instruction(numbers: &[i32]) -> Instruction {
  let instruction_code = numbers[0];
  let op_code = instruction_code % 100;

  let (operation, inputs_count) = match op_code {
    1 => (OperationType::Add, 3),
    2 => (OperationType::Multiply, 3),
    3 => (OperationType::Input, 1),
    4 => (OperationType::Output, 1),
    5 => (OperationType::JumpIfTrue, 2),
    6 => (OperationType::JumpIfFalse, 2),
    7 => (OperationType::LessThan, 3),
    8 => (OperationType::Equals, 3),
    99 => (OperationType::Halt, 0),
    _ => panic!()
  };

  let get_parameter_mode = |i| {
    match (instruction_code - op_code) / 10i32.pow(2 + i as u32) % 10 {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      _ => panic!()
    }
  };

  Instruction {
    operation,
    inputs: (0..inputs_count)
      .map(|i| (get_parameter_mode(i), numbers[i + 1]))
      .collect()
  }
}

fn run_program(state: (&mut Vec<i32>, &mut usize), input: Option<i32>) -> ProgramResult {
  let (program, pointer) = state;
  let mut input = input;

  while *pointer < program.len() {
    let init_pointer = *pointer;
    let instruction = parse_instruction(&program[*pointer..]);

    let (target_value, target_index) = {
      let get_input = |i| match instruction.inputs[i] {
        (ParameterMode::Position, j) => program[j as usize],
        (ParameterMode::Immediate, value) => value
      };

      let mut jump = |condition, i| {
        if condition {
          *pointer = get_input(i) as usize;
        }

        (None, None)
      };

      let (target_value, output_index) = match instruction.operation {
        OperationType::Add => (Some(get_input(0) + get_input(1)), Some(2)),
        OperationType::Multiply => (Some(get_input(0) * get_input(1)), Some(2)),
        OperationType::Input => match input {
          Some(x) => {
            input = None;
            (Some(x), Some(0))
          },
          _ => return ProgramResult::WaitForInput
        },
        OperationType::Output => (Some(get_input(0)), None),
        OperationType::LessThan => (Some((get_input(0) < get_input(1)) as i32), Some(2)),
        OperationType::Equals => (Some((get_input(0) == get_input(1)) as i32), Some(2)),
        OperationType::JumpIfTrue => jump(get_input(0) != 0, 1),
        OperationType::JumpIfFalse => jump(get_input(0) == 0, 1),
        OperationType::Halt => break
      };

      (target_value, output_index.map(|i| instruction.inputs[i].1 as usize))
    };

    if *pointer == init_pointer {
      *pointer += instruction.inputs.len() + 1;
    }

    if let Some(target_value) = target_value {
      if let Some(target_index) = target_index {
        program[target_index] = target_value;
      } else {
        return ProgramResult::Output(target_value);
      }
    }
  }

  ProgramResult::Halt
}

fn get_phase_settings(n: usize, values: std::ops::Range<usize>) -> Vec<Vec<i32>> {
  if n == 1 {
    return values.map(|i| vec![i as i32]).collect();
  }

  let values_clone = values.clone();

  values.flat_map(|i| {
    get_phase_settings(n - 1, values_clone.clone()).into_iter()
    .filter(move |phase_setting| !phase_setting.contains(&(i as i32)))
    .map(move |mut phase_setting| {
      phase_setting.push(i as i32);
      phase_setting
    })
  })
  .collect()
}

fn start_thruster_amplifier(program: &Vec<i32>, phase_setting: &Vec<i32>) -> i32 {
  let mut input = 0;

  for &phase in phase_setting {
    let mut program = program.clone();
    let mut pointer = 0;

    match run_program((&mut program, &mut pointer), Some(phase)) {
      ProgramResult::WaitForInput => (),
      _ => panic!()
    };

    input = match run_program((&mut program, &mut pointer), Some(input)) {
      ProgramResult::Output(output) => output,
      _ => 0,
    };
  }

  input
}

fn start_thruster_amplifier_feedback_loop(program: &Vec<i32>, phase_setting: &Vec<i32>) -> i32 {
  let count = phase_setting.len();
  let mut program_results = (0..count)
    .map(|_| ProgramResult::WaitForInput)
    .collect::<Vec<_>>();
  let mut program_states = (0..count)
    .map(|_| program.clone())
    .collect::<Vec<_>>();
  let mut program_pointers = (0..count)
    .map(|_| 0)
    .collect::<Vec<_>>();
  let mut input_queues = phase_setting.iter()
    .map(|&phase| {
      let mut queue = VecDeque::new();
      queue.push_back(phase);
      queue
    })
    .collect::<Vec<_>>();
  let mut outputs = (0..count)
    .map(|_| vec![])
    .collect::<Vec<_>>();

  input_queues[0].push_back(0);

  while program_results.iter().any(|&result| result != ProgramResult::Halt) {
    for i in 0..count {
      if program_results[i] == ProgramResult::Halt {
        continue;
      }

      let queue = &mut input_queues[i];
      let result = run_program((&mut program_states[i], &mut program_pointers[i]), queue.pop_front());
      program_results[i] = result;

      if let ProgramResult::Output(output) = result {
        outputs[i].push(output);
        input_queues[(i + 1) % count].push_back(output);
      }
    }
  }

  outputs.last().unwrap().last().cloned().unwrap()
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i32>().ok())
    .collect::<Vec<_>>();

  let highest_signal = get_phase_settings(5, 0..5).into_iter()
    .map(|phase_setting| start_thruster_amplifier(&numbers, &phase_setting))
    .max();

  println!("Part 1: {}", highest_signal.unwrap());

  let highest_signal = get_phase_settings(5, 5..10).into_iter()
    .map(|phase_setting| start_thruster_amplifier_feedback_loop(&numbers, &phase_setting))
    .max();

  println!("Part 2: {}", highest_signal.unwrap());
}
