use std::fs;
use std::error::Error;


mod ten;


fn main() -> Result<(), Box<dyn Error>> {

    let input = fs::read_to_string("./day_ten_test.txt")?;
    // let input = fs::read_to_string("./day_ten_input.txt")?;
    // println!("{:?}", input);

    
    ten::run(input);
   

    Ok(())

}

