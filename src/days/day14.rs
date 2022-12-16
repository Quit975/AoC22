///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 14 - Regolith Reservoir

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. 
However, you do notice a little path that leads behind the waterfall.

Correction: the distress signal leads you behind a giant waterfall! There seems to be a large cave system here, and the signal definitely leads further inside.

As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand begins pouring into the cave! 
If you don't quickly figure out where the sand is going, you could quickly become trapped!

Fortunately, your familiarity with analyzing the path of falling material will come in handy here. 
You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, 
where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. 
After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point.

Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.

A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), 
the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, 
the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, 
at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, 
the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.

Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

BONUS



Input : inputs\\day14.txt
*/

use crate::prelude::*;

pub struct Day14 {
    
}

impl Day14 {
    pub fn new() -> Day14 {
        let mut day = Day14 {
            
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day14 {
    fn get_day(&self) -> i32 {
        14
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day14.txt").map(|line| line.unwrap());
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}