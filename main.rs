use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Hello, world!");
    let mut deck = populate_deck();
    println!("Deck size {}", deck.len());
    shuffle_deck(&mut deck);

    let player1 = Player {name: "Bobby".to_string(), hand: vec![deck.pop().unwrap(), deck.pop().unwrap()]};
    let player2 = Player {name: "Jess".to_string(), hand: vec![deck.pop().unwrap(), deck.pop().unwrap()]};
    let dealer = Player {name: "Dealer".to_string(), hand: vec![deck.pop().unwrap(), deck.pop().unwrap()]};

    println!("{}", player1);
    println!("{}", player2);
    println!("{}", dealer);

    dealer.check_bust();
}


#[derive(Debug)]
struct Card {
    suit: String,
    value: i32, //the "game" value
    index: i32, //the index of the card, 1=ace, 13 = king
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let named_card: String;
        match self.index {
            1 => named_card = "Ace".to_string(),
            11 => named_card = "Jack".to_string(),
            12 => named_card = "Queen".to_string(),
            13 => named_card = "King".to_string(),
            _ => named_card = self.index.to_string(),
        }
        write!(f, "{} of {}", named_card, self.suit)
    }
}

struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    fn sum_hand(&self) -> i32 {
        let mut sum = 0;
        for card in self.hand.iter() {
            sum = sum + card.value;
        }

        return sum;
    }

    fn check_bust(&self) -> bool {
        if self.sum_hand() > 21 {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player: {}\nHand: ", self.name).unwrap();
        for card in self.hand.iter() {
            write!(f, "{}, ", card).unwrap();
        }
        write!(f, "{}", "")
    }
}

fn populate_deck() -> Vec<Card> {
    let suits = vec!["Diamonds", "Hearts", "Spades", "Clubs"];
    let mut deck = Vec::new();
    for suit in suits.iter() {
        for index in 1..14 {
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

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}

