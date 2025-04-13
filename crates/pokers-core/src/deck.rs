use crate::cards::Card;
use rand::seq::SliceRandom;
use rand::rng;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deck {
    pub cards: Vec<Card>,
}


impl Deck {
    pub fn init_shuffled() -> Deck {
        let all_cards_possible = Card::generate_all_cards();
        let mut card_vec = all_cards_possible.to_vec();
        card_vec.shuffle(&mut rng());
        Deck { cards: card_vec }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let deck = Deck::init_shuffled();
        println!("{:?}", deck);
        // assert!(false);
    }
}
