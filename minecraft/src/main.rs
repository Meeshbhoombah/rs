/// Minecraft
/// src/main.rs
///
/// Binary application running random minecraft utility functions

#[derive(Debug)]
pub struct Block {
    /// We use `i32` to eclipse the largest coordinate possible for a Minecraft
    /// block -- the World Border is located at X/Z ±29,999,984
    x: i32,
    y: i32,
    z: i32,
}


/// The maximum altitude for the Overworld is 320
const MAX_Y_OVERWORLD: i32 = 320;

/// For both the Nether and the End, the maximum altitude is 256
const MAX_Y_OTHERWORLD: i32 = 256;

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


pub fn tree_house_center() -> Block {
    Block {
        x: 6042,
        y: 78,
        z: 910
    }
}

pub fn test_find_se_chunk_corner() {
    let b = tree_house_center();
    let se = find_se_chunk_corner(b);
    assert_eq!(se.x, 6032);
    assert_eq!(se.z, 912);
}

pub fn tests() {
    test_find_se_chunk_corner();
}


fn main() {
    tests();
}

