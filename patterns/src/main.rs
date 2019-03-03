#![allow(dead_code)]

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age = "34".parse::<u8>();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    }
    else if is_tuesday {
        println!("Tuesday is green day!");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }
        else {
            println!("Using orange as the background color");
        }
    }
    else {
        println!("Using blue as the background color");
    }


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{0} is at index {1}, or, v[{1}] = {0}", value, index);
    }

    let (x, y): (i32, i32) = (4, 1);
    println!("x = {}, y = {}", x, y);

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let (x, y) = (Some(3), 10);
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 'c';
    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'o' => println!("middle ASCII letter"),
        'p' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    foo(3, 4);

    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3 ... 7 } =>
            println!("Found an id in range: {}", id_variable),

        Message::Hello { id: 10 ... 12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
