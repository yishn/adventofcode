struct LookAndSay {
    value: String
}

impl LookAndSay {
    fn new(initial_value: &str) -> LookAndSay {
        LookAndSay {value: initial_value.to_string()}
    }
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let chars = self.value.chars().collect::<Vec<_>>();
        let mut count_info = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            let mut number = 1;

            for j in i + 1..chars.len() {
                if chars[j] == c {
                    number += 1;
                    i += 1;
                } else {
                    break;
                }
            }

            count_info.push((number, c));
            i += 1;
        }

        self.value = count_info.into_iter().map(|(number, c)| {
            format!("{}{}", number, c)
        }).fold(String::new(), |acc, string| acc + &string);

        Some(self.value.clone())
    }
}

fn main() {
    let input = "3113322113";
    let mut iterator = LookAndSay::new(input);

    let result = iterator.nth(39).unwrap();
    println!("Part 1: {}", result.len());

    let result = iterator.nth(9).unwrap();
    println!("Part 2: {}", result.len());
}
