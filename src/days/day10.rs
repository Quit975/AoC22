///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 10 - Cathode-Ray Tube

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

Situations like this must be why the Elves prioritized getting the communication system on your handheld device working. 
You pull it out of your pack, but the amount of water slowly draining from a big crack in its screen tells you it probably won't be of much immediate use.

Unless, that is, you can design a replacement for the device's video system! It seems to be some kind of cathode-ray tube screen and simple CPU that are both 
driven by a precise clock circuit. The clock circuit ticks at a constant rate; each tick is called a cycle.

Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1. It supports only two instructions:

    addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
    noop takes one cycle to complete. It has no other effect.

The CPU uses these instructions in a program (your puzzle input) to, somehow, tell the screen what to draw.

Maybe you can learn something by looking at the value of the X register throughout execution. For now, consider the signal strength 
(the cycle number multiplied by the value of the X register) during the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th, 100th, 140th, 180th, 
and 220th cycles).

Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?

BONUS



Input : inputs\\day10.txt
*/

use crate::prelude::*;

pub struct Day10 {
}

impl Day10 {
    pub fn new() -> Day10 {
        let mut day = Day10 {
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day10 {
    fn get_day(&self) -> i32 {
        10
    }

    fn load_data(&mut self) {
        //let lines = read_lines("inputs\\day10.txt");
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}
