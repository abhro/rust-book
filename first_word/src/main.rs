fn main() {
    let s = String::from("this sentence has five words");
    println!("\"{}\"'s first word is at index {}", s, first_word(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
