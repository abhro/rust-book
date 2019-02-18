fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() /* or &a */ {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let mut number = 5;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("We have safe touchdown");
}
