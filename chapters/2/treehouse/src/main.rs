#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;
fn what_is_your_name() -> String{
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read a line!");
    your_name
        .trim()
        .to_lowercase() //return statement w/ no semicolon
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Well, howdy {}!", name);
    let visitor_list = ["bert", "fred", "steve", "carl"];
    let mut is_allowed = false;
    for visitor in &visitor_list {
        if &name == visitor {
            is_allowed = true;
        }
    }
    if is_allowed {
        println!("You're on the list!");
    } else { 
        println!("You're not on the list...");
    }
}
