/* 
////////////////////////////////////////////////////////////////////////////////////////////////////
/// DAY 1 - CALORIES
///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. 
Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

BONUS

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

Input - inputs\\day1.txt  
*/


use crate::prelude::*;

pub struct Day1 {
    calorie_sums : Vec<i32>
}

impl Day1 {
    pub fn new() -> Self {
        let mut day = Self {
            calorie_sums : Vec::new()
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day1 {
    fn get_day(&self) -> i32 {
        1
    }

    fn load_data(&mut self) {     
        let lines  = read_lines("inputs\\day1.txt").map(|line| line.unwrap());
        let mut current_sum : i32 = 0;

        for line in lines {

            if let Ok(calories_num) = line.parse::<i32>() {
                current_sum += calories_num;
            }
            else {
                self.calorie_sums.push(current_sum);
                current_sum = 0;
            }
        }

        self.calorie_sums.sort_unstable_by(|a, b| b.cmp(a));
    }

    fn get_base_answer(&self) -> Option<String> {
        Some(self.calorie_sums[0].to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        Some((self.calorie_sums[0] + self.calorie_sums[1] + self.calorie_sums[2]).to_string())
    }
}