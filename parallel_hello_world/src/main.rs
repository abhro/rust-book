fn main() {
    let i = 0;
    while i < 10 {
        std::thread::spawn(/*proc()*/ {
            let greeting_message = "Hello?";
            println!("{}", greeting_message);
        });
        let i = i + 1;
    }
}
