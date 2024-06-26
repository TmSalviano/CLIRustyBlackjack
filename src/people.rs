// Here you will define everythin that applies for people in general
// FORGOR ABOUT THE GETTER TO THE FIELDS THAT SHOULD BE PRIVATE


use crate::cards::Deck;
use crate::cards::{hand::Hand, Card};
use crate::Difficulty;

pub struct Player {
    // access to the hand struct but no acces to it's fields so it's okay.
    pub hand: Hand,
    name: String,
    funds: i32,
    is_broke: bool,
}
impl Player {
    pub fn new(card1: Card, card2: Card, difficulty: Difficulty, name: String) -> Player {
        Player {
            //these values will most likely need to be changed
            funds: match difficulty {
                Difficulty::Easy => 1000,
                Difficulty::Medium => 300,
                Difficulty::Hard => 50,
            },

            hand: Hand::new(card1, card2),
            name,
            is_broke: false
        }
    }

    pub fn bet(&mut self) -> i32 {
        let developer_question: &str = "Dealer: How much you wanna bet?";

        loop {
            // The flawed get_player_input function
            println!("{}", developer_question);

            use std::io::{stdin,stdout,Write};
           
            let mut input: String =String::new();
            print!("Enter bet amount: ");
            let _=stdout().flush().expect("Failed to flush stdout");
            
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    // Successfully read input
                    if let Some('\n') = input.chars().next_back() {
                        input.pop();
                    }
                    if let Some('\r') = input.chars().next_back() {
                        input.pop();
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                }
            }

            

            let round_bet: i32 = match input.parse::<i32>() {
                Ok(value) => value,
                Err(_) => i32::MAX
            };

            if round_bet == 0 {
                return 0
            }

            if self.try_get_money(round_bet) {
                return round_bet;
            } else {
                continue;
            }
        }
    }

    //this is for the BetSytem For Double and Insurance
    pub fn try_get_money(&mut self, money_amount: i32) -> bool {
        if self.funds >= money_amount {
            self.funds -= money_amount;
            println!("{}$ added to the pot!", money_amount);
            return true
        }
        else {
            println!("Insufficient funds. Funds: {}$!", self.funds);
            return false;
        }
    }

    pub fn hit_or_stand(&mut self, deck: &mut Deck) -> HitStand {
        let mut hit_stand: HitStand = HitStand::Hit;

        while hit_stand == HitStand::Hit &&  self.hand.get_total_value() <= 21 {

            // The flawed get_player_input function
            println!("{}", "Do you want to Hit or Stand?");

            use std::io::{stdin,stdout,Write};
            let mut input: String = String::new();
            print!("Please enter some text: ");
            let _=stdout().flush();
            stdin().read_line(&mut input).expect("Did not enter a correct string");
            if let Some('\n')=input.chars().next_back() {
                input.pop();
            }
            if let Some('\r')=input.chars().next_back() {
                input.pop();
            }

            match input.as_str() {
                "stand" => hit_stand = HitStand::Stand,
                "hit" => {
                    let mut card = None;
                    while card == None  {
                        let name = deck.get_random_card_name();
                        card = deck.get_card(name);
                    }

                    self.hand.add_card(card.unwrap());
                    self.hand.print_cards();
                },
                _ => println!("Type: hit or stand."),
            }
        }


        return hit_stand
    }

    //called at the beggining of every game.
    pub fn is_broke(&mut self) -> bool {
        if self.funds <= 0 {
            self.is_broke = true;
        }

        self.is_broke
    }

    pub fn add_to_funds(&mut self, reward: i32) {
       self.funds += reward
    }

    pub fn funds(&self) -> i32 {
        self.funds
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    
}


//I know what you are thinking and no, it is not worth it to create a "Person" trait for Dealer and Player. Smells like boilerplate.
pub struct Dealer {
    up_card: Card,
    pub hand: Hand,
}
impl Dealer {
    pub fn new(card1: Card, card2: Card) -> Dealer {
        Dealer {
            //the up_card being a reference to card1 just felt wrong/bad. But I am not sure if it's bad anymore.
            up_card: card1.clone(),
            hand: Hand::new(card1, card2),
        }

    }

    pub fn hit_or_stand(&mut self, deck: &mut Deck) {
        while self.hand.get_total_value() <= 21 {
            if self.hand.get_total_value() >= 17 {
                println!("Dealer: I Stand!");
                return
            } else {
                let mut card = None;
                println!("Dealer: I Hit!");
                while card == None  {
                    let name = deck.get_random_card_name();
                    card = deck.get_card(name);
                }

                self.hand.add_card(card.unwrap());
                self.hand.print_cards();
            }
        }
    }

    pub fn up_card(&self) -> &Card {
        &self.up_card
    }
}

#[derive(PartialEq)]
pub enum HitStand {
    Hit,
    Stand,
}


#[cfg(test)]
mod people_test {
    use crate::cards::Deck;
    use super::*;

    //Player tests
    //Done
    #[ignore]
    #[test]
    fn is_broke_test() {
        let mut deck = Deck::new();

        let (card1, card2) = (deck.get_card(String::from("Eight")).unwrap(), deck.get_card(String::from("Nine")).unwrap());

        let mut player = Player::new(card1, card2, Difficulty::Easy, String::from("Tiago"));

        player.bet();

        assert_eq!(player.is_broke(), true);
    }


    //Dealer Tests
    //Done
    #[ignore]
    #[test]
    fn ultimate_dealer_test() {
        let mut deck = Deck::new();
        let mut dealer = Dealer::new(Card::Two, Card::Two);

        let hit_or_stand = HitStand::Hit;

        while hit_or_stand == HitStand::Hit {
            print!("Cards: ");
            dealer.hand.print_cards();

            println!("The up card is {}.", dealer.up_card().to_string());

            print!("Total Value: {}", dealer.hand.get_total_value());

            dealer.hand.add_card(Card::Ten);    
            
            dealer.hit_or_stand(&mut deck);
        }

        println!("Successfully generated a STAND from the dealer");
    }
}
