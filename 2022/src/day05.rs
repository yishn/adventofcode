use std::str::FromStr;

static INPUT: &'static str = include_str!("./day05.txt");

#[derive(Debug)]
struct Instruction {
  n: usize,
  from: usize,
  to: usize,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    let mut tokens = s.split(" ");
    let n = tokens.nth(1);
    let from = tokens.nth(1);
    let to = tokens.nth(1);

    if let (Some(n), Some(from), Some(to)) = (
      n.and_then(|x| x.parse::<usize>().ok()),
      from.and_then(|x| x.parse::<usize>().ok()),
      to.and_then(|x| x.parse::<usize>().ok()),
    ) {
      Ok(Instruction {
        n,
        from: from - 1,
        to: to - 1,
      })
    } else {
      Err(())
    }
  }
}

#[derive(Debug)]
enum InstructionError {
  StackDoesNotExist,
  StackEmpty,
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
  fn do_instruction(
    &mut self,
    instruction: &Instruction,
  ) -> Result<(), InstructionError> {
    for _ in 0..instruction.n {
      let item = self
        .0
        .get_mut(instruction.from)
        .ok_or(InstructionError::StackDoesNotExist)?
        .pop()
        .ok_or(InstructionError::StackEmpty)?;

      self
        .0
        .get_mut(instruction.to)
        .ok_or(InstructionError::StackDoesNotExist)?
        .push(item);
    }

    Ok(())
  }

  fn do_instruction_with_multiple_cranes(
    &mut self,
    instruction: &Instruction,
  ) -> Result<(), InstructionError> {
    let stack = self
      .0
      .get_mut(instruction.from)
      .ok_or(InstructionError::StackDoesNotExist)?;

    let mut items = stack.split_off(stack.len() - instruction.n);

    self
      .0
      .get_mut(instruction.to)
      .ok_or(InstructionError::StackDoesNotExist)?
      .append(&mut items);

    Ok(())
  }

  fn read_top(&self) -> String {
    self.0.iter().filter_map(|stack| stack.last()).collect()
  }
}

fn parse_input(input: &str) -> Option<(Stacks, Vec<Instruction>)> {
  let mut parts = input.split("\n\n");
  let state_input = parts.next();
  let instructions_input = parts.next();

  if let (Some(state_input), Some(instructions_input)) =
    (state_input, instructions_input)
  {
    let instructions = instructions_input
      .lines()
      .filter_map(|line| line.parse::<Instruction>().ok())
      .collect();

    let stack_indices = state_input.lines().last().map(|line| {
      line
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
    });

    let stacks = stack_indices.map(|stack_indices| {
      stack_indices
        .iter()
        .copied()
        .map(|i| {
          state_input
            .lines()
            .rev()
            .skip(1)
            .filter_map(|line| line.chars().nth(i))
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    });

    stacks.map(|stacks| (Stacks(stacks), instructions))
  } else {
    None
  }
}

#[test]
fn part1() -> Result<(), InstructionError> {
  let (mut stacks, instructions) = parse_input(INPUT).unwrap();

  for instruction in instructions.iter() {
    stacks.do_instruction(instruction)?;
  }

  println!("{}", stacks.read_top());

  Ok(())
}

#[test]
fn part2() -> Result<(), InstructionError> {
  let (mut stacks, instructions) = parse_input(INPUT).unwrap();

  for instruction in instructions.iter() {
    stacks.do_instruction_with_multiple_cranes(instruction)?;
  }

  println!("{}", stacks.read_top());

  Ok(())
}
