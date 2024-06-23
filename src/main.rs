use rpml_macros::IntoStringHashMap;

#[derive(Debug, IntoStringHashMap)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
}

fn main() {
    println!("Hello, world!");
}
