fn main() {
    diverges();
}

fn diverges() -> ! {
    panic!("this function never returns!");
}
