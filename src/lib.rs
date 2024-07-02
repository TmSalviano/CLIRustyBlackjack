pub mod people;
pub mod cards;
use std::{io::{stdin,stdout,Write}, thread::sleep, time::Duration};
use cards::{Card, Deck};
use people::*;

// The game flow and the bet system will be defined here
// maybe you should create a randomizer struct: fields -> thread: rng::thread_rng(), card_key_array: [Card],
// method -> get_random_card()


pub struct Game  {
    reward_multiplier: f32,
    bet_amount: i32,
    dealer: Dealer,
    player: Player,
    //To increase game performance we will check if the game is still going (None)
    //or if the game already ended Some(value)
}

impl Game {
    //constructor
    pub fn new(deck: &mut Deck) -> Game {

        let (p, d) = Game::player_dealer_creator(deck);

        Game {
            reward_multiplier: 1.0,
            bet_amount: 0,
            player: p,
            dealer: d,
        }
    }
    fn player_dealer_creator(deck: &mut Deck) -> (Player, Dealer) {
        let mut card_vec = Game::get_initial_cards(deck);
    
        let difficulty = select_difficulty();

        let player_name = select_name();

        sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H");

        let player = people::Player::new(card_vec.pop().unwrap(), card_vec.pop().unwrap(), difficulty, player_name);
        let dealer = people::Dealer::new(card_vec.pop().unwrap(), card_vec.pop().unwrap());
    
        return (player, dealer)
    }

    fn get_initial_cards(deck: &mut Deck) -> Vec<Card> {
        let mut card_vec: Vec<Card> = Vec::new();
        while card_vec.len() < 4 {
            let name = deck.get_random_card_name();
            let card = deck.get_card(name);
            let card = match card {
                Some(value) => value,
                None => continue
            };
            card_vec.push(card);
        }

        card_vec
    }

    //This only contains the re-initialization that pertains to the Game fields. You still need to make the returned deck the one that will be used in the next round
    pub fn re_initialize(&mut self,) -> Deck {

        let mut deck = Deck::new();

        let mut card_vec = Game::get_initial_cards(&mut deck);

        self.player.hand.clear();
        self.player.hand.add_card(card_vec.pop().unwrap());
        self.player.hand.add_card(card_vec.pop().unwrap());
        
        self.dealer.hand.clear();
        let upcard = card_vec.pop().unwrap();
        self.dealer.hand.add_card(upcard.clone());
        self.dealer.set_upcard(upcard);
        self.dealer.hand.add_card(card_vec.pop().unwrap());
        
        deck
    }   

    //getters
    pub fn reward_multiplier(&self) -> f32 {
        self.reward_multiplier
    }
    pub fn bet_amount(&self) -> i32 {
        self.bet_amount
    }
    
    pub fn is_player_broke(&mut self) -> bool {
        self.player.is_broke()
    }

//Every round will exectued and evaluated by this method
    pub fn round_manager(&mut self, deck: &mut Deck) {
        match self.round(deck) {
            Some(value) => {
                if value == WinLose::Win {
                    let reward = self.bet_amount as f32 * self.reward_multiplier;
                    self.player.add_to_funds(reward as i32 + self.bet_amount);
                    println!("Congratulations {}. You won: {}$", self.player.name(), reward)
                } else {
                    //The money is already subtracted from the funds as the game progresses.
                    println!("You've lost {}$. Your funds are now: {}$", self.bet_amount(), self.player.funds())
                }
            },
            None => {
                self.player.add_to_funds(self.bet_amount);
                println!("It's a draw! Returning the money to your funds...");
                println!("Funds: {}", self.player.funds());
            }
        }
        sleep(Duration::from_secs(2));
        print!("\x1B[2J\x1B[1;1H");
    }

    // In this method, the return value None is equivalent to a draw and not a negation of the proposition like in the bet methods.
    fn round(&mut self, deck: &mut Deck) -> Option<WinLose> {
        //We check if the player is broke before the round starts not during the round. So the edge case should not be handled here
        self.ante_bet();

        println!("{}:", self.player.name());
        self.player.hand.print_cards();
        
        println!("Dealer:",);
        print!("    Cards -> ");
        println!("{}, hidden.", self.dealer.up_card().to_string());
        

        println!("\n{}'s turn!", self.player.name());
        sleep(Duration::from_secs(3));
        print!("\x1B[2J\x1B[1;1H");

        if *self.dealer.up_card() == Card::Ace {
            match self.insurance() {
                Some(value) => return Some(value),
                None => {},
            }
        }
        match self.double(deck) {
            Some(value) => return Some(value),
            None => {},
        }
        
        self.player.hit_or_stand(deck);
        if self.player.hand.is_bust() {
                return Some(WinLose::Lose)
        }

        println!("\nDealer's turn!");
        sleep(Duration::from_secs(2));
        print!("\x1B[2J\x1B[1;1H");

        println!("Dealer:");
        self.dealer.hand.print_cards();
        self.dealer.hit_or_stand(deck, self.player.hand.get_total_value());
        if self.dealer.hand.is_bust() {
            return Some(WinLose::Win)
        }
        
        Game::player_dealer_value_comparison(self.player.hand.get_total_value(), self.dealer.hand.get_total_value())
    }
    
    //actions
    //implementation of the split should be done in the Game since it is a very special ocurrance that does not fit BetSytem
    fn ante_bet(&mut self) {
        let amount = self.player.bet();
        self.bet_amount = amount;

        if self.player.hand.is_blackjack() {
            println!("Hey... you have a blackjack?! You will now get paid 3:2 if you win!");
            sleep(Duration::from_secs(1));
            print!("\x1B[2J\x1B[1;1H");
            self.reward_multiplier *= 1.5;
        }
    }

    //called if player accepted the doubling of the bet
    fn double(&mut self, deck: &mut Deck) -> Option<WinLose>{

        println!("{}:", self.player.name());
        self.player.hand.print_cards();
        println!("Dealer: Do you want to double the pot? \nFunds: {}$ \n-> Yes\n-> No", self.player.funds());

        let mut input: String = String::new(); 
        while input.to_lowercase() != "yes" && input.to_lowercase() != "no" {
        input.clear();

            let _=stdout().flush();
            stdin().read_line(&mut input).expect("Did not enter a correct string");
            if let Some('\n')=input.chars().next_back() {
                input.pop();
            }
            if let Some('\r')=input.chars().next_back() {
                input.pop();
            }
        }    

        if input.to_lowercase() == "yes" {
            println!("Doubling the bet_amount...");
            if self.player.try_get_money(self.bet_amount) {
                self.bet_amount *= 2;

                let mut card = None;
                while card == None  {
                    let name = deck.get_random_card_name();
                    card = deck.get_card(name);
                }

                self.player.hand.add_card(card.unwrap());

                println!("A card was added for accepting the double offer!");

                println!("{}:", self.player.name());
                self.player.hand.print_cards();


                if self.player.hand.get_total_value() > 21 {
                    println!("Busted for accepting the double bet");
                    return Some(WinLose::Lose)
                }

                sleep(Duration::from_secs(1));
                print!("\x1B[2J\x1B[1;1H");
                return None
            }
            else {
                println!("Not enough funds for the double bet offer.");
                sleep(Duration::from_secs(1));
                print!("\x1B[2J\x1B[1;1H");
            }
        } 
        
        println!("Dealer: HAHAHAHA! Your Loss!");
        sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H");
        return None
    }


    fn insurance(&mut self) -> Option<WinLose> {
            // The flawed get_player_input function
            println!("{}", "My Ace is showing... wanna bet that I have a blackjack? Well, you have to put half of your pot on the table then.");

            let mut input: String = String::new(); 

            while input.to_lowercase() != "yes" && input.to_lowercase() != "no" {
            input.clear();

            println!("yes or no?");
    
                print!("Please enter some text: ");
                let _=stdout().flush();
                stdin().read_line(&mut input).expect("Did not enter a correct string");
                if let Some('\n')=input.chars().next_back() {
                    input.pop();
                }
                if let Some('\r')=input.chars().next_back() {
                    input.pop();
                }
            }    

        //the insurance bet can end the game or not. it depends if the player accepts it or not.
        if input.to_lowercase() == "yes" {
            let insurance_bet_price = self.bet_amount / 2;

            if self.player.try_get_money(insurance_bet_price )
            {
                self.bet_amount += insurance_bet_price;

                if self.dealer.hand.is_blackjack() {
                    self.reward_multiplier *= 2.0;
                    println!("He had a blackjack!");
                    return Some(WinLose::Win)
                } else {
                    println!("He did not have a blackjack!");
                    return Some(WinLose::Lose)
                }
            }
        }

        None
    }


    fn player_dealer_value_comparison(player_total: i32, dealer_total: i32) -> Option<WinLose> {
        if player_total > dealer_total {
            Some(WinLose::Win)
        } else if player_total < dealer_total {
            Some(WinLose::Lose)
        } else {
            None
        }
    }
}


pub fn select_difficulty() -> Difficulty {

    loop {
        let mut input = String::new();
        println!("Select Difficulty: \n    -> Easy \n    -> Medium\n    -> Hard");

        fn message(money: i32) {
            println!("Your Buy-in today is {}$", money);
            sleep(Duration::from_secs(1));
            print!("\x1B[2J\x1B[1;1H");
        }
        
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();
                match input.as_str() {
                    "easy" => {
                        message(1000);
                        return Difficulty::Easy;
                    },
                    "medium" => {
                        message(300);
                        return Difficulty::Medium;
                    },
                    "hard" => {
                        message(50);
                        return Difficulty::Hard;
                    },
                    _ => {
                        println!("Invalid input, please try again.");
                    }
                }
            },
            Err(_) => {
                println!("Failed to read line, please try again.");
                continue;
            }
        }
    }
}


pub fn select_name() -> String {
    loop {
        let mut input = String::new();
        println!("Type your name below:");
        let input: String = match stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(_) => {
                continue
            }
        };

        println!("Okay {}, I hope you make a lot of money!", input);
        sleep(Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H");

        return input
    }
}

#[derive(PartialEq, Clone, Copy)]
enum WinLose {
    Win,
    Lose
}
pub enum Difficulty {
    Easy,
    Medium,
    Hard
 }

 #[allow(dead_code)]
#[cfg(test)]
mod lib_test {
    use cards::*;
    use rand::Rng;
    use super::*;
    //still need to test all fields and method of the BetSytem

    fn get_cards() -> Vec<Card> {
        let mut rng_thread = rand::thread_rng();
        let mut deck = Deck::new();

        let card_keys = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut cards: Vec<Card> = Vec::new();

        for _iter in 0..4 {
           let card = loop {
                let random_num = rng_thread.gen_range(0..13);
                let random_key = card_keys[random_num];
    
                match deck.get_card(random_key.to_string()) {
                    Some(value) => break value,
                    None => {
                        continue;
                    }
                };
            };
            cards.push(card);
        }

        assert_eq!(cards.len(), 4);

        cards

    }

    fn print_reward_and_bet(reward_multiplier: &f32, bet_amount: &i32) {
        println!("reward_multiplier: {}, bet_amount: {}$.", reward_multiplier, bet_amount)
    }


    // Do not forget to make it so the change the test to test when the player and the dealer
    //have a blackjack to see if everything works.
    #[ignore]
    #[test]
    fn bet_system_test() {
        //Preparation
        let mut deck = Deck::new();

        let mut bet_system = Game::new(&mut deck);

        print_reward_and_bet(&bet_system.reward_multiplier(), &bet_system.bet_amount());
        
        bet_system.ante_bet();
        print_reward_and_bet(&bet_system.reward_multiplier(), &bet_system.bet_amount());
        
        bet_system.double(&mut deck);
        print_reward_and_bet(&bet_system.reward_multiplier(), &bet_system.bet_amount());
        
        bet_system.insurance();
        print_reward_and_bet(&bet_system.reward_multiplier(), &bet_system.bet_amount());


    }

}
