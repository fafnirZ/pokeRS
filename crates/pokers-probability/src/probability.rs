// // use pokers_core::hand
// use pokers_core::cards::Card;

// pub fn calculate_better_hand_probability(user_hand: &[Card]) -> f64 {

//     return 0.0
// }

// fn factorial(num: i32) -> i32 {
//     if num == 0 {
//         return 1
//     }
//     return num * factorial(num-1);
// }

// // 52C5
// fn probability_of_five_hand_cards() -> i32{
//     factorial(52) / (factorial(5) * factorial(52-5))
// }

// fn probability_of_royal_flush() -> f64 {
//     let num_suit_combinations = 4;
//     let total_number_of_5_hand_cards = probability_of_five_hand_cards();
//     return (num_suit_combinations as f64) / (total_number_of_5_hand_cards as f64);
// }

// // probability of royal flush with sliding window
// // {10..A}
// // {9..K}
// // {8..Q}
// // ...
// // {1..5}
// // straight flush is total number minus probability of royal flush.
// fn probability_of_straight_flush() -> f64 {
//     let total_num_straight_flush_including_royal = 10 * probability_of_royal_flush();
//     return total_num_straight_flush_including_royal - probability_of_royal_flush();
// // }