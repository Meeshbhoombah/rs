use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error;
use std::fs;


struct MiddlePage {
    orderings: HashMap<i32, Vec<i32>>,
    page_groups: Vec<Vec<i32>>
}


impl  MiddlePage {

    pub fn new() -> Self {
        MiddlePage {
            orderings: HashMap::new(),
            page_groups: vec![]
        }
    }

    fn load(& mut self, input: String) -> Result<(), Box<dyn Error>> {
        let lines: Vec<&str> = input.split('\n').collect();
        // println!("{:?}", lines);


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
                    match page.parse::<i32>() {
                        Ok(v) => {
                            op_i32.push(v);
                        },
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    }
                }

                // println!("{:?}", op_i32);

                match self.orderings.entry(op_i32[0]) {
                    Entry::Occupied(v) => {
                        v.into_mut().push(op_i32[1]);
                    },
                    Entry::Vacant(v) => {
                        v.insert(vec![op_i32[1]]);
                    }
                };

                // println!("{:?}", afters);
            } else {
                let page_group_str: Vec<&str> = line.split(',').collect();
                
                let mut pg_i32: Vec<i32> = vec![];
                for page in page_group_str {
                    match page.parse::<i32>() {
                        Ok(v) => {
                            pg_i32.push(v);
                        },
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    }
                }

                self.page_groups.push(pg_i32);
            }
        }

        // println!("{:?}", self.orderings);
        // println!("{:?}", self.page_groups);

        Ok(())
    }

    fn sum(&self) -> i32 {

        let sum = 0;

        for page_group in &self.page_groups {
            println!("{:?}", page_group);

            for page in page_group {
                if let Some(values) = self.orderings.get(page) {
                    // TODO: parse order pairs to construct a HashMap of 
                    // orderings such that each key is every number that is
                    // listed as being necessitated to come after a said 
                    // number
                    //
                    // Currently, the HashMap stores orderings where each key
                    // is a number that needs to come before a set of numbers,
                    // which is the value
                    println!("In orderings: {:?} -- {:?}", page, values);
                } else {
                    println!("Not in orderings: {:?}", page);
                };
            }
        }

        return sum;
    }

}



fn main() {

    // let input = fs::read_to_string("./day_five_input.txt").unwrap();
    let input = fs::read_to_string("./day_five_test.txt").unwrap();
    // println!("{:?}", input);

    let mut mp = MiddlePage::new();
    mp.load(input);
    mp.sum();

}

