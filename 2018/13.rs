use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Point = (isize, isize);
type Tracks = HashMap<Point, Vec<Point>>;

struct Cart {
    position: Point,
    direction: Point,
    decision: usize
}

impl fmt::Debug for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.position, match self.direction {
            (-1, 0) => '<',
            (1, 0) => '>',
            (0, -1) => '^',
            (0, 1) => 'v',
            _ => ' '
        }))
    }
}

impl Cart {
    fn new(position: Point, direction: Point) -> Self {
        Self {
            position,
            direction,
            decision: 0
        }
    }

    fn move_cart(&mut self, neighbors: &[Point]) {
        let (x, y) = self.position;
        let (dx, dy) = self.direction;

        // Left, Straight, Right
        let targets = [(0, -1), (1, 0), (0, 1)].into_iter()
            .map(|(za, zb)| (x + dx * za - dy * zb, y + dx * zb + dy * za))
            .filter(|target| neighbors.contains(target))
            .collect::<Vec<_>>();

        let (nx, ny) = match targets.len() {
            1 => targets[0],
            3 => {
                let target = targets[self.decision];
                self.decision = (self.decision + 1) % 3;
                target
            },
            _ => return
        };

        self.position = (nx, ny);
        self.direction = (nx - x, ny - y);
    }
}

fn parse(input: &str) -> (Tracks, Vec<Cart>) {
    input.lines()
    .enumerate()
    .flat_map(|(j, line)| {
        let chars: Vec<char> = line.chars().collect();

        chars.iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, c)| {
            let (x, y) = (i as isize, j as isize);
            let horizontal_chars = ['-', '+', '<', '>'];

            let neighbors = match (c, chars.get(i + 1)) {
                ('/', Some(nc)) if horizontal_chars.contains(nc) => Some(vec![(x + 1, y), (x, y + 1)]),
                ('/', _) => Some(vec![(x - 1, y), (x, y - 1)]),
                ('\\', Some(nc)) if horizontal_chars.contains(nc) => Some(vec![(x + 1, y), (x, y - 1)]),
                ('\\', _) => Some(vec![(x - 1, y), (x, y + 1)]),

                ('+', _) => Some(vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]),
                ('-', _) | ('<', _) | ('>', _) => Some(vec![(x - 1, y), (x + 1, y)]),
                ('|', _) | ('^', _) | ('v', _) => Some(vec![(x, y - 1), (x, y + 1)]),

                _ => None
            };

            let cart_direction = match c {
                '<' => Some((-1, 0)),
                '>' => Some((1, 0)),
                '^' => Some((0, -1)),
                'v' => Some((0, 1)),
                _ => None
            };

            neighbors
            .map(|n| ((x, y), n, cart_direction.map(|d| Cart::new((x, y), d))))
        })
        .collect::<Vec<_>>()
    })
    .fold((Tracks::new(), Vec::new()), |(mut tracks, mut carts), (k, v, cart)| {
        tracks.insert(k, v);
        cart.map(|c| carts.push(c));

        (tracks, carts)
    })
}

fn tick(tracks: &Tracks, carts: &mut [Cart]) -> Option<Point> {
    carts.sort_unstable_by_key(|cart| (cart.position.1, cart.position.0));

    for i in 0..carts.len() {
        {
            let mut cart = carts.get_mut(i).unwrap();
            let neighbors = tracks.get(&cart.position).unwrap();
            cart.move_cart(neighbors);
        }

        if let Some(collision) = get_crash_location(carts) {
            return Some(collision);
        }
    }

    None
}

fn get_crash_location(carts: &[Cart]) -> Option<Point> {
    let positions = carts.iter()
        .map(|cart| cart.position)
        .collect::<Vec<_>>();

    positions.iter()
    .cloned()
    .enumerate()
    .filter(|&(i, p)| positions.iter().skip(i + 1).any(|&q| q == p))
    .map(|(_, p)| p)
    .next()
}

fn main() {
    let input = get_input().unwrap();
    let (tracks, mut carts) = parse(&input);

    loop {
        if let Some((x, y)) = tick(&tracks, &mut carts) {
            println!("Part 1: {},{}", x, y);
            break;
        }
    }
}
