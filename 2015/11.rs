fn get_alphabet() -> Vec<char> {
    "abcdefghijklmnopqrstuvwxyz".chars().collect()
}

struct AlphaIterator {
    alphabet: Vec<char>,
    value: String
}

impl AlphaIterator {
    fn new(init_value: &str) -> AlphaIterator {
        AlphaIterator {
            alphabet: get_alphabet(),
            value: init_value.to_string()
        }
    }
}

impl Iterator for AlphaIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.value.chars().collect::<Vec<_>>();
        if chars.len() == 0 {
            return Some("a".to_string());
        }

        let mut bump = true;

        for i in (0..chars.len()).rev() {
            if !bump {
                break;
            }

            let c = chars[i];
            let index = self.alphabet.iter().position(|&a| a == c).unwrap();

            bump = index + 1 == self.alphabet.len();            
            let c = self.alphabet[(index + 1) % self.alphabet.len()];

            chars[i] = c;
        }

        self.value = chars.into_iter().fold(String::new(), |mut acc, c| {
            acc.push(c);
            acc
        });

        Some(self.value.clone())
    }
}

fn valid_pwd(alphabet: &Vec<char>, string: &str) -> bool {
    // Passwords must include one increasing straight of at least three letters
    alphabet.windows(3).map(|window| {
        window.iter().fold(String::new(), |mut acc, &c| {
            acc.push(c);
            acc
        })
    }).any(|abc| string.contains(&abc))

    // Passwords may not contain certain letters
    && ['i', 'o', 'l'].iter().all(|&c| !string.contains(c))

    // Passwords must contain at least two different, non-overlapping pairs of letters
    && {
        let chars = string.chars().collect::<Vec<_>>();

        (0..chars.len() - 3).any(|i| {
            chars[i] == chars[i + 1]
            && (i + 2..chars.len() - 1).any(|j| {
                chars[j] == chars[j + 1]
            })
        })
    }
}

fn main() {
    let input = "hxbxwxba";
    let alphabet = get_alphabet();
    let iterator = AlphaIterator::new(input);
    
    let mut passwords = iterator
        .filter(|string| valid_pwd(&alphabet, &string));

    println!("Part 1: {}", passwords.nth(0).unwrap());
    println!("Part 2: {}", passwords.nth(0).unwrap());
}
