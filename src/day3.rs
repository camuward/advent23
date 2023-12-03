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
