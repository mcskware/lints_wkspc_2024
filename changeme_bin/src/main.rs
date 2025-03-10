//! Binary crate

/// Main function
fn main() {
    let greeting = changeme_lib::get_greeting_message();
    println!("{greeting}");
}
