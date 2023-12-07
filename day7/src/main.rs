use std::cmp::Ordering;
use std::collections::HashMap;

macro_rules! enum_with_traits {
    ($name:ident, $($variant:ident),+) => {
        #[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Copy, Clone)]
        enum $name {
            $($variant),+
        }
    };
}

enum_with_traits!(
    HandType,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
);
enum_with_traits!(
    Card, Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
);

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    with_joker: bool,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = HashMap::new();
        for &card in &self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }
        if self.with_joker {
            counts.remove(&Card::Joker);
        }

        let mut counts = counts.values().cloned().collect::<Vec<usize>>();
        counts.sort();

        if self.with_joker {
            let missing = 5 - counts.iter().sum::<usize>();

            match counts.last_mut() {
                None => counts.push(5),
                Some(v) => *v += missing,
            }
        }

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

fn parse_card(c: char, with_joker: bool) -> Result<Card, String> {
    match c {
        '2' => Ok(Card::Two),
        '3' => Ok(Card::Three),
        '4' => Ok(Card::Four),
        '5' => Ok(Card::Five),
        '6' => Ok(Card::Six),
        '7' => Ok(Card::Seven),
        '8' => Ok(Card::Eight),
        '9' => Ok(Card::Nine),
        'T' => Ok(Card::Ten),
        'J' => {
            if with_joker {
                Ok(Card::Joker)
            } else {
                Ok(Card::Jack)
            }
        }
        'Q' => Ok(Card::Queen),
        'K' => Ok(Card::King),
        'A' => Ok(Card::Ace),
        _ => Err(format!("Invalid card: {}", c)),
    }
}

fn parse_hand(line: &str, with_joker: bool) -> Result<Hand, String> {
    let (cards, bid) = line.split_once(' ').ok_or("Invalid line format")?;
    let cards = cards
        .chars()
        .map(|c| parse_card(c, with_joker))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| "Invalid number of cards")?;

    let bid = bid.parse::<u32>().map_err(|_| "Invalid bid format")?;

    Ok(Hand {
        cards,
        bid,
        with_joker,
    })
}

fn part_one(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| parse_hand(line, false))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    hands.sort();

    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| parse_hand(line, true))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    hands.sort();

    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn main() {
    let input = include_str!("input");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}
