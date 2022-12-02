mod input;
mod solutions;

mod prelude {
    pub use crate::input::*;
}

use solutions::*;

fn main() {
    day1();
    day2();
    test();
}

fn day1() {
    println!("========== DAY 1 ===========");
    println!("Day 1 - BASE answer : {}", day1_base());
    println!("Day 1 - BONUS answer : {}\n", day1_bonus());
}

fn day2() {
    println!("========== DAY 2 ===========");
    println!("Day 2 - BASE answer : {}", day2_base());
    println!("Day 2 - BONUS answer : {}\n", day2_bonus());
}

fn test() {
    println!("========== TEST ===========");
    println!("Test - BASE answer : {}", test_base());
    println!("Test - BONUS answer : {}\n", test_bonus());
}