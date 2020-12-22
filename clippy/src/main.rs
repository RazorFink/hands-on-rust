#![warn(clippy::all, clippy::pedantic)]
fn main() {
    let mylist = ["One", "two", "A-three"];
    for num in &mylist {
        println!("{}", num);
    }
}
