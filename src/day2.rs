fn parse_id(id: &str) -> Option<u32> {
    let id = id.strip_prefix("Game ")?;
    let id = id.parse().ok()?;
    Some(id)
}

fn count_per_color(subset: &str) -> [u32; 3] {
    subset
        .split(", ")
        .fold([0, 0, 0], |[mut r, mut g, mut b], count_color| {
            let (count, color) = count_color.split_once(' ').expect("no color count");
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
    let possible_games = input.lines().zip(1..).filter_map(|(line, id)| {
        let (_id, set_of_cubes) = line.split_once(": ").expect("no game id");
        debug_assert_eq!(parse_id(_id), Some(id));

        let mut subsets = set_of_cubes.split("; ").map(count_per_color);
        subsets
            .all(|[r, g, b]| r <= 12 && g <= 13 && b <= 14)
            .then(|| id)
    });

    possible_games.sum()
}

#[yaah::aoc(day2, part2)]
fn part_two(input: &str) -> u32 {
    let powers = input.lines().map(|line| {
        let (_id, set_of_cubes) = line.split_once(": ").expect("no game id");

        let subsets = set_of_cubes.split("; ").map(count_per_color);
        let required = subsets
            .reduce(|[r1, g1, b1], [r2, g2, b2]| [r1.max(r2), g1.max(g2), b1.max(b2)])
            .expect("game record empty");

        let [r, g, b] = required;
        r * g * b
    });

    powers.sum()
}
