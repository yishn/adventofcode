use std::fs::File;
use std::io::Read;

extern crate json;
use json::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("12.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn count(value: &JsonValue, ignore_red: bool) -> i32 {
    match *value {
        JsonValue::Number(x) => x.into(),
        JsonValue::Array(ref vec) => {
            vec.iter().map(|x| count(&x, ignore_red)).sum()
        },
        JsonValue::Object(ref obj) => {
            if ignore_red && obj.iter().any(|(_, value)| match *value {
                JsonValue::Short(x) => x.as_str() == "red",
                _ => false
            }) {
                0
            } else {
                obj.iter().map(|(_, x)| count(&x, ignore_red)).sum()
            }
        }
        _ => 0
    }
}

fn main() {
    let input = get_input().unwrap();
    let parsed = json::parse(&input).unwrap();

    println!("Part 1: {}", count(&parsed, false));
    println!("Part 2: {}", count(&parsed, true));
}
