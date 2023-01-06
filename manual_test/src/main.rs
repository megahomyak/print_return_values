#[print_return_values::print_return_values]
pub fn a(b: String) -> String {
    b.to_lowercase()
}

fn main() {
    a(String::from("ABC"));
}
