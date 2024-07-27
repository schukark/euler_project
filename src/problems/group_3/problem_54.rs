use std::fs;

#[derive(Clone, PartialEq, Hash, Debug)]
enum Suit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
enum Value {
    Number(i128),
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn to_numerical(self) -> i128 {
        match self {
            Value::Number(value) => value,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_numerical().cmp(&other.to_numerical()))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, PartialEq, Hash, Debug)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn new(value: char, suit: char) -> Card {
        let new_suit = match suit {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            _ => panic!("Invalid suit"),
        };

        let new_value = match value {
            '2'..='9' => Value::Number(value.to_digit(10).unwrap() as i128),
            'T' => Value::Number(10),
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' => Value::Ace,
            _ => panic!("Invalid card value"),
        };

        Card {
            value: new_value,
            suit: new_suit,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn from_10_cards(cards: Vec<Card>) -> (Hand, Hand) {
        let mut hand1: Hand = Hand {
            cards: cards[..5].to_vec(),
        };
        hand1.cards.sort();
        let mut hand2: Hand = Hand {
            cards: cards[5..].to_vec(),
        };
        hand2.cards.sort();

        (hand1, hand2)
    }

    #[allow(dead_code)]
    fn from_string(card_string: &str) -> Hand {
        let mut hand = Hand {
            cards: card_string
                .split_whitespace()
                .map(|card| Card::new(card.chars().next().unwrap(), card.chars().nth(1).unwrap()))
                .collect::<Vec<_>>(),
        };
        hand.cards.sort();
        hand
    }

    fn is_royal_flush(&self) -> bool {
        self.is_flush()
            && self.cards.iter().map(|a| a.value).collect::<Vec<_>>()
                == vec![
                    Value::Number(10),
                    Value::Jack,
                    Value::Queen,
                    Value::King,
                    Value::Ace,
                ]
    }

    fn is_straight_flush(&self) -> Option<i128> {
        self.is_straight().filter(|&_highest| self.is_flush())
    }

    fn is_four(&self) -> Option<i128> {
        let counts = self.cards.iter().fold([0; 15], |mut acc, card| {
            acc[card.value.to_numerical() as usize] += 1;
            acc
        });

        if !counts.iter().any(|&count| count == 4) {
            None
        } else {
            Some(counts.to_vec().iter().position(|x| *x == 4).unwrap() as i128 + 1)
        }
    }

    fn is_full_house(&self) -> Option<i128> {
        let counts = self.cards.iter().fold([0; 15], |mut acc, card| {
            acc[card.value.to_numerical() as usize] += 1;
            acc
        });

        if !(counts.contains(&3) && counts.contains(&2)) {
            None
        } else {
            Some(counts.to_vec().iter().position(|x| *x == 3).unwrap() as i128 + 1)
        }
    }

    fn is_flush(&self) -> bool {
        self.cards
            .iter()
            .all(|card| card.suit == self.cards.first().unwrap().suit)
    }

    fn is_straight(&self) -> Option<i128> {
        let mut values: Vec<i128> = self
            .cards
            .iter()
            .map(|card| card.value.to_numerical())
            .collect();
        values.sort();

        if !values.windows(2).all(|w| w[1] == w[0] + 1) {
            None
        } else {
            Some(*values.last().unwrap())
        }
    }

    fn is_three(&self) -> Option<i128> {
        let counts = self.cards.iter().fold([0; 15], |mut acc, card| {
            acc[card.value.to_numerical() as usize] += 1;
            acc
        });

        if !counts.iter().any(|&count| count == 3) {
            None
        } else {
            Some(counts.to_vec().iter().position(|x| *x == 3).unwrap() as i128 + 1)
        }
    }

    fn is_two_pairs(&self) -> Option<i128> {
        let counts = self.cards.iter().fold([0; 15], |mut acc, card| {
            acc[card.value.to_numerical() as usize] += 1;
            acc
        });

        if counts.iter().filter(|&&count| count == 2).count() != 2 {
            None
        } else {
            let mut max_ = 0;
            counts
                .to_vec()
                .iter()
                .enumerate()
                .for_each(|(index, value)| {
                    if *value == 2 {
                        max_ = index;
                    }
                });

            Some(max_ as i128)
        }
    }

    fn is_two(&self) -> Option<i128> {
        let counts = self.cards.iter().fold([0; 15], |mut acc, card| {
            acc[card.value.to_numerical() as usize] += 1;
            acc
        });

        if !counts.iter().any(|&count| count == 2) {
            None
        } else {
            Some(counts.to_vec().iter().position(|x| *x == 2).unwrap() as i128 + 1)
        }
    }

    pub fn to_numerical(&self) -> i128 {
        if self.is_royal_flush() {
            1000
        } else if let Some(highest) = self.is_straight_flush() {
            900 + highest
        } else if let Some(highest) = self.is_four() {
            800 + highest
        } else if let Some(highest) = self.is_full_house() {
            700 + highest
        } else if self.is_flush() {
            600
        } else if let Some(highest) = self.is_straight() {
            500 + highest
        } else if let Some(highest) = self.is_three() {
            400 + highest
        } else if let Some(highest) = self.is_two_pairs() {
            300 + highest
        } else if let Some(highest) = self.is_two() {
            200 + highest
        } else {
            1
        }
    }
}

#[derive(Debug)]
struct Matchup {
    hands: (Hand, Hand),
}

impl Matchup {
    pub fn new(hands: (Hand, Hand)) -> Matchup {
        Matchup { hands }
    }

    fn winner(&self) -> i128 {
        let hand1_rank = self.hands.0.to_numerical();
        let hand2_rank = self.hands.1.to_numerical();

        match hand1_rank.cmp(&hand2_rank) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => {
                // Compare individual cards if ranks are the same
                for i in (0..5).rev() {
                    match self.hands.0.cards[i].cmp(&self.hands.1.cards[i]) {
                        std::cmp::Ordering::Greater => return 1,
                        std::cmp::Ordering::Less => return 2,
                        std::cmp::Ordering::Equal => continue,
                    }
                }
                panic!("Should never be reached due to guaranteed no ties")
            }
        }
    }
}

fn parse_file(file: &str) -> Vec<Matchup> {
    fs::read_to_string(file)
        .unwrap()
        .split('\n')
        .map(|a| {
            a.split(' ')
                .map(|b| Card::new(b.chars().next().unwrap(), b.chars().nth(1).unwrap()))
                .collect::<Vec<_>>()
        })
        .map(Hand::from_10_cards)
        .map(Matchup::new)
        .collect::<Vec<_>>()
}

fn _solve(file: &str) -> i128 {
    parse_file(file)
        .iter()
        .map(|matchup| matchup.winner())
        .filter(|&winner| winner == 1)
        .count() as i128
}

#[test]
pub fn test_royal_flush() {
    let hand1 = Hand::from_string("TH AH KH QH JH");
    assert!(hand1.is_royal_flush());

    let hand2 = Hand::from_string("TH AH KH QH JD");
    assert!(!hand2.is_royal_flush());

    let hand3 = Hand::from_string("TH AH KH QH 6H");
    assert!(!hand3.is_royal_flush());

    let hand4 = Hand::from_string("TH AH KH QH 7H");
    assert!(!hand4.is_royal_flush());
}

#[test]
pub fn test_straight_flush() {
    let hand1 = Hand::from_string("TH AH KH QH JH");
    assert!(hand1.is_straight_flush().is_some());

    let hand2 = Hand::from_string("TH 9H KH QH JH");
    assert!(hand2.is_straight_flush().is_some());

    let hand3 = Hand::from_string("3H 4H 6H 7H 5H");
    assert!(hand3.is_straight_flush().is_some());

    let hand4 = Hand::from_string("3H 4H 6S 7H 5H");
    assert!(hand4.is_straight_flush().is_none());
}

#[test]
pub fn test_four() {
    let hand1 = Hand::from_string("2S 2C 2D 2H AD");
    assert!(hand1.is_four().is_some());

    let hand2 = Hand::from_string("TS TC AD TH TD");
    assert!(hand2.is_four().is_some());

    let hand3 = Hand::from_string("9S TC AD TH TD");
    assert!(hand3.is_four().is_none());

    let hand4 = Hand::from_string("4S 4C JD 4H 5D");
    assert!(hand4.is_four().is_none());
}

#[test]
pub fn test_winner_determination() {
    let cases = vec![
        // Royal Flush vs. Straight Flush
        (
            Hand::from_string("TH AH KH QH JH"), // Royal Flush
            Hand::from_string("9H 8H 7H 6H 5H"), // Straight Flush
            1,
        ),
        // Four of a Kind vs. Full House
        (
            Hand::from_string("9C 9D 9H 9S 7D"), // Four of a Kind
            Hand::from_string("8C 8D 8H 7S 7H"), // Full House
            1,
        ),
        // Straight vs. Flush
        (
            Hand::from_string("6C 7D 8H 9S TD"), // Straight
            Hand::from_string("2C 4C 6C 8C KC"), // Flush
            2,
        ),
        // Two Pair vs. One Pair
        (
            Hand::from_string("3H 3D 5S 5H 8C"), // Two Pair
            Hand::from_string("2H 2D 4S 8H QC"), // One Pair
            1,
        ),
        // High Card vs. High Card
        (
            Hand::from_string("2C 3D 4H 8S KH"), // High Card K
            Hand::from_string("2H 3C 4S 8D QS"), // High Card Q
            1,
        ),
        // Low Straight vs. High Straight
        (
            Hand::from_string("AS 2D 3C 4H 5S"), // Low Straight
            Hand::from_string("9C TD JH QS KC"), // High Straight
            2,
        ),
        // Full House with different three-of-a-kind
        (
            Hand::from_string("3H 3D 3S 6C 6H"), // Full House, Threes over Sixes
            Hand::from_string("2C 2H 2S 9D 9C"), // Full House, Twos over Nines
            1,
        ),
        // Pair with different high cards
        (
            Hand::from_string("3H 3D 5S 8C KH"), // Pair of Threes, High Card King
            Hand::from_string("3S 3C 5D 8H QS"), // Pair of Threes, High Card Queen
            1,
        ),
        // Flush with different high cards
        (
            Hand::from_string("2H 4H 6H 8H QH"), // Flush, High Card Queen
            Hand::from_string("2C 4C 6C 8C JC"), // Flush, High Card Jack
            1,
        ),
        // Four of a Kind with different kicker
        (
            Hand::from_string("4H 4D 4S 4C 8H"), // Four of a Kind, Kicker 8
            Hand::from_string("4C 4S 4D 4H 7S"), // Four of a Kind, Kicker 7
            1,
        ),
        // Straight Flush vs. Straight Flush (higher rank)
        (
            Hand::from_string("QH JH TH 9H 8H"), // Straight Flush
            Hand::from_string("KH QH JH TH 9H"), // Straight Flush
            2,
        ),
        // Two Pair with different pairs
        (
            Hand::from_string("2H 2D 5S 5H 8C"), // Two Pair, 5s and 2s
            Hand::from_string("3H 3D 4S 4H AC"), // Two Pair, 4s and 3s
            1,
        ),
        // High Card with kicker comparison
        (
            Hand::from_string("2C 3D 4H 8S AH"), // High Card A, kicker 8
            Hand::from_string("2H 3C 4S 8D KH"), // High Card K, kicker 8
            1,
        ),
        // Full House with same three-of-a-kind, different pair
        (
            Hand::from_string("3H 3D 3S 9C 9H"), // Full House, Threes over Nines
            Hand::from_string("3C 3S 3D 8H 8S"), // Full House, Threes over Eights
            1,
        ),
        // Flush with all cards same suit, different ranks
        (
            Hand::from_string("2H 4H 6H 8H 9H"), // Flush, High Card 9
            Hand::from_string("2D 4D 6D 8D TD"), // Flush, High Card T
            2,
        ),
        // Three of a Kind vs. Two Pair
        (
            Hand::from_string("5C 5D 5H 7S 8C"), // Three of a Kind, 5s
            Hand::from_string("2H 2D 3S 3H 9C"), // Two Pair, 3s and 2s
            1,
        ),
        // High Card vs. High Card with no pairs or better
        (
            Hand::from_string("2C 3D 4H 6S 8H"), // High Card 8
            Hand::from_string("2H 3C 4S 7D 9S"), // High Card 9
            2,
        ),
    ];

    for (index, (hand1, hand2, expected_winner)) in cases.iter().enumerate() {
        let matchup = Matchup::new(((*hand1).clone(), (*hand2).clone()));
        let result = matchup.winner();
        println!("Test case {}: {:?}", index + 1, matchup);
        assert_eq!(
            result,
            *expected_winner,
            "Failed on case {}: {:?} vs {:?} - Expected winner: {}, Got: {}",
            index + 1,
            hand1,
            hand2,
            expected_winner,
            result
        );
    }
}

#[test]
fn test_file_parsing() {
    let matchups = parse_file("src/txts/prob_54.txt");
    for (index, matchup) in matchups.iter().enumerate() {
        println!("Matchup {}: {:?}", index + 1, matchup);
    }
    assert!(!matchups.is_empty());
}

pub fn solve() -> i128 {
    _solve("src/txts/prob_54.txt")
}
