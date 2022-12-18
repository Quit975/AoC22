///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 12 - Hill Climbing Algorithm

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from above broken into a grid; 
the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E). 
Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. 
To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, 
if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower 
than the elevation of your current square.)

What is the fewest steps required to move from your current position to the location that should get the best signal?

BONUS



Input : inputs\\day12.txt
*/

use crate::prelude::*;

pub struct Day12 {
    map : Vec<Cell>,
    start : Position,
    goal : Position,
    map_width : usize,
    map_height : usize
}

impl Day12 {
    pub fn new() -> Day12 {
        let mut day = Day12 {
            map : Vec::new(),
            start : Position{x : 0, y : 0},
            goal : Position{x : 0, y : 0},
            map_width : 0,
            map_height : 0
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day12 {
    fn get_day(&self) -> i32 {
        12
    }

    fn load_data(&mut self) {
        let mut lines = read_lines("inputs\\day12.txt").map(|line| line.unwrap()).enumerate();
        const ASCII_OFFSET : i32 = 97;

        while let Some((row, line)) = lines.next() {
            let mut cells = line.chars().enumerate();
            while let Some((col, cell)) = cells.next() {
                if cell == 'S' {
                    self.start = Position{x : col, y : row};
                }
                else if cell == 'Z' {
                    self.goal = Position{x : col, y : row};
                }
                self.map.push(Cell::new(cell as i32 - ASCII_OFFSET, col, row));
                self.map_width = col + 1;
            }
            self.map_height = row + 1;
        }

        for idx in 0..self.map.len() {
            let pos : Position = self.map[idx].pos;
            let height : i32 = self.map[idx].height;
            let mut cost_up : i32 = 99;
            let mut cost_down : i32 = 99;
            let mut cost_left : i32 = 99;
            let mut cost_right : i32 = 99;

            if pos.y > 0 {
                cost_up = (self.map[idx - self.map_width].height - height) as i32;
            }

            if pos.y < self.map_height - 1 {
                cost_down = (self.map[idx + self.map_width].height - height) as i32;
            }

            if pos.x > 0 {
                cost_left = (self.map[idx - 1].height - height) as i32;
            }

            if pos.x < self.map_width - 1 {
                cost_right = (self.map[idx + 1].height - height) as i32;
            }
            
            let mut cell = &mut self.map[idx];
            cell.cost_up = cost_up;
            cell.cost_down = cost_down;
            cell.cost_left = cost_left;
            cell.cost_right = cost_right;
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}

#[derive(Clone, Copy)]
struct Position {
    x : usize,
    y : usize
}

struct Cell {
    height : i32,
    pos : Position,
    cost_up : i32,
    cost_down : i32,
    cost_left : i32,
    cost_right : i32
}

impl Cell {
    fn new(height : i32, x : usize, y : usize) -> Cell {
        Cell {
            height,
            pos : Position{x, y},
            cost_up : 99,
            cost_down : 99,
            cost_left : 99,
            cost_right : 99,
        }
    }
}