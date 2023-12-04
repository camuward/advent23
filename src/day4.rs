#[yaah::aoc(day4, part1, naive)]
fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_card, contents) = line.split_once(':').expect("no card id");
            let (winning, your_hand) = contents.split_once('|').expect("no winning separator");

            let winning: Vec<u32> = winning
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
                .expect("invalid winning number");

            let count_winning = your_hand
                .split_whitespace()
                .map(|s| s.parse().expect("invalid card number"))
                .filter(|card| winning.contains(card))
                .count() as u32;

            match count_winning {
                0 => 0,
                n => 2u32.pow(n - 1),
            }
        })
        .sum()
}
