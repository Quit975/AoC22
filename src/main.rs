mod input;
mod days;

mod prelude {
    pub use crate::input::*;
    pub use crate::days::*;
}

use days::*;

fn main() {
    Day1::new().print_answers();
    Day2::new().print_answers();
    Day3::new().print_answers();
    Day4::new().print_answers();
    Day5::new().print_answers();
    Day6::new().print_answers();
    Day7::new().print_answers();
}