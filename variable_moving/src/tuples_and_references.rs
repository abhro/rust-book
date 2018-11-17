fn main() {
    let s1 = String::from("hello");

    let (s2, length) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, length);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    return (s, length);
}
