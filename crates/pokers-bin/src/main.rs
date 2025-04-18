use pokers_core::cards::Card;
use pokers_core::deck::Deck;
use pokers_core::hands as Hands;
use pokers_probability::determine::{determine_how_many_hands_better};

// fn main() {
//     let all_cards = Cards::Card::generate_all_cards();
//     for card in all_cards.iter() {
//         println!("Card(suit={:?}, number={:?})", card.suit, card.number,)
//     }

//     let mut cards = Vec::new();
//     // populating with random cards
//     cards.push(Cards::Card {
//         suit: Cards::Suit::Diamond,
//         number: Cards::CardNumber::Two,
//     });
//     cards.push(Cards::Card {
//         suit: Cards::Suit::Club,
//         number: Cards::CardNumber::Two,
//     });
//     cards.push(Cards::Card {
//         suit: Cards::Suit::Diamond,
//         number: Cards::CardNumber::Three,
//     });
//     cards.push(Cards::Card {
//         suit: Cards::Suit::Diamond,
//         number: Cards::CardNumber::Ace,
//     });
//     cards.push(Cards::Card {
//         suit: Cards::Suit::Diamond,
//         number: Cards::CardNumber::Queen,
//     });

//     // result
//     let _res = match Hands::determine_hand(cards) {
//         Ok(hand) => hand,
//         Err(err) => {
//             println!("Error: {:?}", err);
//             return;
//         }
//     };
//     println!("{:?}",_res);
// }


fn main() {
    println!("Welcome to PokeRS simulator.");
    let mut deck = Deck::init_shuffled();

    // user
    let user_cards = vec![
        deck.cards.pop().unwrap(),
        deck.cards.pop().unwrap(),
    ];
    println!("User's hand:");
    for uc in user_cards.iter() {
        println!("\t{}", uc.format_print_short())
    } 

    // table
    let table_cards = vec![
        deck.cards.pop().unwrap(),
        deck.cards.pop().unwrap(),
        deck.cards.pop().unwrap(),
    ];
    println!("Table's hand:");
    for tc in table_cards.iter() {
        println!("\t{}", tc.format_print_short())
    } 


    // user's best hand
    println!("User's best hand:");
    let _user_table_combined = {
        let mut _res = user_cards;
        _res.extend(table_cards);
        if _res.len() != 5 {
            panic!("Invalid hand expected len 5");
        }
        _res
    };
    let user_hand = Hands::determine_hand( &_user_table_combined)
                                    .unwrap();
    println!("\t{:?}", user_hand);

    // user hand sorted
    println!("User's hand sorted:");
    print!("\t");
    for c in Hands::sort_cards(&_user_table_combined) {
        print!("{}", c.format_print_short());
        print!(",");
    }
    println!("");
    

    // hands better than user's hand
    determine_how_many_hands_better(&_user_table_combined);

}