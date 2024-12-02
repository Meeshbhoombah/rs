48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20


// TODO: part 2 -- check each possible failure case per report to find a
// case that passes (if it does, add it to the count of safe reports)

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

for mut report in reports {
    println!("Report({:?})", report);
    let mut report_length = report.len();

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

    let mut edits = 0;
    let mut is_safe_report = true;

    while j < report_length {
        let a = report[i];
        let b = report[j];
        // println!("{:?}, {:?}", a, b);


        if a < b {
            if initial_slope == 1 {
                if edits > 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                } else {
                    // If this is the first instance of this failure, 
                    // increment that an edit is to be made
                    println!("Resetting loop for Report({:?}), {:?}, {:?}, removing {:?}", report, a, b, a);
                    edits += 1;

                    // Remove the first of the two elements where the error
                    // occured
                    report.remove(i);
                    report_length = report.len();

                    // Reset the loop
                    i = 0;
                    j = 1;
                    continue;
                }
            }
        } else {
            if initial_slope == 0 {
                if edits > 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                } else {
                    // If this is the first instance of this failure, 
                    // increment that an edit is to be made
                    println!("Resetting loop for Report({:?}), {:?}, {:?}, removing {:?}", report, a, b, a);
                    edits += 1;

                    // Remove the first of the two elements where the error
                    // occured
                    report.remove(i);
                    report_length = report.len();

                    // Reset the loop
                    i = 0;
                    j = 1;
                    continue;
                }
            }
        }


        if initial_slope == 0 {
            if b - a > 3 || b - a == 0 {
                if edits > 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                } else {
                    // If this is the first instance of this failure, 
                    // increment that an edit is to be made
                    println!("Resetting loop for Report({:?}), {:?}, {:?}, removing {:?}", report, a, b, a);
                    edits += 1;

                    // Remove the first of the two elements where the error
                    // occured
                    report.remove(i);
                    report_length = report.len();

                    // Reset the loop
                    i = 0;
                    j = 1;
                    continue;
                }
            }
        } else {
            if a - b > 3 || b - a == 0 {
                if edits > 0 {
                    is_safe_report = false;
                    println!("{:?}, {:?} cause Report({:?}) to be false", a, b, report);
                    break;
                } else {
                    // If this is the first instance of this failure, 
                    // increment that an edit is to be made
                    println!("Resetting loop for Report({:?}), {:?}, {:?}, removing {:?}", report, a, b, a);
                    edits += 1;

                    // Remove the first of the two elements where the error
                    // occured
                    report.remove(i);
                    report_length = report.len();

                    // Reset the loop
                    i = 0;
                    j = 1;
                    continue;
                }
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


