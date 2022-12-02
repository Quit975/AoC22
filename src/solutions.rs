use crate::prelude::*;

///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 1 - CALORIES

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. 
Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

Input - inputs\\day1.txt

*/

pub fn day1_base() -> i32 {
    let lines  = read_lines("inputs\\day1.txt");

    let mut highest_calories : i32 = 0;
    let mut current_sum : i32 = 0;

    for line in lines {
        if let Ok(input) = line {
            if let Ok(calories_num) = input.parse::<i32>() {
                current_sum += calories_num;
            }
            else {
                if current_sum > highest_calories {
                    highest_calories = current_sum;
                }
                current_sum = 0;
            }
        }
    }
    
    highest_calories
}

/*
BONUS

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

pub fn day1_bonus() -> i32 {
    let lines  = read_lines("inputs\\day1.txt");
    
    let mut top_three_calories : [i32; 3] = [0; 3];
    let mut current_sum : i32 = 0;

    for line in lines {
        if let Ok(input) = line {
            if let Ok(calories_num) = input.parse::<i32>() {
                current_sum += calories_num;
            }
            else {
                if let Some(lowest) = top_three_calories.iter_mut().min() {
                    if *lowest < current_sum {
                        *lowest = current_sum;
                    }
                }

                current_sum = 0;
            }
        }
    }

    top_three_calories.iter().sum()
}



///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 2 - ROCK PAPER SCISSORS

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure
to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. 
The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. 
Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. 
The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score 
for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).


What would your total score be if everything goes exactly according to your strategy guide?

Input : inputs\\day2.txt
*/


pub fn day2_base() -> i32 {
    let mut total_score : i32 = 0;
    let mut rounds : Vec<(char, char)> = Vec::new();

    let lines  = read_lines("inputs\\day2.txt");
    for line in lines {
        if let Ok(input) = line {
            let mut chars = input.chars().filter(|&c| c != ' ');
            rounds.push((chars.next().unwrap(), chars.next().unwrap()));
        }
    }

    rounds.iter().for_each(|round| {
        let round_score : i32 = match round.0 {
            'A' => match round.1 {
                'X' => 4,//rock rock(1) draw(3),
                'Y' => 8,//rock paper(2) win(6),
                 _  => 3,//rock scissor(3) loss(0)
            },
            'B' => match round.1 {
                'X' => 1,//paper rock(1) loss(0),
                'Y' => 5,//paper paper(2) draw(3),
                 _  => 9,//paper scissor(3) win(6)
            },
            _ => match round.1 {
                'X' => 7,//scissor rock(1) win(6),
                'Y' => 2,//scissor paper(2) loss(0),
                 _  => 6,//scissor scissor(3) draw(3)
            },
        };
        total_score += round_score;
    });

    total_score
}

/*
BONUS

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: 
X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

Following the Elf's instructions for the second column, what would your total score be if everything goes 
exactly according to your strategy guide?

*/

pub fn day2_bonus() -> i32 {
    let mut total_score : i32 = 0;
    let mut rounds : Vec<(char, char)> = Vec::new();

    let lines  = read_lines("inputs\\day2.txt");
    for line in lines {
        if let Ok(input) = line {
            let mut chars = input.chars().filter(|&c| c != ' ');
            rounds.push((chars.next().unwrap(), chars.next().unwrap()));
        }
    }

    rounds.iter().for_each(|round| {
        let round_score : i32 = match round.0 {
            'A' => match round.1 {
                'X' => 3, //loss(0) rock scissors(3)
                'Y' => 4, //draw(3) rock rock(1),
                 _  => 8, //win(6) rock paper(2)
            },
            'B' => match round.1 {
                'X' => 1, //loss(0) paper rock(1)
                'Y' => 5, //draw(3) paper paper(2)
                 _  => 9, //win(6) paper scissors(3)
            },
            _ => match round.1 {
                'X' => 2, //loss(0) scissor paper(2)
                'Y' => 6, //draw(3) scissor scissors(3)
                 _  => 7, //win(6)  scissor rock(1)
            },
        };
        total_score += round_score;
    });

    total_score
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 1/2021 - TEST

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

To do this, count the number of times a depth measurement increases from the previous measurement. 
(There is no measurement before the first measurement.) 

input file - inputs\\test.txt
*/

pub fn test_base() -> i32 {
    let lines  = read_lines("inputs\\test.txt");

    let mut increment_count : i32 = 0;
    let mut prev_depth : i32 = -1000;
    
    for line in lines {
        if let Ok(input) = line {
            let depth: i32 = input.parse().unwrap();

            if prev_depth == -1000 {
                prev_depth = depth;
                continue;
            }

            if depth > prev_depth {
                increment_count += 1;
            }

            prev_depth = depth;
        }
    }

    increment_count
}

/* DAY 1/2021 - TEST BONUS

Instead, consider sums of a three-measurement sliding window. Again considering the above example:

Start by comparing the first and second three-measurement windows. 
The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607. 
The second window is marked B (200, 208, 210); its sum is 618. 
The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.

Your goal now is to count the number of times the sum of measurements in this sliding window increases from the previous sum. 
So, compare A with B, then compare B with C, then C with D, and so on. 
Stop when there aren't enough measurements left to create a new three-measurement sum.

Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
*/

pub fn test_bonus() -> i32 {
    let depths : Vec<i32> = read_lines("inputs\\test.txt")
                                .map(|line| line.unwrap().parse::<i32>().unwrap())
                                .collect();

    let mut increment_count : i32 = 0;
    let mut last_sum : i32 = -1000;
    let mut idx : usize = 0;

    loop {
        if idx + 2 > depths.len() - 1 {
            break;
        }

        let sum = depths[idx] + depths[idx + 1] + depths[idx + 2];

        if last_sum == -1000 {
            last_sum = sum;
            idx += 1;
            continue;
        }

        if sum > last_sum {
            increment_count += 1;
        }

        last_sum = sum;
        idx += 1;
    } 

    increment_count
}

