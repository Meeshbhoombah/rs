use std::fmt;
use std::error::Error;
use std::collections::VecDeque;


#[derive(Debug)]
enum E {
    UnableToParseMapElevationAsDecimal
}

impl Error for E {}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            E::UnableToParseMapElevationAsDecimal => {
                write!(f, "Failed to parse map elevation as decimal")
            }
        }
    }
}


#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}


type Elevation = (usize, Direction);


#[derive(Debug)]
struct Trail {
    map: Vec<i8>,
    trailheads: Vec<usize>,
    side_len: usize
}

impl From<String> for Trail {
    fn from(topographic_map: String) -> Trail {
        let mut map = vec![];
        let mut trailheads = vec![];
        let mut side_len: usize = 0;
        let mut measure_length = false;

        let mut index: usize = 0;
        for c in topographic_map.chars() {
            if c == '\n' {
                measure_length = true;
                continue;
            }

            if measure_length == true {
                side_len = index;
                measure_length = false;
            }

            if c == '0' {
                trailheads.push(index);
            }

            let r: u32 = 10;
            if let Some(d) = c.to_digit(r) {
                map.push(d as i8);
            }

            index += 1;
        }

        side_len = index - side_len;


        Trail {
            map,
            trailheads,
            side_len
        }
    }
}

impl Trail {

    fn new() -> Self {
        Trail {
            map: vec![],
            trailheads: vec![],
            side_len: 0
        }
    } 

    fn find_valid_directions(
        &self, 
        pos: usize, 
        from: Option<Direction>
    ) -> Option<Vec<Elevation>> {
        let mut vd = vec![];

       
        let mut avoid: Direction =
        if from.is_some() {
            let avoid 
        }


        if let Some(i) = pos.checked_sub(self.side_len) {
            if let Some(e) = self.map.get(i) {
                if *e == self.map[pos] + 1 {
                    vd.push((i, Direction::Up));
                }
            }
        }

        if let Some(i) = pos.checked_add(1) {
            if let Some(e) = self.map.get(i) {
                if *e == self.map[pos] + 1 {
                    vd.push((i, Direction::Right));
                }
            }
        }

        if let Some(i) = pos.checked_add(self.side_len) {
            if let Some(e) = self.map.get(i) {
                if *e == self.map[pos] + 1 {
                    vd.push((i, Direction::Down));
                }
            }
        }

        if let Some(i) = pos.checked_sub(1) {
            if let Some(e) = self.map.get(i) {
                if *e == self.map[pos] + 1 {
                    vd.push((i, Direction::Left));
                }
            }
        }

        if vd.is_empty() {
            return None;
        } else {
            return Some(vd);
        }
    }


    fn find_all_trails(&self, start_pos: usize) -> i32 {
        let mut q = VecDeque::new();

        if let Some(valid_directions) = self.find_valid_directions(start_pos, None) {
            for vd in valid_directions {
                q.push_front(vd);
            }
        };

        // println!("Trailhead at {:?}: ValidDirections({:?})", start_pos, q);

        for elevation in &q {
            // println!("{:?}", i);
            if let Some(valid_directions) = self.find_valid_directions(elevation.0, Some(elevation.1)) {
                for vd in valid_directions {
                    q.push_front(vd);
                }
            };
        }

        println!("Trailhead at {:?}: ValidDirections({:?})", start_pos, q);

        return q.len() as i32;
    }


    fn sum_trailheads(&self) -> i32 {
        let mut sum = 0;

        for trailhead in &self.trailheads {
            let trails = self.find_all_trails(*trailhead);
            sum += trails;
        }

        return sum;
    }

}


pub fn run(input: String) -> Result<(), Box<dyn Error>> {

    // println!("{:?}", input);

    let t = Trail::from(input);
    // println!("{:?}", t.map);
    // println!("{:?}", t.map.len());
    // println!("{:?}", t.side_len);
    // println!("{:?}", t.trailheads);
    println!("{:?}", t.sum_trailheads());

    Ok(())

}

