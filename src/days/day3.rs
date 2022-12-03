///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 3 - Rucksack Reorganization

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. 
Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. 
The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. 
Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, 
so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

BONUS

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. 
For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. 
That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the 
Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks 
so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by 
finding the one item type that is common between all three Elves in each group.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?

Input : inputs\\day3.txt
*/

use crate::prelude::*;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

pub struct Day3 {
    priority_map : HashMap<char, u8>,
    rucksack_vector : Vec<String>
}

impl Day3 {
    pub fn new() -> Day3 {
        let mut day = Day3{
            priority_map : HashMap::new(),
            rucksack_vector : Vec::new()
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day3 {
    fn get_day(&self) -> i32 {
        3
    }

    fn load_data(&mut self) {
        for (i, c) in ('a'..='z').enumerate() {
            self.priority_map.insert(c, i as u8 + 1);
        }

        for (i, c) in ('A'..='Z').enumerate() {
            self.priority_map.insert(c, i as u8 + 27);
        }

        let lines  = read_lines("inputs\\day3.txt").map(|line| line.unwrap());
        for line in lines {
            self.rucksack_vector.push(line);
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut priority_sum : i32 = 0;
        for rucksack in &self.rucksack_vector {

            let mut item_set : HashSet<u8> = HashSet::new();

            let container = rucksack.split_at(rucksack.len()/2);

            container.0.chars().for_each(|c| {
                item_set.insert(self.priority_map[&c]);
            });

            let priority : u8 = self.priority_map[
                &container.1
                .chars()
                .find(|c| item_set.contains(&self.priority_map[&c]))
                .unwrap()];

            priority_sum += priority as i32;
        }

        Some(priority_sum.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut priority_sum : i32 = 0;

        let mut idx : usize = 0;
        loop {
            if self.rucksack_vector.len() <= idx + 2 {
                break;
            }
            let mut set1 : HashSet<u8> = HashSet::new();
            let mut set2 : HashSet<u8> = HashSet::new();
            let mut set3 : HashSet<u8> = HashSet::new();

            for c in self.rucksack_vector[idx].chars() {
                set1.insert(self.priority_map[&c]);
            }

            for c in self.rucksack_vector[idx + 1].chars() {
                set2.insert(self.priority_map[&c]);
            }

            for c in self.rucksack_vector[idx + 2].chars() {
                set3.insert(self.priority_map[&c]);
            }

            let intersection1 = set1.intersection(&set2).collect::<HashSet<&u8>>();
            let intersection2 = set1.intersection(&set3).collect::<HashSet<&u8>>();
            
            let mut final_intersection = intersection1.intersection(&intersection2);

            priority_sum += **(final_intersection.next().unwrap()) as i32;

            idx += 3;
        }

        Some(priority_sum.to_string())
    }
}