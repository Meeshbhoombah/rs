use std::env;
use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() -> Result<(), Box<dyn Error>> {

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // Raw file input
    let input: String = fs::read_to_string("day_one_test.txt")?;
    // let input: String = fs::read_to_string("day_one_input.txt")?;
    // println!("{}", input);


    let mut current_list = 0;
    let mut list_zero = BinaryHeap::new();
    let mut list_one = BinaryHeap::new();


    let pairs = input.split("\n");
    for pair in pairs {
        let elements = pair.split("   ");
        for element in elements {
            // The list we are adding to atm
            // println!("{}", current_list);

            // Print each indiviudal location id in dual list setup
            // println!("{}", element);

            if current_list == 0 {
                list_zero.push(Reverse(element));
                current_list = 1;
            } else {
                list_one.push(Reverse(element));
                current_list = 0;
            }
        }
    }

    // For some reason, inputting all values into both min_heaps adds a "" 
    // value to `list_zero` that we do not need
    list_zero.pop();

    println!("{:?}", list_zero);
    println!("{:?}", list_one);

    Ok(())

}

