fn parse_id(id: &str) -> Option<u32> {
    let id = id.strip_prefix("Game ")?;
    let id = id.parse().ok()?;
    Some(id)
}

fn count_per_color(subset: &str) -> [u32; 3] {
    subset.split(", ").fold([0, 0, 0], |[mut r, mut g, mut b], count_color| {
        let (count, color) = count_color.split_once(' ').expect("no count");
        let count: u32 = count.parse().expect("count is not a number");

        match color {
            "red" => r += count,
            "green" => g += count,
            "blue" => b += count,
            _ => unreachable!("unknown color"),
        };

        [r, g, b]
    })
}

#[yaah::aoc(day2, part1)]
fn part_one(input: &str) -> u32 {
    let possible = input.lines().zip(1..).filter_map(|(line, id)| {
        let (_id, set_of_cubes) = line.split_once(": ").expect("no game id");
        debug_assert_eq!(parse_id(_id), Some(id));

        let mut subsets = set_of_cubes.split("; ").map(count_per_color);
        subsets.all(|[r, g, b]| r <= 12 && g <= 13 && b <= 14).then(|| id)
    });

    possible.sum()
}
