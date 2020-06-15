mod game;

fn main() {
    println!("Hello, world!");
    let mut deck = game::populate_deck();
    println!("Deck size {}", deck.len());
    game::shuffle_deck(&mut deck);

    let mut player1 = game::Player::new("Bobby".to_string(), &mut deck);
    let player2 = game::Player::new("Jess".to_string(), &mut deck);
    let mut dealer = game::Player::new("Dealer".to_string(), &mut deck);

    println!("{}", player1);
    println!("{}", player2);
    println!("{}", dealer);

    dealer.check_bust();
    player1.hit(&mut deck);
    player1.hold();
    println!("{}", player1);
}


