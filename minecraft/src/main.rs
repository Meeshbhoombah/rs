/// Minecraft
/// src/main.rs
///
/// Binary application running random minecraft utility functions

#[derive(Debug)]
pub struct Block {
    /// We use `i32` to eclipse the largest coordinate possible for a Minecraft
    /// block
    ///
    /// World border is located at X/Z ±29,999,984
    x: i32,
    y: i32,
    z: i32,
}



const MAX_Y_OVERWORLD: i32 = 320;
const MAX_Y_END_OR_NETHER: i32 = 256;

const SEA_LEVEL: i32 = 62;


const CHUNK_SIDE_LENGTH: i32 = 16;

pub struct Chunk {
    nw: Block,
    ne: Block,
    se: Block,
    sw: Block,
}

pub fn find_se_chunk_corner(b: Block) -> Block {
    let x = b.x / 16 * 16;
    let z = b.z / 16 * 16;

    Block {
        x,
        y: SEA_LEVEL,
        z
    }
}

/*
pub fn find_sw_chunk_corner(b: Block) -> {
    let x = b.x / 16 * 16;
    let z = b.z / 16 * 16 + 16;

    Block {
        x,
        y: SEA_LEVEL,
        z
    }
}
*/


impl Chunk {
    /*
    pub fn new_from_block(b: Block) -> Self {
        let se = find_se_chunk_corner(b);

        Self {
        } 
    }
    */
}


/*
pub fn make_portal_path(points: Vec<Block>) -> Vec<Block> {
}

pub fn make_water_channel(points: Vec<Block>) -> Vec<Block> {
}
*/

fn main() {
    print!("Results \n");

    // Base
    let base = Block {
        x: 6041,
        y: 83,
        z: 923
    };

    let random_block = Block {
        x: 6056,
        y: 23,
        z: 905
    };
    
    let se = find_se_chunk_corner(base);
    let se_random = find_se_chunk_corner(random_block);
    println!("{:?}", se);
    println!("{:?}", se_random);
}

#[cfg(tests)]
mod tests {
    use super::*;

    pub fn base_block() -> Block {
        Block {
            x: 6041,
            y: 83,
            z: 932
        }
    }

    #[test]
    pub fn find_se_chunk_corner() {
         
    }
}

