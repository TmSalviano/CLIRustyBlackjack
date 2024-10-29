use std::io::stdin;

use blackjack::cards::Deck;

//Everything that you are not going to use here should be private. That is true not only
// in the KING Binary file but also to the rest of the hierarchy.


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
            input.clear();
            println!("Do you want to quit?");
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    // Trim the input to remove any trailing newline characters
                    let input = input.trim().to_lowercase();
                    
                    if input == "yes" {
                        println!("Thank you for playing! I hope you enjoyed yourself!");
                        return;
                    }
                     else if input == "no" {
                        println!("The next round begins!");
                        break;
                    } else {
                        println!("-> yes or no?");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Failed to read line.");
                    continue;
                    // Handle the error appropriately
                }
            }
        }

    }
}
    



