mod thingies;

fn main() {
    println!("Hello, world!");
    let mut deck = thingies::populate_deck();
    println!("Deck size {}", deck.len());
    thingies::shuffle_deck(&mut deck);

    let mut player1 = thingies::Player::new("Bobby".to_string(), &mut deck);
    let player2 = thingies::Player::new("Jess".to_string(), &mut deck);
    let mut dealer = thingies::Player::new("Dealer".to_string(), &mut deck);

    println!("{}", player1);
    println!("{}", player2);
    println!("{}", dealer);

    dealer.check_bust();
    player1.hit(&mut deck);
    player1.hold();
    println!("{}", player1);
}


