fn main() {
    // ERROR: borrow of moved value: `c_var`
    // Chaine de character est stocké dans le heap et le stack stock son pointeur
    // il ne peut y avoir un seul pointeur (address). #Move #Ownership
    let c_var = "c variable".to_owned();
    // let b_var = c_var;
    // println!("{} {}", c_var, b_var);

    // Scope peut-être créer n'importe ou
    {
        // si borrow dans le scope avec c_var, c_var n'existera plus
    }
    
    // Résolution: mais c_var n'existera plus
    let mut d_var = c_var.clone();
    d_var.push_str("mutable moved value");
    println!("{}, {}", d_var, c_var);
}
 