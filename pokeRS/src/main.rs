use pokers_core::cards::Card;
use pokers_core::hands as Hands;


fn main() {
    let all_cards = Card::generate_all_cards();
    for card in all_cards.iter() {
        println!(
            "Card(suit={:?}, number={:?})",
            card.suit,
            card.number,
        )
    }

    let cards = Vec::new();
    let _res = match Hands::determine_hand(cards) {
        Ok(hand) => hand,
        Err(err) => {
            println!("Error: {:?}", err);
            return
        },
    };
}
