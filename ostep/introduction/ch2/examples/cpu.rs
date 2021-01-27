use std::env;
use std::{thread, time};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cpu <string>");
        std::process::exit(1)
    }

    let msg = args.into_iter().nth(1);
    loop {
        match &msg {
            Some(data) => {
                thread::sleep(time::Duration::from_secs(1));
                println!("{}", data)

            },
            _ => std::process::exit(0),
        }
    }
}


