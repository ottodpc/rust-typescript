// Oject
struct Personne {
    name: String,
    age: String,
}
// 2 types: methods ass. et methods. Les methods associées seront accessible comme les methods static sans l'instanciation de la class
impl Personne {
    fn new(name: String, age: String) -> Personne {
        Personne {  name: name, age: age }
    }
}
/// `main` is a function that is called when the program is executed.
fn main() {
    let Cyprien = Personne::new("Cyprien".to_owned(),"infini".to_owned());
    println!("Name {}", Cyprien.name);
}
