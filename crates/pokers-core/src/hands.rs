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
    InvalidCardCount(&'static str),
}
pub fn determine_hand(cards: Vec<Card>) -> Result<Hand, HandError> {
    if cards.len() != 5 {
       return Err(HandError::InvalidCardCount("Invalid card count")); 
    }

    Ok(Hand::HighCard)
}
