#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub length: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.length
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 7, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle { width: 7, length: 8 };
        let smaller = Rectangle { width: 1, length: 5 };

        assert!(!smaller.can_hold(&larger));
    }
}
