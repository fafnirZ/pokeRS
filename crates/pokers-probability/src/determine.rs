use pokers_core::cards::{Card};
use pokers_core::hands::{Hand, determine_hand};

pub fn determine_how_many_hands_better(hand: &[Card]) {
    let users_hand = determine_hand(hand).unwrap();
    let idx = users_hand.rank();
    let hands_categories_better_than_users = 
        &Hand::get_hands()[idx..10];
    println!("{:?}", hands_categories_better_than_users);
}
