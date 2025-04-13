use crate::cards::{Card, CardNumber, Suit};
use std::{cmp::Ordering, collections::HashMap};

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
    let __is_royal_flush = is_royal_flush(&_ordered_cards);
    let __is_straight_flush = is_straight_flush(&_ordered_cards);
    let __is_straight = is_straight(&_ordered_cards);
    let __is_flush = is_flush(&_ordered_cards);

    let __n_of_a_kind = n_of_a_kind(&_ordered_cards);

    println!("ordered {:?}", _ordered_cards);  
    println!("is royal flush {:?}", __is_royal_flush);  
    println!("is straight flush {:?}", __is_straight_flush);  
    println!("is straight {:?}", __is_straight);  
    println!("is flush {:?}", __is_flush);

    println!("n of a kind {:?}", __n_of_a_kind);

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

// assume vec sorted
fn is_straight(cards: &Vec<Card>) -> bool {
    for (curr, next) in cards.iter().zip(cards.iter().skip(1)) {
        if next.number.value() - curr.number.value() > 1 {
            return false
        }
    }
    true
}

// assume len(vec) == 5
fn is_flush(cards: &Vec<Card>) -> bool {
    let _first_card = cards[0];
    let _first_card_suit = _first_card.suit;
    cards.iter().all(|c| c.suit == _first_card_suit)
}

fn is_royal_flush(cards: &Vec<Card>) -> bool {
    let numbers: Vec<CardNumber> = cards
                    .iter()
                    .map(|card| card.number)
                    .collect();

    let expected_numbers = vec![
        CardNumber::Ten,
        CardNumber::Jack,
        CardNumber::Queen,
        CardNumber::King,
        CardNumber::Ace,
    ];
    if numbers != expected_numbers{
        return false
    }      
    
    let all_suits_ace = cards
        .iter()
        .map(|card| card.suit)
        .all(|suit| suit == Suit::Spade);
    if !all_suits_ace {
        return false
    }
    return true
}

fn is_straight_flush(cards: &Vec<Card>) -> bool {
    if !is_flush(cards) {
        return false
    }
    if !is_straight(cards) {
        return false;
    }
    return true;
}

fn n_of_a_kind(cards: &Vec<Card>) -> HashMap<CardNumber, u8> {
    let mut hm: HashMap<CardNumber, u8> = HashMap::new();
    for card in cards.iter() {
        let card_num = card.number;
        hm.entry(card_num)
            .and_modify(|val| {*val += 1})
            .or_insert(1);
    }
    hm
}