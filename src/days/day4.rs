///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 4 - Camp Cleanup

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. 
Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. 
To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. 
In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, 
so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?

BONUS

It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In how many assignment pairs do the ranges overlap?

Input : inputs\\day4.txt
*/

use crate::prelude::*;

pub struct Day4 {
    assignments : Vec<Assignment>
}

impl Day4 {
    pub fn new() -> Day4 {
        let mut day : Day4 = Day4 {
            assignments : Vec::new()
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day4 {
    fn get_day(&self) -> i32 {
        4
    }

    fn load_data(&mut self) {
        let lines  = read_lines("inputs\\day4.txt").map(|line| line.unwrap());
        for line in lines {
            let mut split = line.split(',');
            self.assignments.push(Assignment::new(split.next().unwrap()));
            self.assignments.push(Assignment::new(split.next().unwrap()));
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut sum : i32 = 0;
        let mut iter = self.assignments.iter().zip(self.assignments.iter().skip(1)).step_by(2);
        while let Some(pair) = iter.next() {
            if is_any_contained(pair.0, pair.1) {
                sum += 1;
            }
        }

        Some(sum.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut sum : i32 = 0;
        let mut iter = self.assignments.iter().zip(self.assignments.iter().skip(1)).step_by(2);
        while let Some(pair) = iter.next() {
            if do_assignments_overlap(pair.0, pair.1) {
                sum += 1;
            }
        }

        Some(sum.to_string())
    }
}

struct Assignment {
    lower_id : i32,
    higher_id : i32
}

impl Assignment {
    fn new(info : &str) -> Assignment {
        let mut split = info.split('-');

        Assignment { 
            lower_id: split.next().unwrap().parse().unwrap(),
            higher_id: split.next().unwrap().parse().unwrap() 
        }
    }
}

fn is_any_contained(a : &Assignment, b : &Assignment) -> bool {
    (a.lower_id >= b.lower_id && a.higher_id <= b.higher_id)
    ||
    (b.lower_id >= a.lower_id && b.higher_id <= a.higher_id)
}

fn do_assignments_overlap(a : &Assignment, b : &Assignment) -> bool {
    use std::cmp::{min, max};

    max(a.lower_id, b.lower_id) - min(a.higher_id, b.higher_id) <= 0
}