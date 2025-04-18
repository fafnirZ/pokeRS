
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
            let number_slice = &card_nums[start..start+5];
            let mut hand = Vec::new();
            for &card_num in number_slice {
                hand.push(Card{suit: suit, number: card_num});
            }
            set.insert(hand);
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


pub fn permute_full_house() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    return set
}
pub fn permute_flush() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    return set
}
pub fn permute_straight() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    return set
}
pub fn permute_three_of_a_kind() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    return set
}

pub fn permute_two_pair() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    return set
}
pub fn permute_pair() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
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
    }
    #[test]
    fn test_straight_flush() {
        let res = permute_straight_flush();
        println!("{:?}", res);
        assert!(res.len() == 4*8);
    }

    #[test]
    fn test_four_of_a_kind() {
        let res = permute_four_of_a_kind();
        println!("{:?}", res);
        assert!(res.len() == 13);
    }
    
    #[test]
    fn test_full_house() {
        let res = permute_full_house();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }
    #[test]
    fn test_flush() {
        let res = permute_flush();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }
    #[test]
    fn test_straight() {
        let res = permute_straight();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }
    #[test]
    fn test_three_of_a_kind() {
        let res = permute_three_of_a_kind();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }
    #[test]
    fn test_two_pair() {
        let res = permute_two_pair();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }

    #[test]
    fn test_pair() {
        let res = permute_pair();
        println!("{:?}", res);
        assert!(res.len() == 999);
    }

}