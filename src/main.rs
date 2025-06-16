use std::string;

fn main() {
    println!("Hello from Rust!");
    // test_func();
    let myresult = get_full_name("Angus", "Higgins");
    println!("Hello from {}", myresult);
}

fn get_full_name(first: &str, last: &str) -> String {
    let full_name: String = format!("{} {}", first, last);
    return full_name;
}

#[allow(dead_code)]
fn test_func() {
}
