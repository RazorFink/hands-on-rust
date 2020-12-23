#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn extend_greeting(&self) {
        println!("{}.", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read a line!");
    your_name.trim().to_lowercase() //return statement w/ no semicolon
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    let visitor_list = [
        Visitor::new("bert", "Hi Bert, would you like a sandwich?"),
        Visitor::new("fred", "Hi Fred, how's it hangin?"),
        Visitor::new("steve", "Is that Steve?  Steeeeve..."),
        Visitor::new("carl", "What the fuck're you doing Carl?"),
    ];
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.extend_greeting(),
        None => println!("You are not known to us..."),
    }
}
