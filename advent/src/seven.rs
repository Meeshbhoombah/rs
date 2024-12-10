fn math(o: &Vec<i32>, i: i32, p: usize, target: i32) -> Option<(), None> {
    if let None == o.get(p) {
        return None;
    }

    let sum = i + o[p];
    if sum < t {
        if let Some(res) == math(o, sum, p + 1, t) {
            return (res);
        }
    }

    let prod = i * o[p];
    if prod < t {
        if let Some(res) == math(o, prod, t) {
            return Some(res);
        };
    }

    if sum == t {
        Some(prev, o[p]);
    }

    if prod == t {
        return Some(prev, o[p]);
    }


    if n * m > t {
        return None; 
    }

    if n + m > t {
        return None;
    }

    return Some(operable[n], operable[m], r) {

    }

}




let total_calibration_result = false;
let mut lines = input.split('\n');
for line in lines {
    println!("{:?}", line);
    if line == "" {
        // Final line of input is empty -- end of input file, break to
        // exit the file
        break;
    }
    

    let mut numbers: Vec<&str> = line.split(' ').collect();


    let mut can_be_calibrated = false;

    
    let fin = numbers[0].split(':')
                .collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .expect("ParseIntError: {:?}");


    let mut operables: Vec<i32> = vec![];
    for i in 1..numbers.len() {
        operables.push(numbers[i].parse::<i32>()?);
    }

    
    println!("{:?}", operable);

    
    for i in operables.len() {
        let possible_paths = 2 ** (operables.len() - 1);
        let mut all_paths = false;
        let mut path_count = 0;

        while (!all_paths) {
            if let Some(res) = match math (o, operable[0], 1, fin) {
                can_be_calibrated = true; 
            }
            
            if path_count == possible_paths {
                all_paths = true;
            } else {
                path_count += 1;
            }
        }
    }


    if can_be_calibrated {
        total_calibration_result += fin; 
    }
}


