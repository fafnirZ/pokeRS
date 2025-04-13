use crate::cards::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Hand {
    pub fn rank(&self) -> usize {
        // highest rank wins
        match self {
            Hand::HighCard => 0,
            Hand::Pair => 1,
            Hand::TwoPair => 2,
            Hand::ThreeOfAKind => 3,
            Hand::Straight => 4,
            Hand::Flush => 5,
            Hand::FullHouse => 6,
            Hand::FourOfAKind => 7,
            Hand::StraightFlush => 8,
            Hand::RoyalFlush => 9,
        }
    }
}

#[derive(Debug)]
pub enum HandError {
    // InvalidCardCount(&'static str),
    InvalidCardCount(String),
}
pub fn determine_hand(cards: Vec<Card>) -> Result<Hand, HandError> {
    if cards.len() != 5 {
       return Err(HandError::InvalidCardCount(
            format!("Invalid card count expected: 5, got {}", cards.len())
        )); 
    }

    let mut _ordered_cards = cards.clone();
    _ordered_cards.sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    println!("ordered {:?}", _ordered_cards);    

    Ok(Hand::HighCard)
}
