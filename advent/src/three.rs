// let input = fs::read_to_string("./day_three_test.txt")?;
let input = fs::read_to_string("./day_three_input.txt")?;
// println!("{:?}", input);


let mut found_mul = false;
let mut multipliers_start = false;

let mut number_builder = String::from(""); 

let mut all_multipliers: Vec<Vec<i32>> = vec![];
let mut curr_set_of_multipliers = vec![];

let mut prev_char: char = '\0';

for c in input.chars() {
    println!("{:?}", c);

    if c == 'm' {
        found_mul = true;
        println!("found_mul");
    }

    if found_mul {
        if c == ',' {
            if number_builder == String::from("") {
                found_mul = false;
                continue;
            }

            println!("first number: {:?}", number_builder);
            curr_set_of_multipliers.push(number_builder.parse::<i32>()?);
            number_builder = String::from("");
        }

        if multipliers_start {
            if c != '(' && c != ',' && c != ')' {
                // println!("Pushing character: {:?}", c);
                if !c.is_ascii_digit() && c != ',' {
                    println!("breaking mul instruction pattern: `mul(` -- reset");
                    curr_set_of_multipliers = vec![];
                    number_builder = String::from("");
                    multipliers_start = false;
                    found_mul = false;
                    continue;
                }

                number_builder.push(c);
            }
        }

        if multipliers_start == false {
            if c != 'm' && c != 'u' && c != 'l' && c != '(' {
                println!("breaking mul instruction pattern: `mul(` -- reset");
                found_mul = false; 
            }
        }

        if c == '(' {
            if prev_char != 'l' {
                found_mul = false;
                continue;
            }

            multipliers_start = true;
            println!("multiplers_start");
        }

        if c == ')' {
            println!("second number: {:?}", number_builder);
            curr_set_of_multipliers.push(number_builder.parse::<i32>()?);
            println!("current multipliers: {:?}", curr_set_of_multipliers);
            all_multipliers.push(curr_set_of_multipliers);
            curr_set_of_multipliers = vec![];
            number_builder = String::from("");
            multipliers_start = false;
            found_mul = false;
            println!("multiplers_end");
        }
    }

    prev_char = c;
}

println!("{:?}", all_multipliers);

let mut sum = 0;
for group in all_multipliers {
    if group.len() != 2 {
        println!("Throwing out: {:?}", group);
        continue;
    }

    sum += group[0] * group[1];
}

println!("All mul summed result: {:?}", sum);

let correct_instruction = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
let mut total = 0;

for (_, [num1, num2]) in correct_instruction
    .captures_iter(&input)
    .map(|c| c.extract())
{
    total += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
}


println!("All mul summed result w/ regex: {:?}", total);

let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
let mut enabled = true;

let part2_sum = re.captures_iter(&input)
    .filter_map(|cap| {
        if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
            if enabled {
                let x = x.as_str().parse::<i32>().unwrap();
                let y = y.as_str().parse::<i32>().unwrap();
                Some(x * y)
            } else {
                None
            }
        } else {
            match &cap[0] {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {}
            }
            None
        }
    })
    .sum::<i32>();

println!("All enabled mul summed result w/ regex: {:?}", part2_sum);



