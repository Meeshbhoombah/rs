use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;


struct MiddlePage {
    orderings: HashMap<i32, Vec<i32>>,
    page_list: Vec<Vec<i32>>
}


impl  MiddlePage {

    pub fn new() -> Self {
        MiddlePage {
            orderings: HashMap::new(),
            page_list: vec![]
        }
    }

    fn parse(& mut self, input: String) {
        let mut lines: Vec<&str> = input.split('\n').collect();
        println!("{:?}", lines);


        let mut reading_queue = true;
        for line in lines {
            if line == "" {
                reading_queue = false; 
                continue;
            }

            if reading_queue {
                let ordering_pair_str: Vec<&str> = line.split('|').collect();

                let mut op_i32: Vec<i32> = vec![];
                for page in ordering_pair_str {
                    op_i32.push(page.parse::<i32>().expect("Not an integer."))
                }

                println!("{:?}", op_i32);

                // This accidentally dupes the page that I am adding to a 
                // particular orderings' key, which is why this match statement
                // works better
                // let afters = self.orderings.entry(op_i32[0]).or_insert(vec![op_i32[1]]);
                match self.orderings.entry(op_i32[0]) {
                    Entry::Occupied(v) => {
                        v.into_mut().push(op_i32[1]);
                    },
                    Entry::Vacant(v) => {
                        v.insert(vec![op_i32[1]]);
                    }
                };

                // println!("{:?}", afters);
            }
        }

        println!("{:?}", self.orderings);
    }

}



fn main() {

    // let input = fs::read_to_string("./day_five_input.txt").unwrap();
    let input = fs::read_to_string("./day_five_test.txt").unwrap();
    println!("{:?}", input);

    let mut mp = MiddlePage::new();
    mp.parse(input);

}

