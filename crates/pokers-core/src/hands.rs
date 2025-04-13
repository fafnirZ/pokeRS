use crate::cards::Card;
use std::cmp::Ordering;

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

    let _ordered_cards = sort_cards(cards);

    println!("ordered {:?}", _ordered_cards);    

    Ok(Hand::HighCard)
}

fn sort_cards(cards: Vec<Card>) -> Vec<Card> {
    // sort by number then suits
    let mut _ordered_cards = cards.clone();
    _ordered_cards.sort_by(|a, b| {
        let a_num = a.number;
        let b_num = b.number;
        let a_suit = a.suit;
        let b_suit = b.suit;
        
        let num_cmp = a_num.partial_cmp(&b_num).unwrap();
        match num_cmp {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let suit_cmp = a_suit.partial_cmp(&b_suit).unwrap();
                suit_cmp   
            }
        }
    });
    _ordered_cards
}