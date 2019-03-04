#![allow(dead_code)]

fn main() {
    let mut num = 5;

    let r1: *const i32 = &num as *const i32;
    let r2: *mut i32 = &mut num as *mut i32;

    println!("r1 = {:?}", r1);
    println!("r2 = {:?}", r2);

    unsafe {
        println!("*r1 is: {}", *r1);
        println!("*r2 is: {}", *r2);
    }

    let address: usize = 0x012345;
    let r: *const i32 = address as *const i32;
    println!("r = {:?}", r);

    unsafe {
        dangerous();
    }

    use_split_at_mut();

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}

fn use_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid),
         std::slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))

        // (&mut slice[..mid], &mut slice[mid..])
    }
}
