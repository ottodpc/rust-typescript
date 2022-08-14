fn main() {
    // Ownership vs borrowing: avec Ownership/Scope la var de base (a) n'existe plus résolution: b = a.clone() copie de a donc mais avec borrowing "&" b utilise emprête la valeur de a et a existe tjr
    let mut a = String::from("a");
    let b = &a;
    // variable_mutable_sur_a est proprietaire de la valeur de a
    let variable_mutable_sur_a = &mut a;
}
