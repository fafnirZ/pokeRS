use pokers_core::cards::Card;


fn main() {
    let all_cards = Card::generate_all_cards();
    for card in all_cards.iter() {
        println!(
            "Card(suit={:?}, number={:?})",
            card.suit,
            card.number,
        )
    }
}
