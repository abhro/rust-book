use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, run for {} minutes!",
                     expensive_result.value(intensity));
        }
    }
}

pub struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: HashMap<u32, u32>,   // argument, value
}

impl<T: Fn(u32) -> u32> Cacher<T> {

    pub fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: HashMap::new() }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        if !self.value.contains_key(&arg) {
            self.value.insert(arg, (self.calculation)(arg));
        }
        *self.value.get(&arg).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cacher_holds_a_value() { // doesn't make sure it only calls once
        let mut c = Cacher::new(|b| b + 3);
        c.value(2);
        assert_eq!(c.value(2), 5);
    }

    #[test]
    fn call_cacher_with_different_values() {
        let mut c = Cacher::new(|a| a);

        c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
