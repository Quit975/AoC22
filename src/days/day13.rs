///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 13 - Distress Signal

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

Your handheld device must still not be working properly; the packets from the distress signal got decoded out of order. 
You'll need to re-order the list of received packets (your puzzle input) to decode the message.

Your list consists of pairs of packets; pairs are separated by a blank line. You need to identify how many pairs of packets are in the right order.

Packet data consists of lists and integers. Each list starts with [, ends with ], and contains zero or more comma-separated values (either integers or other lists). 
Each packet is always a list and appears on its own line.

When comparing two values, the first value is called left and the second value is called right. Then:

    If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. 
    If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; 
    continue checking the next part of the input.

    If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, 
    the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. 
    If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.

    If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. 
    For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].

What are the indices of the pairs that are already in the right order? (The first pair has index 1, the second pair has index 2, and so on.) 
In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.

Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?

BONUS



Input : inputs\\day13.txt
*/

use crate::prelude::*;

pub struct Day13 {
    
}

impl Day13 {
    pub fn new() -> Day13 {
        let mut day = Day13 {
            
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day13 {
    fn get_day(&self) -> i32 {
        13
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day13.txt").map(|line| line.unwrap());
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}