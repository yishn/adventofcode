#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos(usize, usize);

struct CodeIterator {
    value: Option<u64>
}

impl CodeIterator {
    fn new() -> CodeIterator {
        CodeIterator {value: None}
    }
}

impl Iterator for CodeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = Some(match self.value {
            None => 20151125,
            Some(x) => (x * 252533) % 33554393
        });

        self.value.clone()
    }
}

fn pos_to_index(Pos(row, col): Pos) -> usize {
    let level = row + col - 1;
    (1..level).sum::<usize>() + col - 1
}

fn main() {
    let input = Pos(2981, 3075);
    let index = pos_to_index(input);
    let mut code_iterator = CodeIterator::new();

    println!("Part 1: {}", code_iterator.nth(index).unwrap());
}
