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

Rather than two knots, you now must simulate a rope consisting of ten knots. One knot is still the head of the rope and moves according to the series of motions. 
Each knot further down the rope follows the knot in front of it using the same rules as before.

Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?

Input : inputs\\day9.txt
*/

use crate::prelude::*;
use std::collections::HashSet;

pub struct Day9 {
    movement_commands : Vec<Movement>
}

impl Day9 {
    pub fn new() -> Day9 {
        let mut day = Day9 {
            movement_commands : Vec::new()
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
        let lines = read_lines("inputs\\day9.txt").map(|line| line.unwrap());
        for line in lines {
            let mut line_iter = line.split_whitespace();
            let dir_char : char = line_iter.next().unwrap().chars().next().unwrap();
            let cycles : i32 = line_iter.next().unwrap().parse::<i32>().unwrap();
            let dir : Vec2 = match dir_char {
                'U' => Vec2{x : 0, y : -1},
                'D' => Vec2{x : 0, y : 1},
                'L' => Vec2{x : -1, y : 0},
                _ => Vec2{x : 1, y : 0}
            };

            self.movement_commands.push(Movement{dir, cycles});
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut visited_coords : HashSet<Vec2> = HashSet::new();
        let mut head : Vec2 = Vec2{x : 0, y : 0};
        let mut tail : Vec2 = Vec2{x : 0, y : 0};

        // lel, start also counts
        visited_coords.insert(tail);

        for command in &self.movement_commands {
            for _ in 0..command.cycles {
                move_vector(&mut head, &command.dir);
                if get_distance(&head, &tail) >= 2.0 {
                    move_towards(&mut tail, &head);
                    visited_coords.insert(tail);
                }
            }
        }

        Some(visited_coords.len().to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut visited_coords : HashSet<Vec2> = HashSet::new();
        let mut rope : [Vec2; 10] = [Vec2{x:0, y:0}; 10];

        // lel, start also counts
        visited_coords.insert(rope[9]);

        for command in &self.movement_commands {
            for _ in 0..command.cycles {
                move_vector(&mut rope[0], &command.dir);
                for i in 1..10 {
                    if get_distance(&rope[i-1], &rope[i]) >= 2.0 {
                        pull_rope_knot(&mut rope, i);
                        if i == 9 {
                            visited_coords.insert(rope[i]);
                        }
                    }
                }
            }
        }

        Some(visited_coords.len().to_string())
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Vec2 {
    x : i32,
    y : i32
}

impl Vec2 {
    fn normalized(&self) -> Vec2 {
        let x = if self.x == 0 { 0 } else { self.x / self.x.abs() };
        let y = if self.y == 0 { 0 } else { self.y / self.y.abs() };

        Vec2{x, y}
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Vec2{x, y}
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vec2{x, y}
    }
}

struct Movement {
    dir : Vec2,
    cycles : i32
}

fn move_vector(mover : &mut Vec2, dir : &Vec2) {
    mover.x = mover.x + dir.x;
    mover.y = mover.y + dir.y;
}

fn move_towards(mover : &mut Vec2, target : &Vec2) {
    let vec = *target - *mover;
    move_vector(mover, &vec.normalized());
}

// hax, cannot use move_towards in bonus scenario as I cannot pass both rope[i-1] and rope[i] due to exlusiveness of &mut
fn pull_rope_knot(rope : &mut [Vec2; 10], knot_to_pull : usize) {
    let vec = rope[knot_to_pull - 1] - rope[knot_to_pull];
    move_vector(&mut rope[knot_to_pull], &vec.normalized());
}

fn get_distance(a : &Vec2, b : &Vec2) -> f32 {
    let dist : Vec2 = *a - *b;
    ((dist.x * dist.x + dist.y * dist.y) as f32).sqrt()
}