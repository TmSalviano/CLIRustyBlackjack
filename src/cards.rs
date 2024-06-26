use std::collections::HashMap;

use rand::Rng;

pub mod hand;
// Here you will define anything that relates to hand and card implemenation
#[allow(dead_code)]

pub struct Deck {
    deck_array: [Suit; 4]
}
impl Deck {
    pub fn new() -> Self {
        let hearts =  Suit::new(String::from("hearts"));
        let spades =  Suit::new(String::from("spades"));
        let diamonds =  Suit::new(String::from("diamonds"));
        let clubs =  Suit::new(String::from("clubs"));

        let deck_array: [Suit; 4] = [hearts, spades, diamonds, clubs];
        Self {
            deck_array
        }
    }

    //This ias more of a test function. The real one should be random.
    pub fn get_card(&mut self, card_name_first_letter_uppercase: String) -> Option<Card> {

        'outer: for suit in &mut self.deck_array {
           for (key, value) in &mut suit.cards {
                if *key != card_name_first_letter_uppercase {
                    continue;
                }
                if  *key ==  card_name_first_letter_uppercase && value.is_some() {
                    return value.take()
                }
                else {
                    continue 'outer
                }
           }
        }

        // if this function returns None, then there are no more cards with that card_name
        return None

    }

    //This is for calling get_card() with random returns
    pub fn get_random_card_name(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let rand_card = Card::try_from(rng.gen_range(0..=12)).unwrap();
        return rand_card.to_string()
    }

}


struct Suit {
    #[allow(dead_code)]
     name: String,
     cards: HashMap<String, Option<Card>>
}
impl Suit {
    pub fn new( name: String) -> Self {
        let mut map: HashMap<String, Option<Card>> = HashMap::new();

        //prep to populate
        let cards: Vec<Card> = vec![
            Card::Ace,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King
        ];
        let populate_map_closure = |  map: & mut HashMap<String, Option<Card>>, vec: Vec<Card> | { 
            for card in vec {
                map.insert(card.to_string(), Some(card));
            };
        };
        //end of prep
        populate_map_closure(&mut map, cards);
        
        Self {
                name: name,
                cards: map
            }
    }
}


#[derive(PartialEq, Eq, Clone)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    
}
impl ToString for  Card {
    fn to_string(&self) -> String {
        match self {
            Card::Ace => return String::from("Ace"),
            Card::Two => return String::from("Two"),
            Card::Three => return String::from("Three"),
            Card::Four => return String::from("Four"),
            Card::Five => return String::from("Five"),
            Card::Six => return String::from("Six"),
            Card::Seven => return String::from("Seven"),
            Card::Eight => return String::from("Eight"),
            Card::Nine => return String::from("Nine"),
            Card::Ten => return String::from("Ten"),
            Card::Jack => return String::from("Jack"),
            Card::Queen => return String::from("Queen"),
            Card::King => return String::from("King"),
        };
    }

}
impl TryFrom<i32> for Card {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ace),
            1 => Ok(Self::Two),
            2 => Ok(Self::Three),
            3 => Ok(Self::Four),
            4 => Ok(Self::Five),
            5 => Ok(Self::Six),
            6 => Ok(Self::Seven),
            7 => Ok(Self::Eight),
            8 => Ok(Self::Nine),
            9 | 10 | 11 | 12 => Ok(Self::Ten),
            _ => Err(()),
            
        }
    }
    
    type Error = ();
}

#[cfg(test)]
mod hand_card_tests {
    use super::*;
    
    #[ignore]
    #[test]
    fn deck_retrieve() {
        let mut deck = Deck::new();

        let takeaway_card = deck.get_card(String::from("Jack"));

        println!("Take away card: {}", takeaway_card.unwrap().to_string());

        let mut iterations = 0;
        
        for suit in deck.deck_array {
            for (key, value) in suit.cards {
                iterations += 1;

                if value.is_none() {
                    iterations -= 1;
                    println!("In the Key: {} of the suit {}, the value was none", key, suit.name);
                    continue
                }

                println!("{}: key = {}, value = {}", &suit.name, &key, &value.unwrap().to_string())
            }
        }
      assert_eq!(iterations, 51)
    }

    #[ignore]
    #[test]
    fn no_more_cards_() {

        let mut deck = Deck::new();

        let mut iterations: i32 = 0;
        loop {
            iterations += 1;
            let result: Option<Card> = deck.get_card(String::from("Jack"));

            if result.is_some() {

                println!("result is {} at iteration: {}", result.unwrap().to_string(), iterations)
            }
            else 
            {
                println!("The None value was found at iteration: {}", iterations);
                break;
            }
        }

        assert_eq!(iterations, 5);
    }
}