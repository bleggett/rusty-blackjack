use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Player {
    name: String,
    hand: Vec<Card>,
    playing: bool,
    holding: bool,
}

impl Player {
    pub fn new(name: String, deck: &mut Vec<Card>) -> Player {
        Player {name, hand: vec![deck.pop().unwrap(), deck.pop().unwrap()], playing: true, holding: false}
    }

    pub fn sum_hand(&self) -> i32 {
        let mut sum = 0;
        for card in self.hand.iter() {
            sum = sum + card.value;
        }

        return sum;
    }

    pub fn hit(&mut self, deck: &mut Vec<Card>) {
        self.hand.push(deck.pop().unwrap());
        self.check_bust();
    }

    pub fn hold(&mut self) {
        self.holding = true;
    }

    pub fn check_bust(&mut self) -> bool {
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

#[derive(Debug)]
pub struct Card {
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

pub fn populate_deck() -> Vec<Card> {
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

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_populate_deck() {
        assert_eq!(52, populate_deck().len())
    }



    #[test]
    fn test_shuffle_deck() {
        let mut test_deck = populate_deck();
        shuffle_deck(&mut test_deck);
        assert_eq!(52, test_deck.len())
    }

}
