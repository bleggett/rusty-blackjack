use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Hello, world!");
    let mut deck = populate_deck();
    println!("{:?}", shuffle_deck(&mut deck));

    let player1 = Player {name: "Bobby".to_string(), hand: vec![deck.pop().unwrap(), deck.pop().unwrap()]};
    let player2 = Player {name: "Jess".to_string(), hand: vec![deck.pop().unwrap(), deck.pop().unwrap()]};
}


#[derive(Debug)]
struct Card {
    suit: String,
    value: i32, //the "game" value
    index: i32, //the index of the card, 1=ace, 13 = king
}

struct Player {
    name: String,
    hand: Vec<Card>,
}

fn populate_deck() -> Vec<Card> {
    let suits = vec!["Diamonds", "Hearts", "Spades", "Clubs"];
    let mut deck = Vec::new();
    for suit in suits.iter() {
        for index in 1..13 {
            let mut value = index;
            //Cards over 10 have their game value capped at 10
            if index > 10 {
                value = 10
            }
            deck.push(Card {suit: suit.to_string(), value, index});
        }
    }

    return deck;
}

fn shuffle_deck(deck: &mut Vec<Card>) -> &Vec<Card> {
    let mut rng = thread_rng();

    deck.shuffle(&mut rng);

    return deck;
}
