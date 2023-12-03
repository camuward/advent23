#[yaah::aoc(day3, part1)]
fn part_one(input: &str) -> u32 {
    let symbols: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, b)| {
                if b == b'.' || b.is_ascii_digit() {
                    None
                } else {
                    Some((x, y))
                }
            })
        })
        .collect();

    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .split(|b| !b.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|bytes| std::str::from_utf8(bytes).expect("invalid utf8"))
                .map(|s| {
                    let num: u32 = s.parse().unwrap();

                    // SAFETY: s is a substring of line
                    let x_start = unsafe { s.as_ptr().offset_from(line.as_ptr()) as usize };
                    let x_end = x_start + s.len();

                    // check if a symbol is adjacent
                    let y_range = y.saturating_sub(1)..=y + 1;
                    let x_range = x_start.saturating_sub(1)..=x_end;

                    if symbols
                        .iter()
                        .any(|(x, y)| y_range.contains(y) && x_range.contains(x))
                    {
                        num
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

enum GearAdj {
    None,
    One(u16),
    Two(u16, u16),
    TooMany,
}

#[yaah::aoc(day3, part2)]
fn part_two(input: &str) -> u32 {
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
            .map(|bytes| std::str::from_utf8(bytes).expect("invalid utf8"));

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
