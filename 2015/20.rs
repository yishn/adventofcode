fn get_present_counts(max: u32, presents: u32, limit: Option<u32>) -> Vec<u32> {
    let len = max / presents;
    let mut houses = Vec::with_capacity(len as usize);

    for _ in 0..len {
        houses.push(0);
    }

    for i in 1..len {
        let mut count = 0;
        let mut j = i;

        while j < len {
            houses[j as usize] += i * presents;
            
            count += 1;
            j += match limit {
                Some(x) if count >= x => break,
                _ => i
            };
        }
    }

    houses
}

fn main() {
    let input = 34000000;
    let result = get_present_counts(input, 10, None)
        .into_iter()
        .position(|x| x >= input).unwrap();

    println!("Part 1: {}", result);

    let result = get_present_counts(input, 11, Some(50))
        .into_iter()
        .position(|x| x >= input).unwrap();

    println!("Part 2: {}", result);
}
