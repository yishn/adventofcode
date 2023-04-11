use std::collections::{HashSet, VecDeque};

static INPUT: &'static str = include_str!("./day06.txt");

struct ChunkIterator<I: Iterator> {
  n: usize,
  iter: I,
  buffer: VecDeque<I::Item>,
}

impl<I: Iterator> ChunkIterator<I> {
  pub fn new(n: usize, iter: I) -> Self {
    Self {
      n,
      iter,
      buffer: VecDeque::with_capacity(n),
    }
  }
}

impl<I> Iterator for ChunkIterator<I>
where
  I: Iterator,
  I::Item: Clone,
{
  type Item = VecDeque<I::Item>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.buffer.len() < self.n {
      self
        .buffer
        .extend((&mut self.iter).take(self.n - self.buffer.len()));

      Some(self.buffer.clone())
    } else if let Some(item) = self.iter.next() {
      self.buffer.pop_front();
      self.buffer.push_back(item);

      Some(self.buffer.clone())
    } else {
      None
    }
  }
}

#[test]
fn part1() {
  let chars = INPUT.chars();
  let n = 4;
  let mut chunks = ChunkIterator::new(n, chars);

  println!(
    "{}",
    chunks
      .position(|chunk| chunk.into_iter().collect::<HashSet<_>>().len() == n)
      .map(|i| i + n)
      .unwrap()
  );
}

#[test]
fn part2() {
  let chars = INPUT.chars();
  let n = 14;
  let mut chunks = ChunkIterator::new(n, chars);

  println!(
    "{}",
    chunks
      .position(|chunk| chunk.into_iter().collect::<HashSet<_>>().len() == n)
      .map(|i| i + n)
      .unwrap()
  );
}
