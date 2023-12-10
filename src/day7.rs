#[repr(u8, align(2))]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard([u8; 5], u16),
    Pair([u8; 5], u16),
    TwoPair([u8; 5], u16),
    ThreeOfKind([u8; 5], u16),
    FullHouse([u8; 5], u16),
    FourOfKind([u8; 5], u16),
    FiveOfKind([u8; 5], u16),
}

impl Hand {
    fn bid(&self) -> u16 {
        match self {
            Hand::HighCard(_, bid) => *bid,
            Hand::Pair(_, bid) => *bid,
            Hand::TwoPair(_, bid) => *bid,
            Hand::ThreeOfKind(_, bid) => *bid,
            Hand::FullHouse(_, bid) => *bid,
            Hand::FourOfKind(_, bid) => *bid,
            Hand::FiveOfKind(_, bid) => *bid,
        }
    }
}

#[yaah::aoc(day7, part1)]
fn part_one(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| (&line.as_bytes()[0..5], &line[6..]))
        .map(|(hand, bid)| {
            let cards = {
                let mut buf = [0; 5];
                let mut cards = hand.into_iter().map(|card| match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 9,
                    b'T' => 8,
                    b'2'..=b'9' => card - b'2',
                    _ => unreachable!(),
                });
                buf.fill_with(|| cards.next().unwrap());

                buf
            };

            let counts = cards.into_iter().fold([0; 13], |mut counts, card| {
                counts[card as usize] += 1;
                counts
            });

            let kind = match counts.into_iter().max().unwrap() {
                5 => Hand::FiveOfKind,
                4 => Hand::FourOfKind,
                3 if counts.contains(&2) => Hand::FullHouse,
                3 => Hand::ThreeOfKind,
                _ if counts.iter().filter(|&&c| c == 2).count() == 2 => Hand::TwoPair,
                2 => Hand::Pair,
                1 => Hand::HighCard,
                _ => unreachable!(),
            };

            kind(cards, bid.parse().expect("invalid bid"))
        })
        .collect();

    hands.sort_unstable();
    (1..)
        .zip(hands)
        .map(|(rank, hand)| rank * hand.bid() as u32)
        .sum()
}

#[yaah::aoc(day7, part2)]
fn part_two(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| (&line.as_bytes()[0..5], &line[6..]))
        .map(|(hand, bid)| {
            let cards = {
                let mut buf = [0; 5];
                let mut cards = hand.into_iter().map(|card| match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    b'2'..=b'9' => card - b'2' + 1,
                    _ => unreachable!(),
                });
                buf.fill_with(|| cards.next().unwrap());

                buf
            };

            let counts = cards.into_iter().fold([0; 13], |mut counts, card| {
                counts[card as usize] += 1;
                counts
            });

            let max = counts.into_iter().skip(1).max().unwrap();
            let kind = match (counts[0], max) {
                (5, _) | (4, _) | (3, 2) | (2, 3) | (1, 4) | (_, 5) => Hand::FiveOfKind,
                (3, 1) | (2, 2) | (1, 3) | (0, 4) => Hand::FourOfKind,
                (1, _) if counts.iter().filter(|&&c| c == 2).count() == 2 => Hand::FullHouse,
                (_, 3) if counts.contains(&2) => Hand::FullHouse,
                (2, 1) | (1, 2) | (0, 3) => Hand::ThreeOfKind,
                (0, 2) if counts.iter().filter(|&&c| c == 2).count() == 2 => Hand::TwoPair,
                (1, 1) | (0, 2) => Hand::Pair,
                (0, 1) => Hand::HighCard,
                _ => unreachable!(),
            };

            kind(cards, bid.parse().expect("invalid bid"))
        })
        .collect();

    hands.sort_unstable();
    (1..)
        .zip(hands)
        .map(|(rank, hand)| rank * hand.bid() as u32)
        .sum()
}
