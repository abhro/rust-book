fn main() {
    let mut i = 0;
    while i < 10 {
        std::thread::spawn(|| {
            let greeting_message = "Hello?";
            println!("{}", greeting_message);
        });
        i = i + 1;
    }
}
