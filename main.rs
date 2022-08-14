fn main() {
    // CONSTANTE_VARIABLE vs let_variable: n'utilise pas la mémoire
    const CONSTANTE_VARIABLE: i32 = 44;
    let let_variable_with_suffix_and_casting = 45u16 as i32;
    println!("{}", CONSTANTE_VARIABLE);
}
 