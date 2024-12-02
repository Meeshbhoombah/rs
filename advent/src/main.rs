use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {

    // let input = fs::read_to_string("./day_two_test.txt")?;
    let input = fs::read_to_string("./day_two_input.txt")?;
    println!("{:?}", input);


    Ok(())

}

