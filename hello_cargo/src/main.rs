use std::cell::RefMut;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 10;

    println!("Is {} > {}: {}", x, y, is_greater(x, y));

    println!("{}", x);
    let s1 = "Hey";

    let s = String::from(s1);
    let uppercased = take_ownership(&s);
    println!("upper cased {} -> {}", s, uppercased);
}

fn is_greater(to_compare: i32, base: i32) -> bool {
    let result: bool = if to_compare > base { true } else { false };

    result
}

fn take_ownership(s: &String) -> String {
    return s.to_uppercase();
}
