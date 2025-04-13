#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub number: CardNumber,
}

impl Card {
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
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

    }
}