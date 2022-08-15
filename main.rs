
/// `main` is a function that is called when the program is executed.
fn main() {
    let a: u8 = 48;
    let b: u8 = 32;
    println!("{:?}", superior(calcul, a, b));
    // NOTE: closure , fn anonyme, arg default generique
    println!("closure {:?}", superior(|x,y| x+y, a, b));
    // println!("closure {:?}", superior(|x: u8,y: u8| x+y, a, b));
    println!("closure conscience de son en {:?}", superior_closure(|| a+b));
    let affiche = |arg: String| println!("{}", arg);
    affiche(String::from("affiche"));
    // move: force la proprieté
    let affiche_move= move || println!("affiche_move() {}", a);
    affiche_move();
    println!("{}", a);
}

// 'a = copy la durée de vie car une fois scope est fini les vars s'efface avec lui
fn superior<F>(function: F, v1: u8, v2: u8) 
where F: Fn( u8, u8) -> u8
{
    println!("{}", function(v1, v2));
}
fn superior_closure<F>(function: F) 
where F: Fn( ) -> u8
{
    println!("{}", function());
}

fn calcul(v1: u8, v2: u8) -> u8 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
