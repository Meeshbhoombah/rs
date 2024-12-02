use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {

    // let input = fs::read_to_string("./day_two_test.txt")?;
    let input = fs::read_to_string("./day_two_input.txt")?;
    // println!("{:?}", input);
    
    let mut reports_as_strings: Vec<&str> = input.split("\n").collect();
    // Remove final empty report
    reports_as_strings.pop();
    // println!("{:?}", reports_as_strings);


    let mut reports = vec![];
    for report in reports_as_strings {
        let split = report.split(" ");
        // println!("{:?}", split);
       
        let mut final_report = vec![];
        for n in split {
            let elem = n.parse::<i32>()?;
            final_report.push(elem);
        }

        reports.push(final_report);
    }

    // println!("{:?}", reports);

    let mut safe_report_count = 0;

    for report in reports {
        let report_length = report.len();

        let mut i: usize = 0;
        let mut j: usize = 1;

        // If the first two levels in the report are increasing, then its slope
        // is `0`, otherwise it has a slope of `1`, which would mean the levels 
        // are decreasing
        let mut initial_slope = 0;
        if report[i] < report[j] {
            initial_slope = 0;
        } else {
            initial_slope = 1;
        }

        let mut is_safe_report = true;

        while j < report_length {
            let a = report[i];
            let b = report[j];
            // println!("{:?}, {:?}", a, b);


            if a < b {
                if initial_slope == 1 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                }
            } else {
                if initial_slope == 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                }
            }


            if initial_slope == 0 {
                if b - a > 3 || b - a == 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                }
            } else {
                if a - b > 3 || b - a == 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                }
            }


            i += 1;
            j += 1;
        }

        if is_safe_report {
            safe_report_count += 1;
            println!("Report({:?}) is safe", report)
        }
    }

    println!("Safe Reports: {:?}", safe_report_count);


    Ok(())

}

