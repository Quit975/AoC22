///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 15 - Beacon Exclusion Zone

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. 
You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.

The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. 
You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. 
Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; 
however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. 
(There is never a tie where two beacons are the same distance to a sensor.)

It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input).

None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. 
For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?


BONUS



Input : inputs\\day15.txt
*/

use crate::prelude::*;

pub struct Day15 {
    
}

impl Day15 {
    pub fn new() -> Day15 {
        let mut day = Day15 {
            
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day15 {
    fn get_day(&self) -> i32 {
        15
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day15.txt").map(|line| line.unwrap());
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}