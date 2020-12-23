#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn extend_greeting(&self) {
        println!("{}.", self.greeting);
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read a line!");
    your_name.trim().to_lowercase() //return statement w/ no semicolon
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 25),
        Visitor::new("fred", VisitorAction::AcceptWithNote(note: String::from("Have some juice"), 15),
        Visitor::new("steve", VisitorAction::Refuse, 85),
        Visitor::new("carl", VisitorAction::Accept, 21),
    ];
    loop {
        println!("Hello, what's your name?");
        println!("Enter a blank line to quit.");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.extend_greeting(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("You are not known to us...adding to the list!");
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
    println!("The final list:");
    println!("{:#?}", visitor_list);
}
