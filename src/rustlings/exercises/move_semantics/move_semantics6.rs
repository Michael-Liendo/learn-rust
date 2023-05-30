// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Debe tomar posesión
fn string_uppercase(data: &String) {
    let mut owned_data = data.to_owned();
    owned_data = owned_data.to_uppercase();

    println!("{}", owned_data);
}
