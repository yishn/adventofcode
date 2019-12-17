#[derive(Clone)]
pub struct ProgramState(Vec<i64>, usize, usize);

impl ProgramState {
  pub fn new(program: Vec<i64>) -> ProgramState {
    ProgramState(program, 0, 0)
  }

  pub fn get_mut(&mut self) -> (&mut Vec<i64>, &mut usize, &mut usize) {
    (&mut self.0, &mut self.1, &mut self.2)
  }
}

#[derive(Debug, Copy, Clone)]
pub enum ProgramResult {
  Output(i64),
  WaitForInput,
  Halt,
}

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
  Relative
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
  SetRelativeBase,
  Halt
}

#[derive(Debug)]
struct Instruction {
  operation: OperationType,
  inputs: Vec<(ParameterMode, i64)>
}

fn parse_instruction(numbers: &[i64]) -> Instruction {
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
    9 => (OperationType::SetRelativeBase, 1),
    99 => (OperationType::Halt, 0),
    _ => panic!()
  };

  let get_parameter_mode = |i| {
    match (instruction_code - op_code) / 10i64.pow(2 + i as u32) % 10 {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      2 => ParameterMode::Relative,
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

pub fn run_program(state: &mut ProgramState, input: Option<i64>) -> ProgramResult {
  let (program, pointer, relative_base) = state.get_mut();
  let mut input = input;

  fn extend_memory(program: &mut Vec<i64>, index: usize) {
    while index >= program.len() {
      program.push(0);
    }
  }

  fn get_instruction_input_index(
    program: &mut Vec<i64>,
    instruction: &Instruction,
    relative_base: usize,
    index: usize
  ) -> usize {
    match instruction.inputs[index] {
      (ParameterMode::Relative, d) => {
        let j = (relative_base as i64 + d) as usize;
        extend_memory(program, j);
        j
      },
      (_, j) => {
        extend_memory(program, j as usize);
        j as usize
      }
    }
  }

  fn get_instruction_input(
    program: &mut Vec<i64>,
    instruction: &Instruction,
    relative_base: usize,
    index: usize
  ) -> i64 {
    match instruction.inputs[index] {
      (ParameterMode::Immediate, value) => value,
      _ => {
        let j = get_instruction_input_index(program, instruction, relative_base, index);
        program[j]
      }
    }
  }

  while *pointer < program.len() {
    let init_pointer = *pointer;
    let instruction = parse_instruction(&program[*pointer..]);

    let (target_value, target_index) = {
      let mut get_input = |i| {
        get_instruction_input(program, &instruction, *relative_base, i)
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
        OperationType::LessThan => (Some((get_input(0) < get_input(1)) as i64), Some(2)),
        OperationType::Equals => (Some((get_input(0) == get_input(1)) as i64), Some(2)),
        OperationType::JumpIfTrue => {
          if get_input(0) != 0 {
            *pointer = get_input(1) as usize;
          }

          (None, None)
        },
        OperationType::JumpIfFalse => {
          if get_input(0) == 0 {
            *pointer = get_input(1) as usize;
          }

          (None, None)
        },
        OperationType::SetRelativeBase => {
          *relative_base = (*relative_base as i64 + get_input(0)) as usize;
          (None, None)
        }
        OperationType::Halt => break
      };

      (
        target_value,
        output_index.map(|i| {
          get_instruction_input_index(program, &instruction, *relative_base, i)
        })
      )
    };

    if *pointer == init_pointer {
      *pointer += instruction.inputs.len() + 1;
    }

    if let Some(target_value) = target_value {
      if let Some(target_index) = target_index {
        extend_memory(program, target_index);
        program[target_index] = target_value;
      } else {
        return ProgramResult::Output(target_value);
      }
    }
  }

  ProgramResult::Halt
}

pub fn run_program_with_inputs<I>(state: &mut ProgramState, inputs: I) -> Vec<i64>
where I: Iterator<Item = i64> {
  let mut inputs = inputs;
  let mut outputs = vec![];
  let mut result = run_program(state, None);

  loop {
    if let ProgramResult::Output(x) = result {
      outputs.push(x);
    }

    result = run_program(state, match result {
      ProgramResult::WaitForInput => inputs.next(),
      _ => None
    });

    if let ProgramResult::Halt = result {
      break;
    }
  }

  outputs
}
