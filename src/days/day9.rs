///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 9 - Rope Bridge

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves far enough away from the tail, the tail is pulled toward the head.

Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid. 
Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.

Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching 
(diagonally adjacent and even overlapping both count as touching)

If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough
Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up
You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.

Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?

BONUS



Input : inputs\\day9.txt
*/

use crate::prelude::*;

pub struct Day9 {
}

impl Day9 {
    pub fn new() -> Day9 {
        let mut day = Day9 {
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day9 {
    fn get_day(&self) -> i32 {
        9
    }

    fn load_data(&mut self) {
        //let lines = read_lines("inputs\\day9.txt");
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}
