use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error;
use std::fs;

#[derive(Clone)]
struct MiddlePage {
    orderings: HashMap<i32, Vec<i32>>,
    page_groups: Vec<Vec<i32>>,
    invalids: Vec<Vec<i32>>
}


impl MiddlePage {

    pub fn new() -> Self {
        MiddlePage {
            orderings: HashMap::new(),
            page_groups: vec![],
            invalids: vec![],
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
                
                match self.orderings.entry(op_i32[1]) {
                    Entry::Occupied(v) => {
                        v.into_mut().push(op_i32[0]);
                    },
                    Entry::Vacant(v) => {
                        v.insert(vec![op_i32[0]]);
                    }
                };
            } else {
                // We've finished reading the queue of orderings and are now
                // working with the page groupings list
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

    fn sum(&mut self) {
        let mut valid_page_groups = vec![];
        for page_group in &self.page_groups {
            // println!("{:?}", page_group);
            let mut all_disallowed_values: Vec<i32> = vec![];
            let is_not_valid_page_group = 'page_group_ordering_false: {
                for page in page_group {
                    if let Some(disallowed_values) = self.orderings.get(page) {
                        // println!("Has disallowed vals: {:?} -- {:?}", page, disallowed_values);
                        all_disallowed_values.extend(disallowed_values);
                    } else {
                        // println!("No disallowed vals: {:?}", page);
                    };

                    if all_disallowed_values.contains(&page) {
                        // println!("Out of order printing: {:?}", page);
                        break 'page_group_ordering_false true;
                    }
                    // println!("All disallowed vals: {:?}", all_disallowed_values);
                }

                valid_page_groups.push(page_group);
                false
            };

            if is_not_valid_page_group {
                self.invalids.push(page_group.clone());
                continue;
            }
        }

        let mut sum = 0;
        for page in valid_page_groups {
            let middle = (page.len() as f32 / 2 as f32).floor() as usize;
            // println!("{:?}", middle);
            sum += page[middle];
        }

        println!("Final sum first star: {:?}", sum);
        println!("{}", ' ');
    }

    fn sum2(& mut self) {
        for page_group in &self.invalids {
            // println!("{:?}", page_group);
            let mut pos_map: HashMap<i32, usize> = HashMap::new();
            let mut dv_local: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut i: usize = 0;
            for page in page_group {
                if let None = pos_map.get(page) {
                    pos_map.insert(*page, i);
                } else {
                    panic!("Pre-existing entity {:?}?", page);
                }

                if let Some(disallowed_values) = self.orderings.get(page) {
                    // println!("Has disallowed vals: {:?} ({:?})", page, disallowed_values);
                    for v in disallowed_values {
                        match dv_local.entry(*v) {
                            Entry::Occupied(val) => {
                                val.into_mut().push(*page);
                            },
                            Entry::Vacant(val) => {
                                val.insert(vec![*page]);
                            }
                        };
                    }
                    // println!("Not allowed (keys): {:?}", dv_local);
                } else {
                    // println!("No disallowed vals: {:?}", page);
                };

                i += 1;
            }

            println!("------");
            println!("{:?}", page_group);
            println!("Final: {:?}", pos_map);
            println!("{:?}", dv_local);
            println!("{}", ' ');
        }    
    }

}



fn main() {

    // let input = fs::read_to_string("./day_five_input.txt").unwrap();
    let input = fs::read_to_string("./day_five_test.txt").unwrap();
    // println!("{:?}", input);

    let mut mp = MiddlePage::new();

    if let Err(e) = mp.load(input) {
        println!("Error loading input -- MiddlePage: {:?}", e);
    };

    mp.sum();
    mp.sum2();

}

