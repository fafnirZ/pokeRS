#[cfg(test)]
mod tests {
    use crate::cards::{Card, CardNumber, Suit};
    use crate::hands::{Hand, determine_hand};

    #[test]
    fn test_royal_flush() {
        let mut cards: Vec<Card> = Vec::new();
        cards.push(Card {
            suit: Suit::Diamond,
            number: CardNumber::Two,
        });
        cards.push(Card {
            suit: Suit::Club,
            number: CardNumber::Two,
        });
        cards.push(Card {
            suit: Suit::Diamond,
            number: CardNumber::Three,
        });
        cards.push(Card {
            suit: Suit::Diamond,
            number: CardNumber::Ace,
        });
        cards.push(Card {
            suit: Suit::Diamond,
            number: CardNumber::Queen,
        });

        // result
        let _res = determine_hand(cards).unwrap();
        assert_eq!(_res, Hand::RoyalFlush);
    }
}