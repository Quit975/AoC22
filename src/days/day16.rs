///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 16 - Proboscidea Volcanium

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The sensors have led you to the origin of the distress signal: yet another handheld device, just like the one the Elves gave you. 
However, you don't see any Elves around; instead, the device is surrounded by elephants! They must have gotten lost in these tunnels, 
and one of the elephants apparently figured out how to turn on the distress signal.

The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan the cave with your handheld device; 
it reports mostly igneous rock, some ash, pockets of pressurized gas, magma... this isn't just a cave, it's a volcano!

You need to get the elephants out of here, quickly. Your device estimates that you have 30 minutes before the volcano erupts, 
so you don't have time to go back out the way you came in.

You scan the cave for other options and discover a network of pipes and pressure-release valves. You aren't sure how such a system got into a volcano, 
but you don't have time to complain; your device produces a report (your puzzle input) of each valve's flow rate if it were opened (in pressure per minute) 
and the tunnels you could use to move between the valves.

There's even a valve in the room you and the elephants are currently standing in labeled AA. 
You estimate it will take you one minute to open a single valve and one minute to follow any tunnel from one valve to another. What is the most pressure you could release?

All of the valves begin closed. You start at valve AA, but it must be damaged or jammed or something: its flow rate is 0, so there's no point in opening it. 
However, you could spend one minute moving to valve BB and another minute opening it; 
doing so would release pressure during the remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 = 364. 
Then, you could spend your third minute moving to valve CC and your fourth minute opening it, 
providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52 total pressure released by valve CC.

Making your way through the tunnels like this, you could probably open many or all of the valves by the time 30 minutes have elapsed. 
However, you need to release as much pressure as possible, so you'll need to be methodical. 

Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?


BONUS



Input : inputs\\day16.txt
*/

use crate::prelude::*;

pub struct Day16 {
    
}

impl Day16 {
    pub fn new() -> Day16 {
        let mut day = Day16 {
            
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day16 {
    fn get_day(&self) -> i32 {
        16
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day16.txt").map(|line| line.unwrap());
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}