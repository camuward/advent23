unsafe fn index_in_line(substring: &str, current_line: &str) -> usize {
    let offset = substring.as_ptr().offset_from(current_line.as_ptr()) as usize;
    debug_assert!(offset <= current_line.len());

    offset
}

#[yaah::aoc(day3, part1)]
pub fn part_one(input: &str) -> u32 {
    let line_width = input.lines().next().expect("no input").len();
    let sep_width = match input[line_width..].bytes().next() {
        Some(b'\r') => 2,
        Some(b'\n') => 1,
        None => 0, // there's only one line
        _ => unreachable!(),
    };

    let numbers = input.lines().enumerate().flat_map(|(y, line)| {
        line.as_bytes()
            .split(|b| !b.is_ascii_digit())
            .filter(|s| !s.is_empty())
            // SAFETY: `bytes` contains only ascii digits
            .map(|bytes| unsafe { core::str::from_utf8_unchecked(bytes) })
            // SAFETY: `num` is a substring of `line`
            .map(move |num| (num, unsafe { index_in_line(num, line) }, y))
    });

    numbers
        .filter_map(|(num, x_start, y)| {
            let take_lines = if y == 0 { 2 } else { 3 };
            let start_line_before = y.saturating_sub(1) * (line_width + sep_width);

            input[start_line_before..]
                .lines()
                .take(take_lines)
                .find_map(|line| {
                    let take_bytes = num.len() + if x_start == 0 { 1 } else { 2 };

                    line[x_start.saturating_sub(1)..]
                        .bytes()
                        .take(take_bytes)
                        .any(|b| b != b'.' && !b.is_ascii_digit())
                        .then(|| num.parse::<u32>().unwrap())
                })
        })
        .sum()
}

#[yaah::aoc(day3, part2)]
pub fn part_two(input: &str) -> u32 {
    use alloc::vec::Vec;

    #[derive(Clone, Copy)]
    enum GearAdj {
        None,
        One(u16),
        Two(u16, u16),
        TooMany,
    }

    // scan the input for gears
    let mut gears: Vec<((u8, u8), GearAdj)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_x, b)| b == b'*')
                .map(move |(x, _)| (x as u8, y as u8))
        })
        .zip(core::iter::repeat(GearAdj::None))
        .collect();

    // scan the input for numbers
    for (y, line) in input.lines().enumerate() {
        let num_strs = line
            .as_bytes()
            .split(|b| !b.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|bytes| core::str::from_utf8(bytes).expect("invalid utf8"));

        // search for a gear around each number
        for num in num_strs {
            let x_range = {
                // SAFETY: `num` is a substring of `line`
                let offset = unsafe { index_in_line(num, line) };
                let start = offset.saturating_sub(1);
                let end = offset + num.len();

                start as u8..=end as u8
            };

            let y_range = {
                let start = y.saturating_sub(1);
                let end = y + 1;

                start as u8..=end as u8
            };

            let in_range = |x, y| y_range.contains(y) && x_range.contains(x);
            if let Some(i) = gears.iter().position(|((x, y), _)| in_range(x, y)) {
                let num: u32 = num.parse().unwrap();

                let (_pos, gear) = &mut gears[i];
                match gear {
                    GearAdj::None => *gear = GearAdj::One(num as u16),
                    GearAdj::One(first) => *gear = GearAdj::Two(*first, num as u16),
                    GearAdj::Two(_, _) => *gear = GearAdj::TooMany,
                    GearAdj::TooMany => {}
                };
            }
        }
    }

    gears
        .into_iter()
        .map(|(_pos, gear)| {
            let GearAdj::Two(a, b) = gear else {
                return 0;
            };

            a as u32 * b as u32
        })
        .sum()
}
