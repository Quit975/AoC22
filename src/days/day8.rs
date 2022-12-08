///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 8 - Treetop Tree House

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. 
The Elves explain that a previous expedition planted these trees as a reforestation effort. 
Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. 
To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. 
Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

Consider your map; how many trees are visible from outside the grid?

BONUS

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: 
they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or 
at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, 
at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, 
so they wouldn't be able to see higher than the tree house anyway.

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. 

Consider each tree on your map. What is the highest scenic score possible for any tree?

Input : inputs\\day8.txt
*/

use crate::prelude::*;

pub struct Day8 {
    trees : Vec<Tree>,
    total_width : usize,
    total_height : usize
}

impl Day8 {
    pub fn new() -> Day8 {
        let mut day = Day8 {
            trees : Vec::new(),
            total_width : 0,
            total_height : 0
        };

        day.load_data();
        day
    }

    fn get_height_at_idx(&self, x : usize, y : usize) -> u32 {
        self.trees.as_slice().iter().find(|tree| tree.x == x && tree.y == y).unwrap().height
    }
}

impl DailyPuzzle for Day8 {
    fn get_day(&self) -> i32 {
        8
    }

    fn load_data(&mut self) {
        let lines = read_lines("inputs\\day8.txt").map(|line| line.unwrap());
        let mut row : usize = 0;

        for line in lines {
            let mut column : usize = 0;
            let chars = line.chars();
            for char in chars {
                self.trees.push(Tree{x: column, y: row, height : char.to_digit(10).unwrap()});
                column += 1;
            }
            self.total_width = column - 1;
            row += 1;
        }
        self.total_height = row - 1;
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut count : i32 = 0;
        for tree in &self.trees {
            // one of the outer trees
            if tree.x == 0 || tree.x == self.total_width || tree.y == 0 || tree.y == self.total_height {
                count += 1;
                continue;
            }

            if tree.height == 0 {
                continue;
            }

            // one of the inner trees
            let mut trees_left = self.trees.iter().filter(|other| other.x < tree.x && other.y == tree.y);
            let mut trees_right = self.trees.iter().filter(|other| other.x > tree.x && other.y == tree.y);
            let mut trees_up = self.trees.iter().filter(|other| other.y < tree.y && other.x == tree.x);
            let mut trees_down = self.trees.iter().filter(|other| other.y > tree.y && other.x == tree.x);

            let highest_left = !(trees_left.any(|other| other.height >= tree.height));
            let highest_right = !(trees_right.any(|other| other.height >= tree.height));
            let highest_up = !(trees_up.any(|other| other.height >= tree.height));
            let highest_down = !(trees_down.any(|other| other.height >= tree.height));

            if highest_left || highest_right || highest_up || highest_down {
                count += 1;
            }
        }

        Some(count.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut highest_score : i32 = 0;
        for tree in &self.trees {
            // one of the outer trees, score will be 0
            // also will go on a hunch and discard trees lower than average
            if tree.x == 0 || tree.x == self.total_width || tree.y == 0 || tree.y == self.total_height || tree.height <= 5 {
                continue;
            }

            let mut left_score : i32 = 0;
            for i in (0..=(tree.x - 1)).rev() {
                left_score += 1;
                if self.get_height_at_idx(i, tree.y) >= tree.height { 
                    break;
                }
            }

            let mut right_score : i32 = 0;
            for i in (tree.x + 1)..=self.total_width {
                right_score += 1;
                if self.get_height_at_idx(i, tree.y) >= tree.height {
                    break;
                }
            }

            let mut up_score : i32 = 0;
            for i in (0..=(tree.y - 1)).rev() {
                up_score += 1;
                if self.get_height_at_idx(tree.x, i) >= tree.height {
                    break;
                }
            }

            let mut down_score : i32 = 0;
            for i in (tree.y + 1)..=self.total_height {
                down_score += 1;
                if self.get_height_at_idx(tree.x, i) >= tree.height {
                    break;
                }
            }

            let total_view_score = left_score * right_score * up_score * down_score;
            if total_view_score > highest_score {
                highest_score = total_view_score;
            }
        }

        Some(highest_score.to_string())
    }
}

struct Tree {
    x : usize,
    y : usize,
    height : u32
}