use std::fs::File;


/// `main` is a function that is called when the program is executed.
fn main() {
    // NOTE: panic
    // panic!("PANIC ! CRASH PROGRAMME");

    // unwrap
    let file = File::open("./README.md")
    // unwrap
    // .unwrap();
    .expect("Impossible de lire le ficher");
    print!("{:?}", file);
}
