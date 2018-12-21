fn main() {
    // Doesn't work for arbitrary inputs

    let mut r3;
    let mut r4 = 0;
    let mut values = vec![];

    while 123 & 456 != 72 {}

    for i in 0.. {
        r3 = r4 | 65536;
        r4 = 707129;

        loop {
            r4 = ((r4 + (r3 & 255)) & 16777215) * 65899 & 16777215;

            if r3 < 256 {
                break;
            }

            r3 = (0..).find(|r2| r3 < (r2 + 1) * 256).unwrap();
        }

        if i == 0 {
            println!("Part 1: {}", r4);
        }

        if values.contains(&r4) {
            println!("Part 2: {}", values[values.len() - 1]);
            break;
        }

        values.push(r4);
    }
}
