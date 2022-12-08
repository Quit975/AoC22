///////////////////////////////////////////////////////////////////////////////////////////////////

/* DAY 7 - No Space Left On Device

///////////////////////////////////////////////////////////////////////////////////////////////////

BASE

The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). 
The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing 
the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

    cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
        cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
        cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
        cd / switches the current directory to the outermost directory, /.
    ls means list. It prints out all of the files and directories immediately contained by the current directory:
        123 abc means that the current directory contains a file named abc with size 123.
        dir xyz means that the current directory contains a directory named xyz.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. 
To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the 
files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

BONUS



Input : inputs\\day7.txt
*/

use crate::prelude::*;

pub struct Day7 {

}

impl Day7 {
    pub fn new() -> Day7 {
        let mut day = Day7 {

        };

        day.load_data();
        day
    }
}

impl DailyPuzzle for Day7 {
    fn get_day(&self) -> i32 {
        7
    }

    fn load_data(&mut self) {
        //read_lines("inputs\\day7.txt").next().unwrap().unwrap();
    }

    fn get_base_answer(&self) -> Option<String> {
        

        None
    }

    fn get_bonus_answer(&self) -> Option<String> {
        None
    }
}