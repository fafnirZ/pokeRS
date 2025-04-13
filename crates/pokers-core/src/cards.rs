
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

pub enum CardNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}


pub struct Card {
    suit: Suit,
    number: CardNumber,
}

impl Card {
    pub fn generate_all_cards() -> [Card;52] {
        let mut all_cards = [Card {
            suit: Suit::Diamond,
            number: CardNumber::One,
        }; 52]; // Initialize with a dummy Card, then overwrite.

        // no enum.values() equivalent in rust
        let suits = [
            Suit::Diamond,
            Suit::Club,
            Suit::Heart,
            Suit::Spade
        ];
        let card_nums = [
            CardNumber::One,
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