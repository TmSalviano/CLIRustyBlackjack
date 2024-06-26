use super::*;
use std::collections::LinkedList;

pub struct Hand
{
    // You are going to remove the first card in case of a Split
    cards: LinkedList<Card>,
    is_blackjack: bool,
    is_soft_hand: bool,
    is_bust: bool,
    is_first_and_second_card_equal: bool,
    
}

impl Hand {
    pub fn is_blackjack(&self) -> bool {
        self.is_blackjack
    }

    pub fn is_soft_hand(&self) -> bool {
        self.is_soft_hand
    }

    //For accurate results, can only be called after the cards list stop changing
    pub fn is_bust(&self) -> bool {
        self.is_bust
    }

    pub fn is_first_and_second_card_equal(&self) -> bool {
        self.is_first_and_second_card_equal
    }

    

    //The player and Dealer all start with 2 cards every round so the implementation of the parameters is adequate
    pub fn new(card1: Card, card2: Card) -> Hand {
        let first_and_second = card1 == card2;

        Hand {
            is_blackjack: {
                match (&card1, &card2) {
                    (Card::Ace, Card::Ten | Card::Jack | Card::Queen | Card::King) |
                    (Card::Ten | Card::Jack | Card::Queen | Card::King, Card::Ace) => true,
                    (_, _) => false,
                }
            },
            is_first_and_second_card_equal: first_and_second,
            
            //impossible to not be false when the initial Hand is created
            is_soft_hand: false,
            is_bust: false,
            
            cards: LinkedList::from([card1, card2]),
        }
    }

// A very rare case where not separating concerns is actually okay and improves performance.
    pub fn get_total_value(&mut self) -> i32 {
        let mut total_value = 0;

        
        loop {
            for card in &self.cards {
                match card {
                    Card::Ace => {
                        if self.is_soft_hand {
                            total_value +=1
                        }
                        else {
                            total_value += 11
                        }
                    },
                    Card::Two => total_value += 2,
                    Card::Three => total_value += 3,
                    Card::Four => total_value += 4,
                    Card::Five => total_value += 5,
                    Card::Six => total_value += 6,
                    Card::Seven => total_value += 7,
                    Card::Eight => total_value += 8,
                    Card::Nine => total_value += 9,
                    Card::Ten | Card::Jack | Card::Queen | Card::King => total_value += 10,
                }
            }
            
            
            //once the hand is soft it cannot ever become a hard hand again.
            if self.is_soft_hand == false {
                //softhand check
                if total_value > 21 && *&self.cards.contains(&Card::Ace) {
                    self.is_soft_hand = true;
                    total_value = 0;
                    continue
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
        
        //is_busted check
        let check_busted_closure = |total_value: i32| -> bool {
            if total_value > 21 {
                true
            }
            else {
                false
            }
        };
        self.is_bust = check_busted_closure(total_value);

        total_value
    }

    pub fn print_cards(&self) {
        for (index, card) in self.cards.iter().enumerate() {
            if index == self.cards.len() - 1 {
                print!("{}.\n", card.to_string());
                return
            }

            print!("{}, ", card.to_string());
        }
   }

   pub fn add_card(&mut self, card: Card) {
        self.cards.push_back(card);
   }

   pub fn clear(&mut self) {
        self.cards.clear()
   }
}



#[cfg(test)]
mod hand_test {
    use super::*;

    use rand::{rngs::ThreadRng, Rng};

    //WORKS
    #[ignore]
    #[test]
    fn total_value_test() {
        let keys = ["Ace", "Two", "Three", "Four", "Five", "Six",
         "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut rng = rand::thread_rng();

        for iter in 0..=100 {
            let mut deck = Deck::new();

            let card1_index = rng.gen_range(0..keys.len());
            let card2_index = rng.gen_range(0..keys.len());

            let card1 = deck.get_card(String::from(keys[card1_index])).unwrap();
            let card2 = deck.get_card(String::from(keys[card2_index])).unwrap();
            
            let mut hand = Hand::new(card1.clone(), card2.clone());
            
            match (card1, card2) {
                (Card::Ace, Card::Ten | Card::Jack | Card::Queen | Card::King) |
                (Card::Ten | Card::Jack | Card::Queen | Card::King, Card::Ace) => assert_eq!(hand.is_blackjack, true),
                (_, _) => (),
            }

            assert_eq!(hand.is_bust, false);
            assert_eq!(hand.is_soft_hand, false);

            println!("Iteration {}, TotalValue: {}.", iter, hand.get_total_value());
        }
        
    }
    #[ignore]
    #[test]
    fn boolean_fields_test() {
            let mut deck = Deck::new();

            let keys = ["Ace", "Two", "Three", "Four", "Five", "Six",
            "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];


            let mut rng = rand::thread_rng();

            //since I am initializing hand with an Ace the hand has to become soft eventually
            let mut hand = Hand::new(Card::Ace, Card::Two);

            iterating_info(&mut hand, &mut deck, &mut rng, keys);

         }

    fn iterating_info(hand:&mut Hand, deck: &mut Deck, rng: &mut ThreadRng, keys: [&str; 13]) {
    while !hand.is_bust || !hand.is_soft_hand {
        let card_index = rng.gen_range(0..=12);

        let key = String::from(keys[card_index]);

        let card_option = deck.get_card(key);
        
        let card = match card_option {
            Some(cardtype) => cardtype,
            None => {
                println!("No more cards of this type in the deck");
                continue
            }
        };

        hand.cards.push_back(card);

        let total_value = hand.get_total_value();

        let busted = hand.is_bust;

        let softed = hand.is_soft_hand;

        println!("New Iteration.TotalValue {} IsBust: {}, IsSoft {}", total_value, busted, softed);

        print!("    ");
        for (index, card) in hand.cards.iter().enumerate() {
            if index == hand.cards.len() - 1 {
                print!("{}.\n\n", card.to_string())
            }
            print!("{}, ", card.to_string());
        }
    }

}

    #[ignore]
    #[test]
    fn print_cards_test() {
        let mut deck = Deck::new();
        
        let card1 = deck.get_card(String::from("Ten")).unwrap();
        let card2 = deck.get_card(String::from("Jack")).unwrap();

        let mut hand = Hand::new(card1, card2);

        hand.cards.push_back(deck.get_card(String::from("Seven")).unwrap());

        hand.print_cards();
    }

}