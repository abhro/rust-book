fn main() {
    print_number(2);
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn print_number(x: u64) {
    println!("x is: {}", x);
}
