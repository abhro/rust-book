/// Add one to the number given
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #    x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    println!("5+1 is: {}", add_one(5));
}
