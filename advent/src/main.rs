use std::error::Error;
use std::fs;
use std::fmt;


struct WordSearch {
    contents: Vec<Vec<char>>
}

impl WordSearch {
    fn new() -> Self {
        WordSearch {
            contents: vec![]
        }
    }


    fn pprint(&self) {
        for row in &self.contents {
            println!("{:?}", row);
        }
    }


    fn find_x(&self) {
        let mut row_n = 0;
        let mut col_n = 0;

        for row in &self.contents {
            col_n = 0;

            for c in row {
                if *c == 'X' {
                    println!("found x: {}, {}", row_n, col_n);
                    self.find_m(row_n, col_n);
                }

                col_n += 1;
            }

            row_n += 1;
        }
    }

    fn find_m(&self, x_row: usize, x_col: usize) {
        let mut possible_m_locations = vec![];

        let left_up = (x_row.checked_sub(1), x_col.checked_sub(1));
        possible_m_locations.push(left_up);

        let up = (x_row.checked_sub(1), Some(x_col));
        possible_m_locations.push(up);

        let right_up = (x_row.checked_sub(1), x_col.checked_add(1));
        possible_m_locations.push(right_up);

        let right = (Some(x_row), x_col.checked_add(1));
        possible_m_locations.push(right);

        let right_down = (x_row.checked_add(1), x_col.checked_add(1));
        possible_m_locations.push(right_down);

        let down = (x_row.checked_add(1), Some(x_col));
        possible_m_locations.push(down);

        let left_down = (x_row.checked_add(1), x_col.checked_sub(1));
        possible_m_locations.push(left_down);

        let left = (Some(x_row), x_col.checked_sub(1));
        possible_m_locations.push(left);

        for mut loc in &mut possible_m_locations {
            if loc.0.is_none() || loc.1.is_none() {
                loc.0 = None;
                loc.1 = None;
            }
        }
        
        println!("Possible 'M' locations: M({:?})", possible_m_locations);
    }
}


fn main() -> Result<(), Box<dyn Error>> {

    let input = fs::read_to_string("./day_four_test.txt")?;
    // println!("{:?}", input);


    let mut row: Vec<char> = vec![];
    let mut ws = WordSearch::new();

    for c in input.chars() {
        // println!("{:?}", c);
        if c == '\n' {
            ws.contents.push(row);
            row = vec![];
        } else {
            row.push(c);
        }
    }

    ws.pprint();

    ws.find_x();
    



    Ok(())

}

