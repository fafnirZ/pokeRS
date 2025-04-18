
use pokers_core::cards::{Card, CardNumber, Suit};
use std::{collections::HashSet};

pub fn permute_royal_flush() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for suit in Card::get_suits() {
        set.insert(vec![
            Card{suit: suit, number: CardNumber::Ten},
            Card{suit: suit, number: CardNumber::Jack},
            Card{suit: suit, number: CardNumber::Queen},
            Card{suit: suit, number: CardNumber::King},
            Card{suit: suit, number: CardNumber::Ace},
        ]);
    }
    return set
}

// 2,3,4,5,6,7,8,9 can be starting values
// 0,1,2,3,4,5,6,7 are the indexes 
pub fn permute_straight_flush() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for suit in Card::get_suits() {
        for idx in 0..=7 {
            let start = idx;
            let card_nums = Card::get_numbers();
            let number_slice = &card_nums[start..start+4];
            for &card_num in number_slice {
                set.insert(vec![
                    Card{suit: suit, number: card_num},
                ]);
            }
        }
    }
    return set
}

//
pub fn permute_four_of_a_kind() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for card_num in Card::get_numbers() {
        let mut hand: Vec<Card> = Vec::new();
        for suit in Card::get_suits() {
            hand.push(
                Card{suit: suit, number: card_num}
            );
        } 
        set.insert(hand);
    }
    return set
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_royal_flush() {
        let res = permute_royal_flush();
        println!("{:?}", res);
        assert!(res.len() == 4);
        // assert!(1==0);
    }
    #[test]
    fn test_straight_flush() {
        let res = permute_straight_flush();
        println!("{:?}", res);
        assert!(res.len() == 28);
        // assert!(1==0);
    }

    #[test]
    fn test_four_of_a_kind() {
        let res = permute_four_of_a_kind();
        println!("{:?}", res);
        assert!(res.len() == 13);
        // assert!(1==0);
    }
    
}