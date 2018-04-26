mod md5;

fn main() {
    let input = "yzbqklnj";
    let mut part1 = -1;

    for i in 1.. {
        let text = format!("{}{}", input, i);
        let hash = format!("{:x}", md5::compute(text.as_bytes()));

        if hash.chars().take(6).all(|x| x == '0') {
            println!("Part 2: {}", i);
            break;
        } else if part1 < 0 && hash.chars().take(5).all(|x| x == '0') {
            println!("Part 1: {}", i);
            part1 = i;
        }
    }
}
