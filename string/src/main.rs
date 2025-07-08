// &str is string slice and its immutable it not ownes it
fn print_str(s: &str) {
    // let mut new_string = s.to_string();
    // new_string.push_str(" some other string");
    let new_string = format!("{} other stuff here", s);
    println!("{}", new_string);
}

// its string and its mutable it owns it
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "Hello, Vrushabh!";
    print_str(s);

    let salutation = String::from("hello");
    print_string(salutation);
}