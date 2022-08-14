use std::collections::HashMap;

/// `main` is a function that is called when the program is executed.
fn main() {
    let tableau: [u8; 4] = [3,4,5,2];
    let tableau2 = [[3,4], [2, 3]];
    let tuples = (2, "sjl", 22);
    // vecteur ~tableau & contient qu'un seul type & taille non fixe
    let un_vecteur = vec![2, 3];
    println!("{:?}", tableau);
    println!("{:?}", tableau2[0][1]);
    println!("{:?}", un_vecteur);

    let value_get = handle_match_value(un_vecteur.get(2));
    println!("value_get: {:?}", value_get);
    
    // doit être mutable le ve dynamique
    let mut vecteur = Vec::new();
    vecteur.push(23u8);
    // vecteur.push("value"); // la première valeur set le type

    // Hashmap
    let mut hashmap_var = HashMap::new();
    hashmap_var.insert("a", "a value");
    println!("is exist ? {:?}", hashmap_var.contains_key("b"));
}

fn handle_match_value(&mut arg: Option<&I::Output>) -> String {
    // * déreference on a que la valeur donc
    match *arg {
        Some(x) => println!("{}", x),
        None => println!("none value"),
    }
}
