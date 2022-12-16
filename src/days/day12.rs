///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 12 - Hill Climbing Algorithm

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from above broken into a grid; 
the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E). 
Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. 
To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, 
if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower 
than the elevation of your current square.)

What is the fewest steps required to move from your current position to the location that should get the best signal?

BONUS



Input : inputs\\day12.txt
*/

use crate::prelude::*;

pub struct Day12 {
    
}

impl Day12 {
    pub fn new() -> Day12 {
        let mut day = Day12 {
            
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day12 {
    fn get_day(&self) -> i32 {
        12
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day12.txt").map(|line| line.unwrap());
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}