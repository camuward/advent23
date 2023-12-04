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

#[yaah::aoc(day4, part2, naive)]
fn part_two(input: &str) -> u32 {
    let count = input.lines().count();
    let mut copies = vec![1; count];

    for (current_line, line) in input.lines().enumerate() {
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
            .count();

        let this_card_copies = copies[current_line];
        for card_copies in copies.iter_mut().skip(current_line + 1).take(count_winning) {
            *card_copies += this_card_copies;
        }
    }

    copies.into_iter().sum()
}
