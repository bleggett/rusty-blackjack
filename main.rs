use std::io;


fn main() {
    println!("Hello, world!");
    println!("{:?}", populate_deck());
}

#[derive(Debug)]
struct Card {
    suit: String,
    value: i32,
}

struct Player {
    name: String,
    hand: Vec<Card>,
}

fn populate_deck() -> Vec<Card> {
    let suits = vec!["Diamonds", "Hearts", "Spades", "Clubs"];
    let mut deck = Vec::new();
    for suit in suits.iter() {
        for value in 1..13 {
            deck.push(Card {suit: suit.to_string(), value});
        }
    }

    return deck;
}
