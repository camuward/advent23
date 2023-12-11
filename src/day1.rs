#[yaah::aoc(day1, part1)]
pub fn part_one(input: &str) -> u32 {
    let first_last_digits = input.lines().map(|line| {
        let first = line.bytes().find(u8::is_ascii_digit);
        let last = line.bytes().rfind(u8::is_ascii_digit);

        first.zip(last).expect("line does not contain two numbers")
    });

    let calibration_values = first_last_digits.map(|(first, last)| {
        let num: &[u8] = &[first, last];
        let num: &str = core::str::from_utf8(num).unwrap();
        let num: u8 = num.parse().unwrap();

        num as u32
    });

    calibration_values.sum()
}

#[yaah::aoc(day1, part2)]
pub fn part_two(input: &str) -> u32 {
    let first_last_digits = input.lines().map(|line| {
        let get_number = |(i, b): (usize, u8)| {
            b.is_ascii_digit()
                .then(|| b)
                .or_else(|| match line.as_bytes()[i..] {
                    [b'o', b'n', b'e', ..] => Some(b'1'),
                    [b't', b'w', b'o', ..] => Some(b'2'),
                    [b't', b'h', b'r', b'e', b'e', ..] => Some(b'3'),
                    [b'f', b'o', b'u', b'r', ..] => Some(b'4'),
                    [b'f', b'i', b'v', b'e', ..] => Some(b'5'),
                    [b's', b'i', b'x', ..] => Some(b'6'),
                    [b's', b'e', b'v', b'e', b'n', ..] => Some(b'7'),
                    [b'e', b'i', b'g', b'h', b't', ..] => Some(b'8'),
                    [b'n', b'i', b'n', b'e', ..] => Some(b'9'),
                    _ => None,
                })
        };

        let first = line.bytes().enumerate().find_map(get_number);
        let last = line.bytes().enumerate().rev().find_map(get_number);

        first.zip(last).expect("line does not contain two numbers")
    });

    let calibration_values = first_last_digits.map(|(first, last)| {
        let num: &[u8] = &[first, last];
        let num: &str = core::str::from_utf8(num).unwrap();
        let num: u8 = num.parse().unwrap();

        num as u32
    });

    calibration_values.sum()
}
