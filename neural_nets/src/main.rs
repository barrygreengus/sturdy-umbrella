extern crate rand;

use std::io;
use std::fmt;
//use std::option;
//use std::cmp::Ordering;
use rand::Rng;

struct BoundedQueue<T> {
    queue: Vec<T>,
    max_size: usize,
}

impl<T: fmt::Display> BoundedQueue<T> {
    fn new(max_size: usize) -> BoundedQueue<T> {
        BoundedQueue::<T> {
            queue: Vec::new(),
            max_size: max_size,
        }
    }

    fn enqueue(&mut self, elem: T) {
        // insert elem to front of queue
        self.queue.insert(0, elem);
        // if queue past capacity, remove from end
        if self.queue.len() > self.max_size {
            self.queue.pop();
        }
    }

    /*
    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop()
    }
    */

    fn print(&self) {
        println!("BoundedBuffer: max_size={}, len={}", self.max_size, self.queue.len());
        print!("[ ");
        for item in self.queue.iter() {
            print!("{} ", item);
        }
        println!("]");
    }
}

fn get_user_guess() -> String {
    let mut user_guess = String::new();
    loop {
        io::stdin().read_line(&mut user_guess)
            .expect("failed to read line");
        match user_guess.to_lowercase().trim() {
            "rock"     => break,
            "paper"    => break,
            "scissors" => break,
            _          => println!("Invalid guess ... "),
        };
    }
    String::from(user_guess.trim())
}

fn get_comp_guess() -> String {
    let rando = rand::thread_rng().gen_range(0, 3);
    match rando {
        0 => String::from("rock"),
        1 => String::from("paper"),
        2 => String::from("scissors"),
        _ => String::from("Error!"),
    }
}

fn a_beat_b(a: &str, b: &str) -> bool {
    match (a, b) {
        ("rock", "scissors") => true,
        ("scissors", "paper") => true,
        ("paper", "rock") => true,
        _ => false
    }
}

fn main() {
    println!("Let's play Rock Paper Scissors!");
    println!("Input your moves as 'rock', 'paper', or 'scissors'");

    let mut round_num = 0;
    let mut user_wins = 0;
    let mut comp_wins = 0;
    let mut ties = 0;
    let mut buffer = BoundedQueue::<String>::new(3);
    let mut last_comp_move = String::from("");
    let mut comp_won_last_round = false;

    // --- Queue Memory ---
    loop {
        round_num += 1;
        if user_wins == 5 || comp_wins == 5 {
            println!("\nFINAL SCORE | User: {}, Computer: {}, Ties: {}", user_wins, comp_wins, ties);
            buffer.print();
            break;
        }
        println!("\nRound #{}, guess rock, paper, or scissors ...", round_num);
        // create random guess
        let comp_guess = match last_comp_move.as_str() {
            "rock"     if comp_won_last_round  => String::from("scissors"),
            "paper"    if comp_won_last_round  => String::from("rock"),
            "scissors" if comp_won_last_round  => String::from("paper"),
            _                                  => get_comp_guess(),
        };
        //}else{
        //    let comp_guess = get_comp_guess();
        //}
        // get user guess
        let user_guess = get_user_guess();

        print!("you guessed {}, comp guessed {}. ", user_guess, comp_guess);
        if a_beat_b(user_guess.as_str(), comp_guess.as_str()) {
            println!("You won!");
            user_wins += 1;
        }else if a_beat_b(comp_guess.as_str(), user_guess.as_str()) {
            println!("You lost!");
            comp_wins += 1;
            comp_won_last_round = true;
        }else {
            println!("Tie!");
            ties += 1;
        }
        // record moves
        buffer.enqueue(user_guess);
        println!("User: {}, Computer: {}, Ties: {}", user_wins, comp_wins, ties);
        last_comp_move = comp_guess;
    }
}
