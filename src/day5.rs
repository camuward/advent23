use std::ops::Range;

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = u64> + '_,
    impl Iterator<Item = impl Iterator<Item = (Range<u64>, Range<u64>)> + Clone + '_> + Clone + '_,
) {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .and_then(|line| line.strip_prefix("seeds: "))
        .expect("no seed list")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let next_map = move || {
        lines.find(|line| line.ends_with("map:")).and_then(|line| {
            // SAFETY: `line` is guaranteed to be a substring of `input`.
            let remainder = unsafe {
                let line_end = line.as_ptr().add(line.len());
                let offset = line_end.offset_from(input.as_ptr());
                &input[offset as usize..]
            };

            let mut numbers = remainder
                .split_whitespace()
                .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
                .map(|n| n.parse().unwrap());

            let array_chunks = std::iter::from_fn(move || {
                let dst = numbers.next()?;
                let (src, len) = numbers
                    .next()
                    .zip(numbers.next())
                    .expect("incomplete map entry");

                Some((src..src + len, dst..dst + len))
            });

            Some(array_chunks)
        })
    };

    (seeds, std::iter::from_fn(next_map))
}

#[yaah::aoc(day5, part1)]
fn part_one(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    let seeds = seeds.map(|seed| {
        maps.clone().fold(seed, |idx, mut map| {
            match map.find(|(src, _)| src.contains(&idx)) {
                Some((src, dst)) => idx - src.start + dst.start,
                None => idx,
            }
        })
    });

    seeds.min().expect("no seeds")
}

#[yaah::aoc(day5, part2)]
fn part_two(input: &str) -> u64 {
    let (mut seeds, maps) = parse_input(input);

    let mut maps: Vec<Vec<(Range<u64>, Range<u64>)>> =
        maps.map(|entries| entries.collect()).collect();
    maps.iter_mut()
        .for_each(|map| map.sort_unstable_by_key(|(src, _)| src.start));
    let mut ranges: Vec<Range<u64>> = std::iter::from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(s, len)| s..s + len)
        .collect();

    for map_entries in maps {
        let mut destinations = vec![];
        for mut range in ranges {
            // source mapping, destination mapping
            for (src, dst) in &map_entries {
                if range.is_empty() {
                    break;
                }

                let overlap = src.start.max(range.start)..range.end.min(src.end);

                // if there is space before the source mapping, add it
                let before = range.start..overlap.start.min(range.end);
                if !before.is_empty() {
                    destinations.push(before);
                }

                // if the source mapping is partially contained in the range, add the corresponding
                // part of the destination
                if !overlap.is_empty() {
                    let dst_start = dst.start + overlap.start - src.start;
                    let dst_end = dst_start + overlap.clone().count() as u64;
                    destinations.push(dst_start..dst_end);
                    // move the range to the end of the source mapping
                    range.start = overlap.end;
                }
            }

            // if there is space after the last source mapping, add it
            if !range.is_empty() {
                destinations.push(range);
            }
        }
        ranges = destinations;
    }

    ranges.into_iter().map(|r| r.start).min().expect("no seeds")
}
