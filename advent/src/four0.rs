#[derive(Debug)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left
}

#[derive(Debug)]
struct DirectionMatch {
    direction: Direction,
    loc: usize
}


#[derive(Debug)]
struct Size {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct WordSearch {
    // row x column
    size: Size,
    pointer: usize,
    contents: Vec<char>
}

impl WordSearch {

    fn new(input: String) -> Self {
        let mut lines: Vec<&str> = input.split('\n').collect();
        // An extra empty string (`""`) is added to the lines when splitting 
        // the input, we remove this with a `pop()`
        lines.pop();
        // println!("{:?}", lines);

        WordSearch {
            size: Size {
                row: lines.len(),
                col: lines[0].len(),
            },
            // We know that all WordSearches must be grids equal in its length
            // of columns and rows (i.e: 10x10, 140x140, or 420x420) -- as a 
            // result we can simply use the measurement of one length of the 
            // grid as the value for the pointer, which is the base value for
            // measuring, in sequence, the number of characters between some
            // character in the grid, and the character directly above or 
            // below it
            pointer: lines.len(),
            contents: lines.iter()
                        .flat_map(|s| s.chars())
                        .collect()
        }
    }


    fn pprint(&self) {
        println!("Wordsearch");
        println!("-----------------------------------------------");

        let mut row = String::from("");
        for i in 0..self.size.row {
            for j in 0..self.size.col {
                row.push(self.contents[i * 10 + j].clone());

                if j < self.size.col - 1 {
                    row.push(' ');
                }
            } 

            println!("{}", row);
            row = String::from("");
        }

        // println!("{:?}", self.contents);
        println!("{:?}", self.size);
    }

    
    fn find(&self, value: String) {
        let word: Vec<char> = value.chars().collect();

        let initial_positions = self.find_first_char(&word[0]);
        // println!("Initial positions: {:?}", initial_positions);

        for pos in initial_positions {
            let positions = self.find_second_char(&pos, &word[1]);
            println!("{:?} Initial matches: {:?}", pos, positions);

            /*
            for i in 2..word.len() {
                for pos in secondary_position {
                    self.find_with_direction(pos, word[i]);
                }
            }
            */
        }
    }

    fn find_first_char(&self, c: &char) -> Vec<usize> {
        let mut loc: usize = 0;
        let mut positions: Vec<usize> = vec![];
        for character in &self.contents {
            if character == c {
                // println!("Found {:?} at {:?}", c, loc);
                positions.push(loc);
            }

            loc += 1;
        }

        return positions;
    }

    fn find_second_char(&self, loc: &usize, c: &char) -> Vec<DirectionMatch> {
        // println!("At {:?}", loc);
        // println!("Test: up-left-diag char {:?}", &self.contents[loc - (self.pointer + 1)]);
        let mut positions: Vec<DirectionMatch> = vec![];

        let mut up_left: usize = 0;
        if let Some(res) = loc.checked_sub(self.pointer + 1) {
            up_left = res;
            if c == &self.contents[up_left] {
                 positions.push(DirectionMatch {
                    direction: Direction::UpLeft,
                    loc: up_left 
                }) 
            }
        }

        let mut up: usize = 0;
        if let Some(res) = loc.checked_sub(self.pointer) {
            up = res;
            if c == &self.contents[up] {
                 positions.push(DirectionMatch {
                    direction: Direction::Up,
                    loc: up 
                }) 
            }
        }

        let mut up_right: usize = 0;
        if let Some(res) = loc.checked_sub(self.pointer - 1) {
            up_right = res;
            if c == &self.contents[up_right] {
                 positions.push(DirectionMatch {
                    direction: Direction::UpRight,
                    loc: up_right
                }) 
            }
        }

        let mut right: usize = 0;
        if let Some(res) = loc.checked_add(1) {
            right = res;
            if let Some(comp) = &self.contents.get(right) {
                if comp == &c {
                    positions.push(DirectionMatch {
                        direction: Direction::Right,
                        loc: right 
                    }) 
                }
            }
        }

        let mut down_right: usize = 0;
        if let Some(res) = loc.checked_add(self.pointer + 1) {
            down_right = res;
            if let Some(comp) = &self.contents.get(down_right) {
                if comp == &c {
                    positions.push(DirectionMatch {
                        direction: Direction::DownRight,
                        loc: down_right 
                    }) 
                }
            }
        }

        let mut down: usize = 0;
        if let Some(res) = loc.checked_add(self.pointer) {
            down = res;
            if let Some(comp) = &self.contents.get(down) {
                if comp == &c {
                    positions.push(DirectionMatch {
                        direction: Direction::Down,
                        loc: down
                    }) 
                }
            }
        }

        let mut down_left: usize = 0;
        if let Some(res) = loc.checked_add(self.pointer - 1) {
            down_left = res;
            if let Some(comp) = &self.contents.get(down_left) {
                if comp == &c {
                    positions.push(DirectionMatch {
                        direction: Direction::DownLeft,
                        loc: down_left
                    }) 
                }
            }
        }

        let mut left: usize = 0;
        if let Some(res) = loc.checked_sub(self.pointer) {
            left = res;
            if let Some(comp) = &self.contents.get(left) {
                if comp == &c {
                    positions.push(DirectionMatch {
                        direction: Direction::Left,
                        loc: left
                    }) 
                }
            }
        }

        return positions;
    }

}


