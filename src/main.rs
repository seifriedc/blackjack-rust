// Blackjack
// by Chris Seifried

extern crate rand;
use rand::Rng;
use std::io;

struct Player {
    hand: i32,
    new_card: i32,
    total: i32,
    num_aces: i32,
    ace_val: i32,
}

// Methods attached to Player struct
impl Player {

    // Need &mut self to make Player mutable
    fn draw_card(&mut self) {
        let deck: [i32;13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];
        let ind = rand::thread_rng().gen_range(0, 13);

        // Draw card
        self.new_card = deck[ind];

        // Recalculate hand
        self.hand = self.total;
        if self.new_card == 11 && self.ace_val == 11 { self.num_aces+=1; }
        if self.hand + self.new_card > 21 {
            if self.new_card == 11 {
                self.new_card = 1;
                if self.num_aces > 0 { self.num_aces-=1; }
            }

            self.hand = self.hand - (self.num_aces * 10);
            self.num_aces = 0;
            self.ace_val = 1;
        }

        if self.ace_val == 1 && self.new_card == 11 { self.new_card = 1; }
        self.total = self.new_card + self.hand;
    }

}

fn main() {
    // Declare and initialize user, dealer structs
    let mut user = build_player();
    let mut dealer = build_player();

    let mut user_turn = true;

    display_hands(&user, &dealer);
    let mut input = get_move();

    if input == "stand" { user_turn = false; }

    loop {
        while user_turn {
            user.draw_card();
            display_hands(&user, &dealer);

            if user.total > 21 {
                println!("BUSTED!\nYou busted. Dealer wins.");
                return;
            }

            // Get decision from user
            input = get_move();
            if input == "stand" { user_turn = false; }
        }

        if dealer.total >= 17 {
            // Determing who wins if no one busts
            println!("Dealer stands.\n");
            println!("The dealer:\n{} + {} = {}\n", dealer.hand, dealer.new_card, dealer.total);
            println!("You:\n{} + {} = {}\n", user.hand, user.new_card, user.total);

            if dealer.total > user.total {
                println!("Dealer wins.\n");
            } else if dealer.total < user.total {
                println!("You win!\n");
            } else {
                println!("Tie! (Push)\n");
            }

            return;
        }

        else {
            // Dealer's turn - loop
            while dealer.total < 17 {
                println!("\nDealer hits.\n");
                dealer.draw_card();

                // The dealer busts
                if dealer.total > 21 {
                    println!("The dealer:\n{} + {} = {}", dealer.hand, dealer.new_card, dealer.total);
                    println!("BUSTED!\n\n");
                    println!("You:\n{} + {} = {}\n\n", user.hand, user.new_card, user.total);
                    println!("The dealer busted. You win!\n");

                    return;
                }

                display_hands(&user, &dealer);
            }
        }
    }
}

fn display_hands(usr: &Player, dlr: &Player) {
    println!("The dealer:\n? + {}\n", dlr.new_card);
    println!("You:\n{} + {} = {}\n", usr.hand, usr.new_card, usr.total);
}

// Used to return a new default instance of a player
fn build_player() -> Player {
    let mut temp : Player = Player {
        hand: 0,
        new_card: 0,
        total: 0,
        num_aces: 0,
        ace_val: 11
    };

    temp.draw_card();
    temp.draw_card();
    return temp;
}

fn get_move() -> String {
    let mut input = String::new();
    loop {
        println!("Would you like to \"hit\" or \"stand\"?");
        io::stdin().read_line(&mut input).ok();
        input = String::from(input.trim());
        match input.as_ref() {
            "hit" | "stand" => return input,
                           _ => input = String::new(),
        }
    }
}
