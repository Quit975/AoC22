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

BONUS

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: 
X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

Following the Elf's instructions for the second column, what would your total score be if everything goes 
exactly according to your strategy guide?s

Input : inputs\\day2.txt
*/

use crate::prelude::*;

pub struct Day2 {
    rounds : Vec<(char, char)>
}

impl Day2 {
    pub fn new() -> Day2 {
        let mut day = Day2 {
            rounds : Vec::new()
        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day2 {
    fn get_day(&self) -> i32 {
        2
    }

    fn load_data(&mut self) {
        let lines  = read_lines("inputs\\day2.txt").map(|line| line.unwrap());
        for line in lines {
            let mut chars = line.chars().filter(|&c| c != ' ');
            self.rounds.push((chars.next().unwrap(), chars.next().unwrap()));
        }
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut total_score : i32 = 0;

        self.rounds.iter().for_each(|round| {
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

        Some(total_score.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        let mut total_score : i32 = 0;

        self.rounds.iter().for_each(|round| {
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

        Some(total_score.to_string())
    }
}