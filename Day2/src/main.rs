use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::str;

fn return_based_on_type(choice: &str) -> i32 {
    match choice {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => panic!("This will never happen!"),
    }
}
fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("../input.txt")?;
    let mut vec: Vec<(&str, &str)> = Vec::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let mut tuple: (&str, &str) = (split.next().unwrap(), split.next().unwrap());

        match tuple.0 {
            "A" => tuple.0 = "rock",     // Rock
            "B" => tuple.0 = "paper",    // Paper
            "C" => tuple.0 = "scissors", // Scissors
            _ => panic!("This should never happen."),
        }
        match tuple.1 {
            "X" => tuple.1 = "lose",
            "Y" => tuple.1 = "draw",
            "Z" => tuple.1 = "win",
            _ => panic!("This should never happen."),
        }

        vec.push(tuple);
    }
    let mut points = 0;
    for tup in vec {
        if tup.1 == "draw" {
            points += 3 + return_based_on_type(tup.0)
        }
        if tup.1 == "lose" {
            match tup.0 {
                "rock" => points += return_based_on_type("scissors"),
                "paper" => points += return_based_on_type("rock"),
                "scissors" => points += return_based_on_type("paper"),
                _ => panic!("This will never happen!"),
            }
        }
        if tup.1 == "win" {
            match tup.0 {
                "rock" => points += 6 + return_based_on_type("paper"),
                "paper" => points += 6 + return_based_on_type("scissors"),
                "scissors" => points += 6 + return_based_on_type("rock"),
                _ => panic!("This will never happen!"),
            }
        }
    }
    println!("{}", points);
    Ok(())
}
