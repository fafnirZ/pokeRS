use itertools::Itertools;
use pokers_core::cards::{Card, CardNumber, Suit};
use std::collections::HashSet;

pub fn permute_royal_flush() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for suit in Card::get_suits() {
        set.insert(vec![
            Card {
                suit: suit,
                number: CardNumber::Ten,
            },
            Card {
                suit: suit,
                number: CardNumber::Jack,
            },
            Card {
                suit: suit,
                number: CardNumber::Queen,
            },
            Card {
                suit: suit,
                number: CardNumber::King,
            },
            Card {
                suit: suit,
                number: CardNumber::Ace,
            },
        ]);
    }
    return set;
}

// 2,3,4,5,6,7,8,9 can be starting values
// 0,1,2,3,4,5,6,7 are the indexes
pub fn permute_straight_flush() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for suit in Card::get_suits() {
        for idx in 0..=7 {
            let start = idx;
            let card_nums = Card::get_numbers();
            let number_slice = &card_nums[start..start + 5];
            let mut hand = Vec::new();
            for &card_num in number_slice {
                hand.push(Card {
                    suit: suit,
                    number: card_num,
                });
            }
            set.insert(hand);
        }
    }
    return set;
}

// returns vec size 4
pub fn permute_four_of_a_kind() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for card_num in Card::get_numbers() {
        let mut hand: Vec<Card> = Vec::new();
        for suit in Card::get_suits() {
            hand.push(Card {
                suit: suit,
                number: card_num,
            });
        }
        set.insert(hand);
    }
    return set;
}

// for every card number J for Triple, (4C3 suit)
//    for ever card number K not J for double (4C2 suit)
// TODO after finish: permute triple, permute_pairs
pub fn permute_full_house() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for triple in permute_three_of_a_kind() {
        let triple_first_card = triple.first().unwrap();
        for double in permute_pair() {
            let double_first_card = double.first().unwrap();
            if triple_first_card.number == double_first_card.number {
                continue
            }
            let combined_vec: Vec<Card> = triple
                                        .clone()
                                        .into_iter()
                                        .chain(
                                            double
                                            .clone()
                                            .into_iter()
                                        )
                                        .collect();

            set.insert(combined_vec);
        }
    }
    return set
}
// pub fn permute_flush() -> HashSet<Vec<Card>> {
//     let mut set: HashSet<Vec<Card>> = HashSet::new();
//     return set
// }
// pub fn permute_straight() -> HashSet<Vec<Card>> {
//     let mut set: HashSet<Vec<Card>> = HashSet::new();
//     return set
// }

// for ever number
// returns Vec size 3
pub fn permute_three_of_a_kind() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for card_num in Card::get_numbers() {
        let suits_vec = Card::get_suits().to_vec(); // E0716
        let suit_combinations: Vec<Vec<&Suit>> = suits_vec.iter().combinations(3).collect();
        // for pick_3 in suit_combinations
        for suit_combination in suit_combinations {
            let hand: Vec<Card> = suit_combination
                .iter()
                .map(|suit| Card {
                    suit: **suit,
                    number: card_num,
                })
                .collect();
            set.insert(hand);
        }
    }
    return set;
}

pub fn permute_pair() -> HashSet<Vec<Card>> {
    let mut set: HashSet<Vec<Card>> = HashSet::new();
    for card_num in Card::get_numbers() {
        let suits_vec = Card::get_suits().to_vec(); // E0716
        let suit_combinations: Vec<Vec<&Suit>> = suits_vec.iter().combinations(2).collect();
        // for pick_2 in suit_combinations
        for suit_combination in suit_combinations {
            let hand: Vec<Card> = suit_combination
                .iter()
                .map(|suit| Card {
                    suit: **suit,
                    number: card_num,
                })
                .collect();
            set.insert(hand);
        }
    }
    return set;
}

// pub fn permute_two_pair() -> HashSet<Vec<Card>> {
//     let mut set: HashSet<Vec<Card>> = HashSet::new();
//     return set
// }

#[cfg(test)]
mod tests {
    use core::num;

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
        assert!(res.len() == 4 * 8);
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
        let num_cards = 13;
        let combination_triple = 4*num_cards;
        let combination_double = 6*(num_cards-1);
        
        // given triple is "2"
        // double cannot be also "2"
        //  so its actually 4C2 * (13-1) for combination of double, 
        //  given card number cannot be identical to triple
        assert!(res.len() == combination_double*combination_triple);
    }

    // #[test]
    // fn test_flush() {
    //     let res = permute_flush();
    //     println!("{:?}", res);
    //     assert!(res.len() == 999);
    // }
    // #[test]
    // fn test_straight() {
    //     let res = permute_straight();
    //     println!("{:?}", res);
    //     assert!(res.len() == 999);
    // }

    #[test]
    fn test_three_of_a_kind() {
        let res = permute_three_of_a_kind();
        println!("{:?}", res);
        let four_c_three = 4;
        assert!(res.len() == (four_c_three*13));
    }

    #[test]
    fn test_pair() {
        let res = permute_pair();
        println!("{:?}", res);
        let four_c_two = 6;
        assert!(res.len() == four_c_two * 13);
    }

    // #[test]
    // fn test_two_pair() {
    //     let res = permute_two_pair();
    //     println!("{:?}", res);
    //     assert!(res.len() == 999);
    // }
}
