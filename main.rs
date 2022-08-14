use std::fs::File;


/// `main` is a function that is called when the program is executed.
fn main() {
    let a: u8 = 48;
    let b: u8 = 32;
    println!("{}", superior(&a, &b))
}

// 'a = copy la durée de vie car une fois scope est fini les vars s'efface avec lui
fn superior<'a>(v1: &'a u8, v2: &'a u8) -> &'a u8 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
