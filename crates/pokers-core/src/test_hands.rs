#[cfg(test)]
mod tests {
    use crate::cards::{Card, CardNumber, Suit};
    use crate::hands::{Hand, determine_hand};

    #[test]
    fn test_royal_flush() {
        let suits = vec![
            Suit::Diamond,
            Suit::Club,
            Suit::Heart,
            Suit::Spade,
        ];
        for suit in suits.iter() {
            let mut cards: Vec<Card> = Vec::new();
            cards.push(Card {
                suit: *suit,
                number: CardNumber::Ace,
            });
            cards.push(Card {
                suit: *suit,
                number: CardNumber::King,
            });
            cards.push(Card {
                suit: *suit,
                number: CardNumber::Queen,
            });
            cards.push(Card {
                suit: *suit,
                number: CardNumber::Jack,
            });
            cards.push(Card {
                suit: *suit,
                number: CardNumber::Ten,
            });
            // result
            let _res = determine_hand(cards).unwrap();
            assert_eq!(_res, Hand::RoyalFlush);
        }
 
    }
}