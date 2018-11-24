fn main() {
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, California, Colorado, Connecticut,
    Delaware, Florida, Georgia, Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas,
    Kentucky, Louisiana, Maine, Maryland, Massachusetts, Michigan, Minnesota,
    Mississippi, Missouri, Montana, Nebraska, Nevada, NewHampshire, NewJersey,
    NewMexico, NewYork, NorthCarolina, NorthDakota, Ohio, Oklahoma, Oregon,
    Pennsylvania, RhodeIsland, SouthCarolina, SouthDakota, Tennessee, Texas,
    Utah, Vermont, Virginia, Washington, WestVirginia, Wisconsin, Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
