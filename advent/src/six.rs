fn up(map_width: usize, pos: usize) -> usize {
    pos - map_width
}


let input = fs::read_to_string("./day_six_input.txt").unwrap();
// let input = fs::read_to_string("./day_six_test.txt").unwrap();
println!("{:?}", input);


let mut map_width = 0;
for c in input.chars() {
    if c == '\n' {
        break;
    }

    map_width += 1;
}

// println!("{:?}", map_width);


let mut map = vec![];
let mut start_pos: usize = 0;

let mut i: usize = 0;
for c in input.chars() {
    if c == '^' {
        start_pos = i; 
    }

    if c == '\n' {
        continue;
    } else {
        map.push(c);
    }

    i += 1;
}

// println!("{:?}", map);
// println!("{:?}", start_pos);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn move_gaurd(p: usize, d: Direction) -> usize {
    match d {
        Direction::Up => {
            p.checked_sub(10).expect("Up directional sub fail.")
        },
        Direction::Right => {
            p.checked_add(1).expect("Up directional sub fail.")
        },
        Direction::Down => {
            p.checked_add(10).expect("Up directional sub fail.")
        },
        Direction::Left => {
            p.checked_sub(1).expect("Up directional sub fail.")
        },
    }
}

fn right_turn(d: Direction) -> Direction {
    match d {
        Direction::Up => {
            Direction::Right
        },
        Direction::Right => {
            Direction::Down 
        },
        Direction::Down => {
            Direction::Left
        },
        Direction::Left => {
            Direction::Up
        },
    }
}

fn hit_wall(p: usize, d: Direction) -> usize {
    match d {
        Direction::Up => {
            p.checked_add(10).expect("Up directional sub fail.")
        },
        Direction::Right => {
            p.checked_sub(1).expect("Up directional sub fail.")
        },
        Direction::Down => {
            p.checked_sub(10).expect("Up directional sub fail.")
        },
        Direction::Left => {
            p.checked_add(1).expect("Up directional sub fail.")
        },
    }
}


let mut visits = vec![];

let mut exit = false; 
let mut pos = start_pos;
let mut current_dir = Direction::Up;

while !exit {

    println!("{:?}", current_dir);
    if pos > map.len() {
        println!("{:?}", visits);
        println!("{:?}", visits.len() - 1);
        process::exit(1) 
    }

    let c = map[pos];
    println!("{:?}", c);
    if c == '#' {
        println!("HIT WALL");
        visits.pop();
        pos = hit_wall(pos, current_dir);
        current_dir = right_turn(current_dir);
    }

    pos = move_gaurd(pos, current_dir);

    if visits.contains(&pos) {
        continue;
    } else {
        visits.push(pos);
    }
}
