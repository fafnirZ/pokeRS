use crate::cards::Card;

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
};


