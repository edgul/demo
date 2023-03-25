// blackjack - single player
//
// hit or stand 
// currently no split, double or insurance
// no betting, just play

use std::fmt;
use rand::Rng;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

impl Suit {
    pub fn all() -> Vec<Suit> {
        vec![Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
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

impl Rank {
    pub fn all() -> Vec<Rank> {
        vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
        Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
        Rank::Jack, Rank::Queen, Rank::King]
    }

    pub fn value(&self) -> u8 {
        match self {
            Rank::Ace => 1, // handle +10 with hand context
            Rank::Two => 2,
            Rank::Three=> 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six=> 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            _ => 10,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Card {
    suit : Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

impl Card {
    pub fn all() -> Vec<Card> {
        let mut v = vec![];
        for s in Suit::all() {
            for r in Rank::all() {
                v.push( Card { suit : s, rank : r });
            }
        }
        v
    }

    pub fn value_or_bust(cards: &Vec<Card>) -> Option<u8> {
        let mut aces = 0;
        let mut count = 0;
        for c in cards {
            if c.rank == Rank::Ace {
                aces += 1;
                continue;
            }
            count += c.rank.value();
        }

        // only ever need one high-value ace
        if aces > 0 && count + 10 <= 21 {
            count += 10; 
        }

        // bust
        if count > 21 {
            return None;
        }

        Some(count)
    }
}

// put all the items in a hat and randomly draw them out
fn shuffle(deck: &mut Vec<Card>) {
    let mut hat = vec![];
    while deck.len() > 0 {
        hat.push(deck.remove(0));
    }
    assert!(&deck.is_empty());
   
    while hat.len() > 0 {
        let i = rand::thread_rng().gen_range(0..hat.len());
        deck.push(hat.remove(i));
    }
}

fn easy_print(str : &str, user_hand : &Vec<Card>) {
    print!("{}", str);
    for c in user_hand {
        print!(" {}, ", c);
    }
    println!("");
}

fn main() {
    // shuffle
    let mut deck = Card::all();
    shuffle(&mut deck);

    // deal user and dealer
    let mut user_hand = vec![];
    let mut dealer_hand = vec![];
    user_hand.push(deck.remove(0));
    dealer_hand.push(deck.remove(0));
    user_hand.push(deck.remove(0));
    dealer_hand.push(deck.remove(0));

    // prompt user
    println!("Dealer is showing: {}", dealer_hand.first().unwrap());
    easy_print("You have: ", &user_hand);
    let mut input = String::new();

    // user loop until stand 
    loop {
        println!("(h)it or (s)tay?");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("{}", input);
                let t_input = input.trim();
                if t_input.len() < 1 {
                    continue;
                }
                let first_char = t_input.chars().last().unwrap().to_lowercase().next().unwrap();
                if first_char == 'h' {
                    user_hand.push(deck.remove(0));
                    
                    if Card::value_or_bust(&user_hand).is_none() {
                        easy_print("BUST: ", &user_hand);
                        println!("You Lose.");
                        return;
                    }
                    easy_print("You have :", &user_hand);
                } 
                else if first_char == 's' {
                    break;
                }
            }
            Err(_) => println!("Error getting input, try again"),
        }
    }

    // dealer loop until 17+
    loop {
        let count = Card::value_or_bust(&dealer_hand);
        if count.is_none() {
            easy_print("Dealer busts with: ", &dealer_hand);
            println!("You win!");
            return;
        }
        if count.unwrap() > 16 {
            break;
        }
        dealer_hand.push(deck.remove(0));
    }

    // win/lose message
    let user_count = Card::value_or_bust(&user_hand).unwrap(); // unnecessary compute
    let dealer_count = Card::value_or_bust(&dealer_hand).unwrap();
    println!("User: {}, Dealer: {}", user_count, dealer_count);
    if user_count < dealer_count {
        println!("Dealer wins.");
    } else if user_count == dealer_count {
        println!("Tie game!");
    } else {
        println!("User wins");
    }
}
