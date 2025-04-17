#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardNumber {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14, // well its technically 1, BUT power wise its 14
}

impl CardNumber {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNumber,
}

impl Card {

    pub fn get_suits() -> [Suit;4] {
        return [
            Suit::Diamond,
            Suit::Club,
            Suit::Heart,
            Suit::Spade
        ]; 
    }

    pub fn get_numbers() -> [CardNumber;13] {
        return [
            CardNumber::Two,
            CardNumber::Three,
            CardNumber::Four,
            CardNumber::Five,
            CardNumber::Six,
            CardNumber::Seven,
            CardNumber::Eight,
            CardNumber::Nine,
            CardNumber::Ten,
            CardNumber::Jack,
            CardNumber::Queen,
            CardNumber::King,
            CardNumber::Ace 
        ]
    }

    pub fn generate_all_cards() -> [Card;52] {
        let mut all_cards = [Card {
            suit: Suit::Diamond,
            number: CardNumber::Two,
        }; 52]; // Initialize with a dummy Card, then overwrite.

        // no enum.values() equivalent in rust
        let suits = [
            Suit::Diamond,
            Suit::Club,
            Suit::Heart,
            Suit::Spade
        ];
        let card_nums = [
            CardNumber::Two,
            CardNumber::Three,
            CardNumber::Four,
            CardNumber::Five,
            CardNumber::Six,
            CardNumber::Seven,
            CardNumber::Eight,
            CardNumber::Nine,
            CardNumber::Ten,
            CardNumber::Jack,
            CardNumber::Queen,
            CardNumber::King,
            CardNumber::Ace
        ];

        let mut idx = 0;
        for suit in suits.iter() {
            for card_num in card_nums.iter() {
                all_cards[idx] = Card {
                    suit: *suit,
                    number: *card_num,
                };
                idx += 1;
            }
        }

        // ret
        all_cards
    }

    pub fn format_print_short(&self) -> String {
        let suit = match self.suit {
            Suit::Club => "C",
            Suit::Diamond => "D",
            Suit::Heart => "H",
            Suit::Spade => "S",
        };
        let number = match self.number {
            CardNumber::Two => "2",
            CardNumber::Three => "3",
            CardNumber::Four => "4",
            CardNumber::Five => "5",
            CardNumber::Six => "6",
            CardNumber::Seven => "7",
            CardNumber::Eight => "8",
            CardNumber::Nine => "9",
            CardNumber::Ten => "10",
            CardNumber::Jack => "J",
            CardNumber::Queen => "Q",
            CardNumber::King => "K",
            CardNumber::Ace => "A",
        };

        format!("{}{}", number, suit)
    }
}


