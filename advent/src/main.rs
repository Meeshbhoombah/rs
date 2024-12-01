use std::env;
use std::fs;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() -> Result<(), Box<dyn Error>> {

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // Raw file input
    // let input: String = fs::read_to_string("day_one_test.txt")?;
    let input: String = fs::read_to_string("day_one_input.txt")?;
    // println!("{}", input);


    let mut lists_length = 0;
    let mut list_zero = BinaryHeap::new();
    let mut list_one = BinaryHeap::new();


    let mut current_list = 0;
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
            
        lists_length += 1;
    }


    // For some reason, inputting all values into both min_heaps adds a "" 
    // value to `list_zero` that we do not need
    list_zero.pop();
    lists_length -= 1;
   
    /*
    println!("{:?}", lists_length);
    println!("{:?}", list_zero);
    println!("{:?}", list_one);
    */


    let mut summed_location_difference = 0;

    let mut i = 0;
    while i < lists_length {
        println!("zero: {:?}", list_zero.peek());
        println!("one: {:?}", list_one.peek());


        // `Reversed` is a tuple
        let location_difference = (list_one.pop().unwrap().0.parse::<i128>().unwrap() - list_zero.pop().unwrap().0.parse::<i128>().unwrap()).abs();
        println!("location_difference: {}", location_difference);
        summed_location_difference += location_difference;

        i += 1;

        println!("--- NEXT PAIR ---");
    }


    println!("RESULT:");
    println!("{}", summed_location_difference);


    Ok(())

}

