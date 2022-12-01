mod input;
mod solutions;

mod prelude {
    pub use crate::input::*;
}

use solutions::*;

fn main() {
    day1();
    test();
}

fn day1() {
    println!("========== DAY 1 ===========");
    println!("Day 1 - BASE answer : {}", day1_base());
    //println!("Day 1 - BONUS answer : {}\n", test_bonus());
}

fn test() {
    println!("========== TEST ===========");
    println!("Test - BASE answer : {}", test_base());
    println!("Test - BONUS answer : {}\n", test_bonus());
}