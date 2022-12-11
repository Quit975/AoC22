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

It seems like the X register controls the horizontal position of a sprite. 
Specifically, the sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that sprite. 
(In this system, there is no such thing as "vertical position": if the sprite's horizontal position puts its pixels where the CRT is currently drawing, 
then those pixels will be drawn.)

You count the pixels on the CRT: 40 wide and 6 high. This CRT screen draws the top row of pixels left-to-right, then the row below that, and so on. 
The left-most pixel in each row is in position 0, and the right-most pixel in each row is in position 39.

Like the CPU, the CRT is tied closely to the clock circuit: the CRT draws a single pixel during each cycle. Representing each pixel of the screen as a #, 
here are the cycles during which the first and last pixel in each row are drawn:

So, by carefully timing the CPU instructions and the CRT drawing operations, you should be able to determine whether the sprite is visible the instant each pixel is drawn. 
If the sprite is positioned such that one of its three pixels is the pixel currently being drawn, the screen produces a lit pixel (#); otherwise, the screen leaves the pixel dark (.)

Render the image given by your program. What eight capital letters appear on your CRT?

Input : inputs\\day10.txt
*/

use crate::prelude::*;

pub struct Day10 {
    op_list : Vec<Op>
}

impl Day10 {
    pub fn new() -> Day10 {
        let mut day = Day10 {
            op_list : Vec::new()
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
        let lines = read_lines("inputs\\day10.txt").map(|line| line.unwrap());
        for line in lines {
            if line == "noop" {
                self.op_list.push(Op::Noop);
            }
            else {
                let value = line.split_whitespace().skip(1).next().unwrap().parse::<i32>().unwrap();
                self.op_list.push(Op::Add(value));
            }
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut cycle : i32 = 0;
        let mut total_signal_strength : i32 = 0;
        let mut register : i32 = 1;

        for op in &self.op_list {
            match op {
                Op::Add(value) => {
                    total_signal_strength += cycle_tick(&mut cycle, &mut register, None);
                    total_signal_strength += cycle_tick(&mut cycle, &mut register, Some(*value));
                }
                Op::Noop => total_signal_strength += cycle_tick(&mut cycle, &mut register, None)
            }
        }

        Some(total_signal_strength.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut cycle : i32 = 1;
        let mut pixels_drawn : String = String::from("\n");
        let mut register : i32 = 1;

        for op in &self.op_list {
            match op {
                Op::Add(value) => {
                    pixels_drawn += draw_cycle(&mut cycle, &mut register, None);
                    pixels_drawn += draw_cycle(&mut cycle, &mut register, Some(*value));
                }
                Op::Noop => pixels_drawn += draw_cycle(&mut cycle, &mut register, None)
            }
        }

        Some(pixels_drawn)
    }
}

fn cycle_tick(cycle_counter : &mut i32, register : &mut i32, val : Option<i32>) -> i32 {
    let mut signal : i32 = 0;
    *cycle_counter += 1;
    
    if (*cycle_counter - 20) % 40 == 0 {
        signal = *cycle_counter * *register;
    }
    
    if let Some(num) = val {
        *register += num;
    }
    
    signal
}

fn draw_cycle(cycle_counter : &mut i32, register : &mut i32, val : Option<i32>) -> &'static str {
    let mut newline : bool = false;
    if (*cycle_counter) % 40 == 0 {
        newline = true;
    }

    // works? works. So shaddap xD
    let mut pos = *cycle_counter;
    if pos > 200 {
        pos -= 200;
    } else if pos > 160 {
        pos -= 160;
    } else if pos > 120 {
        pos -= 120;
    } else if pos > 80 {
        pos -= 80;
    } else if pos > 40 {
        pos -= 40;
    }

    let should_draw : bool = pos - 1 == *register || pos - 1 == *register - 1 || pos - 1 == *register + 1;
    
    if let Some(num) = val {
        *register += num;
    }
    *cycle_counter += 1;

    match should_draw {
        true => match newline {
            true => "#\n",
            false => "#",
        },
        false => match newline {
            true => ".\n",
            false => ".",
        },
    }
}


enum Op {
    Add(i32),
    Noop
}