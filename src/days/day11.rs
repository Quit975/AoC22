///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 11 - Monkey in the Middle

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

As you finally start making your way upriver, you realize your pack is much lighter than you remember. 
Just then, one of the items from your pack goes flying overhead. Monkeys are playing Keep Away with your missing things!

To get your stuff back, you need to be able to predict where the monkeys will throw your items. 
After some careful observation, you realize the monkeys operate based on how worried you are about each item.

You take some notes (your puzzle input) on the items each monkey currently has, how worried you are about those items, and how the monkey makes decisions based on your worry level.

Each monkey has several attributes:

    Starting items lists your worry level for each item the monkey is currently holding in the order they will be inspected.
    Operation shows how your worry level changes as that monkey inspects an item. (An operation like new = old * 5 means that your worry level after the monkey inspected the item is five times whatever your worry level was before inspection.)
    Test shows how the monkey uses your worry level to decide where to throw an item next.
        If true shows what happens with an item if the Test was true.
        If false shows what happens with an item if the Test was false.

After each monkey inspects an item but before it tests your worry level, your relief that the monkey's inspection didn't damage the item causes your 
worry level to be divided by three and rounded down to the nearest integer.

The monkeys take turns inspecting and throwing items. On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed. 
Monkey 0 goes first, then monkey 1, and so on until each monkey has had one turn. The process of each monkey taking a single turn is called a round.

When a monkey throws an item to another monkey, the item goes on the end of the recipient monkey's list. 
A monkey that starts a round with no items could end up inspecting and throwing many items by the time its turn comes around. 
If a monkey is holding no items at the start of its turn, its turn ends.

Chasing all of the monkeys at once is impossible; you're going to have to focus on the two most active monkeys if you want any hope of getting your stuff back. 
Count the total number of times each monkey inspects items over 20 rounds

In this example, the two most active monkeys inspected items 101 and 105 times. The level of monkey business in this situation can be found by multiplying these together: 10605.

Figure out which monkeys to chase by counting how many items they inspect over 20 rounds. What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?

BONUS

You're worried you might not ever get your items back. So worried, in fact, that your relief that a monkey's inspection didn't damage an item no longer causes 
your worry level to be divided by three.

Unfortunately, that relief was all that was keeping your worry levels from reaching ridiculous levels. You'll need to find another way to keep your worry levels manageable.

At this rate, you might be putting up with these monkeys for a very long time - possibly 10000 rounds!

After 10000 rounds, the two most active monkeys inspected items 52166 and 52013 times. Multiplying these together, the level of monkey business in this situation is now 2713310158.

Worry levels are no longer divided by three after each item is inspected; you'll need to find another way to keep your worry levels manageable. 
Starting again from the initial state in your puzzle input, what is the level of monkey business after 10000 rounds?

Input : inputs\\day11.txt
*/

use crate::prelude::*;
use std::collections::VecDeque;

pub struct Day11 {
    monkes : Vec<Monke>
}

impl Day11 {
    pub fn new() -> Day11 {
        let mut day = Day11 {
            monkes : Vec::new()
        };

        day.load_data();
        day
    }
}

impl<'a> DailyPuzzle for Day11 {
    fn get_day(&self) -> i32 {
        11
    }

    fn load_data(&mut self) {
        self.monkes.push(Monke::new(
            "66, 79",
            |item| item.worry_level = item.worry_level * 11,
            7,
            6,
            7
            )
        );

        self.monkes.push(Monke::new(
            "84, 94, 94, 81, 98, 75",
            |item| item.worry_level = item.worry_level * 17,
            13,
            5,
            2
            )
        );

        self.monkes.push(Monke::new(
            "85, 79, 59, 64, 79, 95, 67",
            |item| item.worry_level = item.worry_level + 8,
            5,
            4,
            5
            )
        );

        self.monkes.push(Monke::new(
            "70",
            |item| item.worry_level = item.worry_level + 3,
            19,
            6,
            0
            )
        );

        self.monkes.push(Monke::new(
            "57, 69, 78, 78",
            |item| item.worry_level = item.worry_level + 4,
            2,
            0,
            3
            )
        );
        
        self.monkes.push(Monke::new(
            "65, 92, 60, 74, 72",
            |item| item.worry_level = item.worry_level + 7,
            11,
            3,
            4
            )
        );

        self.monkes.push(Monke::new(
            "77, 91, 91",
            |item| item.worry_level = item.worry_level * item.worry_level,
            17,
            1,
            7
            )
        );

        self.monkes.push(Monke::new(
            "76, 58, 57, 55, 67, 77, 54, 99",
            |item| item.worry_level = item.worry_level + 6,
            3,
            2,
            1
            )
        );
    }

    fn get_base_answer(&self) -> Option<String> {
        let mut monkes : Vec<Monke> = self.monkes.clone();

        for _round in 0..20 {
            for monkey_idx in 0..8 {
                while let Some(mut item) = monkes[monkey_idx].get_next_item() {
                    item.reduce_worry_level();
                    let idx = monkes[monkey_idx].get_monke_for_item(&item);
                    monkes[idx].give_item(item);
                }
            }
        }

        monkes.sort_unstable_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
        let score : u64 = monkes[0].inspect_count * monkes[1].inspect_count;
        
        Some(score.to_string())
    }

    fn get_bonus_answer(&self) -> Option<String> {
        // let mut monkes : Vec<Monke> = self.monkes.clone();

        // for _round in 0..10000 {
        //     for monkey_idx in 0..8 {
        //         while let Some(mut item) = monkes[monkey_idx].get_next_item() {
        //             item.adjust_worry_level();
        //             let idx = monkes[monkey_idx].get_monke_for_item(&item);
        //             monkes[idx].give_item(item);
        //         }
        //     }
        // }

        // monkes.sort_unstable_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
        // let score : u64 = monkes[0].inspect_count * monkes[1].inspect_count;
        
        // Some(score.to_string())

        None
    }
}

#[derive(Clone)]
struct Monke {
    items : VecDeque<Item>,
    worry_function : fn(&mut Item),
    test_division : u64,
    true_monke : usize,
    false_monke : usize,
    inspect_count : u64
}

impl Monke {
    fn new(
        startinig_items_string : &str,
        worry_closure : fn(&mut Item),
        division : u64,
        true_monke : usize,
        false_monke : usize
    ) -> Monke {
        let mut monke = Monke {
            items : VecDeque::new(),
            worry_function : worry_closure,
            test_division : division,
            true_monke,
            false_monke,
            inspect_count : 0
        };

        let items = startinig_items_string.split(&[' ', ',']);
        for item in items {
            if let Ok(item_nr) = item.parse::<u64>() {
                monke.items.push_back(Item{worry_level : item_nr});
            }
        }

        monke
    }

    fn get_next_item(&mut self) -> Option<Item> {
        let mut item = self.items.pop_front();
        match item {
            Some(ref mut it) => {
                (self.worry_function)(it);
                self.inspect_count += 1;
            }
            None => ()
        };
        item
    }

    fn get_monke_for_item(&self, item : &Item) -> usize {
        if item.worry_level % self.test_division == 0 {
            self.true_monke
        } else {
            self.false_monke
        }
    }

    fn give_item(&mut self, item : Item) {
        self.items.push_back(item);
    }
}

#[derive(Clone)]
struct Item {
    worry_level : u64
}

impl Item {
    fn reduce_worry_level(&mut self) {
        self.worry_level /= 3;
    }

    fn adjust_worry_level(&mut self) {
    }
}