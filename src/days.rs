mod day1;
mod day2;
mod day3;
mod day4;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;

pub trait DailyPuzzle {
    fn get_day(&self) -> i32;
    fn load_data(&mut self);
    fn get_base_answer(&self) -> Option<String>;
    fn get_bonus_answer(&self) -> Option<String>;

    fn print_answers(&self) {
        println!("========== DAY {} ===========", self.get_day());
        println!("BASE answer : {}", self.get_base_answer().unwrap_or(String::from("Not solved yet")));
        println!("BONUS answer : {}", self.get_bonus_answer().unwrap_or(String::from("Not solved yet")));
    }
}