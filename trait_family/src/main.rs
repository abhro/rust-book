use std::fmt::{self, Display, Formatter};

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point { x: i32, y: i32 }

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    Point { x: 5, y: 10 }.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
