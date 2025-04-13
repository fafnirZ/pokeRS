use pokers_core::cards as Cards;
use pokers_core::hands as Hands;

fn main() {
    let all_cards = Cards::Card::generate_all_cards();
    for card in all_cards.iter() {
        println!("Card(suit={:?}, number={:?})", card.suit, card.number,)
    }

    let mut cards = Vec::new();
    // populating with random cards
    cards.push(Cards::Card {
        suit: Cards::Suit::Diamond,
        number: Cards::CardNumber::Two,
    });

    // result
    let _res = match Hands::determine_hand(cards) {
        Ok(hand) => hand,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };
}
