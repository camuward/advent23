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

enum GearAdj {
    None,
    One(u16),
    Two(u16, u16),
    TooMany,
}

#[yaah::aoc(day3, part2, naive)]
pub fn naive_part_two(input: &str) -> u32 {
    use alloc::vec::Vec;

    let mut gears: Vec<((usize, usize), GearAdj)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, b)| {
                if b == b'*' {
                    Some(((x, y), GearAdj::None))
                } else {
                    None
                }
            })
        })
        .collect();

    for (y, line) in input.lines().enumerate() {
        let num_strs = line
            .as_bytes()
            .split(|b| !b.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|bytes| core::str::from_utf8(bytes).expect("invalid utf8"));

        for num in num_strs {
            // SAFETY: s is a substring of line
            let x_start = unsafe { num.as_ptr().offset_from(line.as_ptr()) as usize };
            let x_end = x_start + num.len();

            let x_range = x_start.saturating_sub(1)..=x_end;
            let y_range = y.saturating_sub(1)..=y + 1;

            if let Some(i) = gears
                .iter()
                .position(|((x, y), _)| y_range.contains(y) && x_range.contains(x))
            {
                let num: u32 = num.parse().unwrap();

                let (_, adj) = &mut gears[i];
                match adj {
                    GearAdj::None => *adj = GearAdj::One(num as u16),
                    GearAdj::One(n) => *adj = GearAdj::Two(*n, num as u16),
                    GearAdj::Two(_, _) => *adj = GearAdj::TooMany,
                    GearAdj::TooMany => {}
                }
            }
        }
    }

    gears
        .into_iter()
        .filter_map(|(_, adj)| match adj {
            GearAdj::Two(a, b) => Some(a as u32 * b as u32),
            _ => None,
        })
        .sum::<u32>()
}
