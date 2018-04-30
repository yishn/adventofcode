use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("18.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos(i32, i32);

struct LightsGrid {
    data: Vec<Vec<bool>>,
    defective: bool
}

impl LightsGrid {
    fn get_neighbors(&self, Pos(x, y): Pos) -> Vec<Pos> {
        [
            Pos(x - 1, y - 1),
            Pos(x - 1, y),
            Pos(x - 1, y + 1),
            Pos(x, y + 1),
            Pos(x + 1, y + 1),
            Pos(x + 1, y),
            Pos(x + 1, y - 1),
            Pos(x, y - 1),
        ].iter().cloned().filter(|&Pos(x, y)| {
            x >= 0 && y >= 0
            && x < self.data.len() as i32
            && self.data.len() > 0 && y < self.data[0].len() as i32
        }).collect()
    }

    fn get_corners(&self) -> Vec<Pos> {
        match self.data.len() {
            0 => vec![],
            _ => vec![
                Pos(0, 0),
                Pos(0, (self.data[0].len() - 1) as i32),
                Pos((self.data.len() - 1) as i32, 0),
                Pos((self.data.len() - 1) as i32, (self.data[0].len() - 1) as i32)
            ]
        }
    }

    fn count_lit(&self) -> u32 {
        self.data.iter().map(|vec| vec.iter().cloned().filter(|&x| x).count() as u32).sum()
    }
}

impl Iterator for LightsGrid {
    type Item = Vec<Vec<bool>>;

    fn next(&mut self) -> Option<Self::Item> {
        let corners = self.get_corners();

        self.data = (0..100).map(|x| {
            (0..100).map(|y| {
                let p = Pos(x as i32, y as i32);
                let lit_neighbors = self.get_neighbors(p).into_iter()
                    .filter(|&Pos(x, y)| self.data[y as usize][x as usize]);

                if self.defective && corners.iter().any(|&x| x == p) {
                    return true;
                }

                match (self.data[y][x], lit_neighbors.count()) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false
                }
            }).collect()
        }).collect();

        Some(self.data.clone())
    }
}

fn main() {
    let input = get_input().unwrap();
    let data = input.lines().filter(|&line| {
        line.len() > 0
    }).map(|line| {
        line.chars().map(|c| c == '#').collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut grid = LightsGrid {data: data.clone(), defective: false};
    grid.nth(99);

    println!("Part 1: {}", grid.count_lit());

    let mut grid = LightsGrid {data, defective: true};
    grid.data[0][0] = true;
    grid.data[0][99] = true;
    grid.data[99][0] = true;
    grid.data[99][99] = true;
    grid.nth(99);

    println!("Part 2: {}", grid.count_lit());
}
