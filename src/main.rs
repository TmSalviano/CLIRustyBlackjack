use std::io::stdin;

use blackjack::cards::Deck;

//blackjack{select_difficulty, select_name};


//Everything that you are not going to use here should be private. That is true not only
// in the KING Binary file but also to the rest of the hierarchy.

//Dont forget taht after you finish logic implementation the program is going to required a lot of polishing

fn main() {
    let mut deck = Deck::new();
    let mut game = blackjack::Game::new(&mut deck);

    loop {
        if game.is_player_broke() {
            println!("You went broke!");
            // Add a special way to finish the game because the player really lost
            break;
        }
        
        let mut deck = game.re_initialize();
        game.round_manager(&mut deck);
        
        let mut input = String::new();
        loop {
            println!("Do you want to quit?");
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    // Trim the input to remove any trailing newline characters
                    let input = input.trim().to_lowercase();
                    
                    if input == "yes" {
                        println!("I hope you enjoyed yourself! Your progress will be saved.");
                        return;
                    }
                    if input == "no" {
                        println!("The next round begins!");
                        break;
                    }
                },
                Err(_) => {
                    println!("Failed to read line.");
                    // Handle the error appropriately
                }
            };
            println!("-> yes or no?")
        }

    }
}
    



