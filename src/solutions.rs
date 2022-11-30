use crate::prelude::*;

/* DAY 1/2021 - TEST BASE

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