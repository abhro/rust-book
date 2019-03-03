#![allow(unused_variables)]

struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let q = Point { x: 4, y: 5 };
    let Point { x, y } = q;
    assert_eq!(4, x);
    assert_eq!(5, y);

    match p {
        Point { x,    y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y    } => println!("On the y axis at {}", y),
        Point { x,    y    } => println!("On neither axis at ({}, {})", x, y),
    }

    let ((feet, inches), Point { x: r, y: t }) = ((5, 10), Point { x: 3, y: -10 });
    println!("At {}'{}\", the subject is standing on ({}, {})", feet, inches, r, t);

    destructure_refs();
}

fn destructure_refs() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter()
                                    .map(| &Point { x, y } | x * x + y * y)
                                    .sum();
}
