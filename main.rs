fn main() {
    let a = get_a();
    // match & pattern binding avec tuple & b n'est pas stocké memoire
    let ddd = ("a", 22, ("e", 78));
    match ddd {
        (a, _, (mut c, d)) if a == "e" => println!("{}", c),
        _ => print!("default")
    }
}
 
fn get_a() {
    let a: i32 = 34;
    if  a > 0 {
        // return a;
        "a > 12";
    } else {
        "a < 12";
    }
}

// match ~= de Switch case
