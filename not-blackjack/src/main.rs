extern crate rand;

use rand::Rng;
use std::io;

struct Hand {
    name: String,
    cards: Vec<u8>
}

impl Hand {
    fn new(name: String) -> Hand {
        let mut cards: Vec<u8> = Vec::new();
        for _ in 0..2 {
            cards.push(rand::thread_rng().gen_range(2, 10));
        }
        Hand { name, cards }
    }

    fn count(&self) -> u8 {
        let mut card_count = 0;
        for card in self.cards.iter() {
            card_count += card;
        }
        card_count
    }

    fn show(&self) -> () {
        println!("{}'s cards:", self.name);
        for card in self.cards.iter() {
            print!("{} ", card);
        }
        println!();
        println!("Count is {}", self.count());
    }

    fn draw(&mut self){
      self.cards.push(rand::thread_rng().gen_range(2, 10));
    }

    fn check_lost(&self) -> bool {
        self.count() > 21
    }

    fn draw_until(&mut self, until: u8) {
        while self.count() < until {
            self.draw();
        }
    }
}

fn main() {
    let mut players_hand = Hand::new("Player".to_string());
    let mut dealers_hand = Hand::new("Dealer".to_string());

    players_hand.show();

    loop {
        println!("You can (s)tart or (d)raw more cards, what is your move?");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read your command!");

        let choice = choice.trim();

        match choice {
            "s" => {
                dealers_hand.draw_until(17);
                let difference: i8 = players_hand.count() as i8 - dealers_hand.count() as i8;
                dealers_hand.show();
                if (difference > 0 || dealers_hand.check_lost()) && !players_hand.check_lost() {
                    println!("You won!!");
                } else if difference == 0 {
                    println!("Its a draw!!");
                } else {
                    println!("You lost!!");
                }
                break;
            },
            "d" => {
                players_hand.draw();
                if players_hand.check_lost() {
                    println!("You went over 21, you lost!!");
                    break;
                }
                players_hand.show();
            },
            _ => println!()
        }
    }
}
