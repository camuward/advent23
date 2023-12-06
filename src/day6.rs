#[yaah::aoc(day6, part1)]
fn part_one(input: &str) -> u32 {
    let (time, distance) = input
        .split_once("Distance:")
        .expect("missing distance label");

    let times = time
        .strip_prefix("Time:")
        .expect("missing time label")
        .split_whitespace()
        .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
        .map(|n| n.parse().unwrap());

    let distances = distance
        .split_whitespace()
        .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
        .map(|n| n.parse().unwrap());

    let records = times.zip(distances);

    // find both roots using quadratic formula
    // hold = (time +- sqrt(time^2 - 4 * distance)) / 2
    let winning_time_ranges = records.map(|(time, distance): (i32, i32)| {
        let discriminant = time * time - 4 * distance;
        let root = (discriminant as f64).sqrt();
        let mut t1 = (time as f64 - root) / 2.0;
        let mut t2 = (time as f64 + root) / 2.0;

        if (t1.fract() - t1.ceil().fract()).abs() < 1e-6 {
            t1 += 1.0;
        }

        if (t2.fract() - t2.floor().fract()).abs() < 1e-6 {
            t2 -= 1.0;
        }

        t1.ceil() as u32..=t2 as u32
    });

    winning_time_ranges
        .map(|range| range.count() as u32)
        .product()
}

#[yaah::aoc(day6, part2)]
fn part_two(input: &str) -> u32 {
    let (time, distance) = input
        .split_once("Distance:")
        .expect("missing distance label");

    let time: i64 = time
        .strip_prefix("Time:")
        .expect("missing time label")
        .split_whitespace()
        .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: i64 = distance
        .split_whitespace()
        .take_while(|s| s.bytes().all(|b| b.is_ascii_digit()))
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();

    // find both roots using quadratic formula
    // hold = (time +- sqrt(time^2 - 4 * distance)) / 2
    let discriminant = time * time - 4 * distance;
    let root = (discriminant as f64).sqrt();
    let mut t1 = (time as f64 - root) / 2.0;
    let mut t2 = (time as f64 + root) / 2.0;

    if (t1.fract() - t1.ceil().fract()).abs() < 1e-6 {
        t1 += 1.0;
    }

    if (t2.fract() - t2.floor().fract()).abs() < 1e-6 {
        t2 -= 1.0;
    }

    let range = t1.ceil() as u32..=t2 as u32;
    range.count() as u32
}
