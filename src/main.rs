// Blackjack
// by Chris Seifried

extern crate rand;
use rand::Rng;

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
    let mut user : Player = build_player();
    let mut dealer : Player = build_player();
    //user.draw_card(); user.draw_card();
    //dealer.draw_card(); dealer.draw_card();

    display_hands(&user, &dealer);

    let mut is_running : bool = true;
    let mut user_turn : bool = true;
}

fn display_hands(usr: &Player, dlr: &Player) {
    println!("The dealer:\n? + {}\n", dlr.new_card);
    println!("You:\n{} + {} = {}", usr.hand, usr.new_card, usr.total);
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
