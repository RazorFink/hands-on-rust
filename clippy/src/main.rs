fn main() {
    let mylist = ["One", "two", "A-three"];
    for num in &mylist {
        println!("{}", num);
    }
}
