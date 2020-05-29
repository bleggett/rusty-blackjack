use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Hello, world!");
    let mut deck = populate_deck();
    println!("Deck size {}", deck.len());
    shuffle_deck(&mut deck);

    let mut player1 = Player::new("Bobby".to_string(), &mut deck);
    let player2 = Player::new("Jess".to_string(), &mut deck);
    let mut dealer = Player::new("Dealer".to_string(), &mut deck);

    println!("{}", player1);
    println!("{}", player2);
    println!("{}", dealer);

    dealer.check_bust();
    player1.hit(&mut deck);
    player1.hold();
    println!("{}", player1);
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
    playing: bool,
    holding: bool,
}

impl Player {

    fn new(name: String, deck: &mut Vec<Card>) -> Player {
        Player {name, hand: vec![deck.pop().unwrap(), deck.pop().unwrap()], playing: true, holding: false}
    }

    fn sum_hand(&self) -> i32 {
        let mut sum = 0;
        for card in self.hand.iter() {
            sum = sum + card.value;
        }

        return sum;
    }

    fn hit(&mut self, deck: &mut Vec<Card>) {
        self.hand.push(deck.pop().unwrap());
        self.check_bust();
    }

    fn hold(&mut self) {
        self.holding = true;
    }

    fn check_bust(&mut self) -> bool {
        if self.sum_hand() > 21 {
            self.playing = false;
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

