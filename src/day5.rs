// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = u32> + '_,
    impl Iterator<Item = impl Iterator<Item = [u32; 3]> + '_> + Clone + '_,
) {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .and_then(|line| line.strip_prefix("seeds: "))
        .expect("no seed list")
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let next_map = move || {
        lines.find(|line| line.ends_with("map:")).and_then(|line| {
            // SAFETY: `line` is guaranteed to be a substring of `input`.
            let remainder = unsafe {
                let line_end = line.as_ptr().add(line.len());
                let offset = line_end.offset_from(input.as_ptr());
                &input[offset as usize..]
            };

            // dbg!(remainder);

            let mut numbers = remainder
                .split_whitespace()
                .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
                .map(|n| n.parse().unwrap());

            // literally pure boilerplate
            let array_chunks = std::iter::from_fn(move || {
                let first = numbers.next()?;
                let (second, third) = numbers
                    .next()
                    .zip(numbers.next())
                    .expect("incomplete map entry");

                Some([first, second, third])
            });

            Some(array_chunks)
        })
    };

    (seeds, std::iter::from_fn(next_map))
}

#[yaah::aoc(day5, part1)]
fn part_one(input: &str) -> u32 {
    let (seeds, maps) = parse_input(input);

    let seeds = seeds.map(|seed| {
        maps.clone().fold(seed, |idx, mut map| {
            let entry = map.find(|&[_, src, len]| {
                let range = src..src + len;
                range.contains(&idx)
            });

            if let Some([dst, src, _]) = entry {
                idx - src + dst
            } else {
                idx
            }
        })
    });

    seeds.min().expect("no seeds")
}
