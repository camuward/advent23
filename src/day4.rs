use alloc::vec::Vec;

#[yaah::aoc(day4, part1)]
pub fn part_one(input: &str) -> u32 {
    let mut winning_numbers: Vec<u32> = Vec::new();
    input
        .lines()
        .map(|line| {
            let (_id, card) = line.split_once(':').expect("no card id");
            let (winning, your_card) = card.split_once('|').expect("no winning separator");

            winning_numbers.clear();
            winning_numbers.extend(
                winning
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().expect("invalid winning number")),
            );

            let count_winning = your_card
                .split_whitespace()
                .map(|s| s.parse().expect("invalid card number"))
                .filter(|number| winning_numbers.contains(number))
                .count() as u32;

            match count_winning {
                0 => 0,
                n => 2u32.pow(n - 1),
            }
        })
        .sum()
}

#[yaah::aoc(day4, part2)]
pub fn part_two(input: &str) -> u32 {
    let count = input.lines().count();
    let mut copies = alloc::vec![1; count];

    let mut winning_numbers: Vec<u32> = Vec::new();
    for (current_line, line) in input.lines().enumerate() {
        let (_id, card) = line.split_once(':').expect("no card id");
        let (winning, your_card) = card.split_once('|').expect("no winning separator");

        winning_numbers.clear();
        winning_numbers.extend(
            winning
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("invalid winning number")),
        );

        let count_winning = your_card
            .split_whitespace()
            .map(|s| s.parse().expect("invalid card number"))
            .filter(|number| winning_numbers.contains(number))
            .count();

        let this_card_copies = copies[current_line];
        for card_copies in copies.iter_mut().skip(current_line + 1).take(count_winning) {
            *card_copies += this_card_copies;
        }
    }

    copies.into_iter().sum()
}
