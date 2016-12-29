extern crate rand;

use std::io;
use std::io::stdout;  // to get stdout
use std::io::Write;   // to use stdout().flush()
use std::cmp::Ordering;
use rand::Rng;

// --------- I/O functions ---------------
fn get_user_int() -> u32 {
    let mut num = 0;
    while num == 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");
        num = match input.trim().parse::<u32>() {
            Ok(x)   => x,
            Err(_)  => 0,
        };
        if num == 0 {
            println!("Invalid: Please enter a numeral greater than 0");
        }
    }
    num
}

fn get_user_guess() -> String {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");
        match input.to_lowercase().trim() {
            "rock"     => return String::from("rock"),
            "paper"    => return String::from("paper"),
            "scissors" => return String::from("scissors"),
            _          => {
                println!("Invalid: Please enter 'rock', 'paper', or 'scissors'");
                continue;
            },
        };
    }
}

// ------------ functions for computer to generate a guess ------------------
fn get_random_guess() -> String {
    let rando = rand::thread_rng().gen_range(0, 3);
    match rando {
        0 => String::from("rock"),
        1 => String::from("paper"),
        2 => String::from("scissors"),
        _ => String::from("Error!"),
    }
}

fn get_comp_guess(last_comp_move: String, comp_won_last_round: bool) -> String {
    // if the comp won last round, we think the human may
    // do what would have beaten the comps winning choice
    match last_comp_move.as_str() {
        "rock"     if comp_won_last_round  => String::from("scissors"),
        "paper"    if comp_won_last_round  => String::from("rock"),
        "scissors" if comp_won_last_round  => String::from("paper"),
        _                                  => get_random_guess(),
    }
}

// the rock paper scissors win logic
fn a_beat_b(a: &str, b: &str) -> bool {
    match (a, b) {
        ("rock", "scissors") => true,
        ("scissors", "paper") => true,
        ("paper", "rock") => true,
        _ => false
    }
}

// function to go through one whole round of games
fn play_round(games_to_win: u32) -> (u32, u32, u32) {
    let mut game_num = 0;
    //      score = (#userwins, #compwins, #ties)
    let mut score = (0, 0, 0);
    // give it initial value so the match statement is kosher
    let mut last_comp_move = String::from("");
    let mut comp_won_last_round = false;

    loop {
        match score {
            (u, c, t) if u == games_to_win => {
                println!("You won the round! {} to {} with {} ties.", u, c, t);
                return score;
            },
            (u, c, t) if c == games_to_win => {
                println!("You lost the round {} to {} with {} ties.", u,c, t);
                return score;
            },
            _ => {},
        };

        game_num += 1;
        // get guesses
        println!("\nGame #{}\nGuess rock, paper, or scissors ...", game_num);
        let comp_guess = get_comp_guess(last_comp_move, comp_won_last_round);
        let user_guess = get_user_guess();

        // determine annd record results
        print!("you guessed {}, comp guessed {}. ", user_guess, comp_guess);
        let user_won = a_beat_b(user_guess.as_str(), comp_guess.as_str());
        let comp_won = a_beat_b(comp_guess.as_str(), user_guess.as_str());
        match (user_won, comp_won) {
            (true, false) => {
                println!("You won!");
                score.0 += 1; //user_wins += 1;
                comp_won_last_round = false;
            },
            (false, true) => {
                println!("You lost!");
                score.1 += 1; //comp_wins += 1;
                comp_won_last_round = true;
            },
            (false, false) => {
                println!("Tie!");
                score.2 += 1; //ties += 1;
                comp_won_last_round = false;
            },
            _ => println!("If this prints, something is wrong."),
        };
        println!("Current Score: you: {}, computer: {}, ties: {}", score.0, score.1, score.2);
        last_comp_move = comp_guess;
    }
}

fn main() {
    println!("Let's play Rock Paper Scissors!");
    println!("Input your moves as 'rock', 'paper', or 'scissors'");
    println!("------------------------------------------------\n");

    print!("Play best out of how many rounds?: ");
    stdout().flush().expect("Could not flush stdout.");  // print! doesn't flush stdout itself
    let num_rounds = get_user_int();
    print!("\n");
    print!("Play each round to how many games?: ");
    stdout().flush().expect("Could not flush stdout.");  // print! doesn't flush stdout itself
    let games_per_round = get_user_int();
    print!("\n");

    let mut user_wins = 0;
    let mut comp_wins = 0;
    for round in 0..num_rounds {
        let score = play_round(games_per_round);
        match score.0.cmp(&score.1) {
            Ordering::Less    => comp_wins += 1,
            Ordering::Greater => user_wins += 1,
            Ordering::Equal   => {/*do nothing*/},
        }
        println!("After {} rounds: User has {} rounds to the computer's {} rounds", round+1, user_wins, comp_wins);
    }
    println!("Final Score: User [{}], Computer [{}]", user_wins, comp_wins);
}

