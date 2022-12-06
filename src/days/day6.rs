///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 5 - Tuning Trouble

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. 
In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position 
where the four most recently received characters were all different. Specifically, it needs to report the number of characters from 
the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. 
The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. 
Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, 
which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is 
complete after 7 characters have been processed.

How many characters need to be processed before the first start-of-packet marker is detected?

BONUS



Input : inputs\\day6.txt
*/

use std::collections::VecDeque;

use crate::prelude::*;

pub struct Day6 {
    input : String
}

impl Day6 {
    pub fn new() -> Day6 {
        let mut day = Day6 {
            input : String::new()
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day6 {
    fn get_day(&self) -> i32 {
        6
    }

    fn load_data(&mut self) {
        self.input  = read_lines("inputs\\day6.txt").next().unwrap().unwrap();
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut count : i32 = 0;
        let letters = self.input.chars();
        let mut deque : VecDeque<char> = VecDeque::new();

        for letter in letters {
            if deque.len() >= 4 {
                deque.pop_front();
            }
            deque.push_back(letter);
            count += 1;

            if count >= 4 {
                let set : std::collections::HashSet<&char> = deque.iter().collect();
                if set.len() == deque.len() {
                    break;
                }
            }
        }

        Some(count.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut count : i32 = 0;
        let letters = self.input.chars();
        let mut deque : VecDeque<char> = VecDeque::new();

        for letter in letters {
            if deque.len() >= 14 {
                deque.pop_front();
            }
            deque.push_back(letter);
            count += 1;

            if count >= 14 {
                let set : std::collections::HashSet<&char> = deque.iter().collect();
                if set.len() == deque.len() {
                    break;
                }
            }
        }

        Some(count.to_string())
    }
}