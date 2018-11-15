// this code is supposed to not compile

fn main() {
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);
}
