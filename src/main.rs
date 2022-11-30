mod input;
mod solutions;

mod prelude {
    pub use crate::input::*;
}

use solutions::*;

fn main() {
    test();
}

fn test() {
    println!("========== TEST ===========");
    println!("Test - BASE answer : {}", test_base());
    println!("Test - BONUS answer : {}\n", test_bonus());
}