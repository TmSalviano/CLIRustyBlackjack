Here are all the things that you will need to implement to make your blackjack application work.

-- Done --
Deck
    1. Deck of 52 cardds
    2. 10, Jack, Queen and King all have value 10;
    4. The A card can have value 11 or 1.
    3. 13 differen types of cards repeat 4 times for a total of 52

-- DONE --
Cards: Enum

-- Test Done But May Requires Testing Again --
Game struct
    fields:
        reward_multiplier
        bet_amount    
        won_or_lost: Options<WinorLose> to check if the game already ended or not.
    Actions
        //the methods that change the bet amount should have a &mut Player argument
        fn ante()
        fn insurance() -> returns a Option<WinOrLose> since it can potentially end the game.
        fn double()
        fn split()
Bets //enum with nested enum is overkill
    Reward Changing Bets
        Ante Bet - bet placed to begin Game. Done before any cards are dealt.
            1. Reward
                1. 3:2 if blackjack reward_multiplier * 1.5
                2. 1:1 everything else reward_multiplier * 1

        Double - Player doubles his bet after cards are dealt or insurance and grabs one more card. reward_multiplier * 2


        Insurance Bet
            If accepted:
                your bet amount increases 50%.
                You win if Dealer has a blackjack with reward multiplier * 2. If he does not you lose the game.
    

    // Smart to make a working game before implement splits to the Game Struct
    Game changing Bets
        Split - Bet done right after cards are dealt or after EvenMoney and/or Insurance offer if the player has cards of equal value
            1.The Spliting can only happen once per Game
    

Game Actions
    -- DONE --
    Busted - total value became bigger than 21. You lose Game!

    Push - nobody wins. Bets are given back to the player


-- Done --
Enum Difficulty 
    Variants
        1. Easy
        2. Medium
        3. Hard


-- Done --
Player
    Actions
        1. fn bet()
        2. fn hit_or_stand()

    Characteristcs
        1. Hand - Cards of the player
        2. Funds

    Special
        1. Needs a Difficulty variant to define the amount of funds
        2. Both cards are hidden but since you are the player you will be able to see both.
    

-- DONE  --
Dealer
    Actions
        1. Conditional Dealer Hit :  when the dealer’s hand is 16 or less, they will draw cards (or “hit”), 
        2. Conditional Dealer Stand :  when it is 17 or more, they will not draw additional cards (or “stand pat”).

        1&2. fn dealer_hit_or_stand() {}

    Characteristcs
        1. Hand - Cards of the dealer
        2. Up Card - card showing -> Hand.Card1. The card that is not hidden.
        


-- DONE --
Hand
    Fields
        1. Card1 - first "beggining card"
        2. Card2 - second "beggining card"
        3. Cards[Card1, Card2, ...]: vec
        4. TotalValue
    States
        1. Soft Hand - when it has an ace that is worth 1
        2. Hard hand - when the hand has an ace that is worth 11
    Characteristcs
        3. IsBlackJack - Card1 and Card2 are a combination of an Ace and a value 10 card
        4. IsBust - is the value of the cards bigger than 21?
        5. IsSoftHand - true if the hand has at least one ace and the total card value is above 21.
        6. GetTotalCardValue - return the value of the cards in your Deck
        



Game
    The point of the game is to reach as close as possible to the value 21 without busting or having less than the House

    The player and the house receive 2 cards

    The house has one card exposed (the up card) and one card concealed (the hole card)

    The Player and the Dealer receive their cards in pairs.
Game Flow

    1. AnteBet
    2. Cards are dealt to both
    match (PlayerHand, DealerUpcard) {
        (Blackjack, Ace) => {
            Even Money Offer; 
            Insurance
        }
        (_, Ace) => Insurance
        (_, _) => No offers are made and game continues
    }
    4.Depending on the player's cards, Here the Player have the ability to do a Double or Split which changes the game entirely
    next item Assumes that player doesnt have the ability or did not engage with the special bets
    5. Player will repeatedly chooose to Hit or Stand until he Busts or 
    7.
    if (Player Busts)
    {
        Players loses Game
    }
    else
    {
        6.Dealer Reveals Hidden card 
        7.Dealer will Hit or Stand depending on if the value is > than 16 or not
        If (Dealer Busts)
        {
            Player Win
        }
        else
        {
            Wins who has the most value and if the values are equal a Push happens
        }
    }

    Adjust the money of the player based on the results

    Re-initialize the Deck.

        Double SituatioN
            The players will double his bet and draw one more card which will maybe make him bust

        Split situation
            1.The Player will now play 2 games simultaenously with the Dealer
            2.The cards are split
            3.You need to make 1 new ante bet for the new parallel game
            4. The dealer deals the 1 new card for the Playr in each of the 2 games
