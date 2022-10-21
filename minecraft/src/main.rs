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


const iCHUNK_SIDE_LENGTH: i32 = 16;
const fCHUNK_SIDE_LENGTH: f32 = 16.0;

#[derive(Debug)]
pub struct Chunk {
    nw: Block,
    ne: Block,
    se: Block,
    sw: Block,
}

impl Chunk {
    fn nw_corner(&self, b: &Block) -> Block {
        let n = b.x as f32 / fCHUNK_SIDE_LENGTH;
        let a = b.z as f32 / fCHUNK_SIDE_LENGTH;
        
        let n = n.floor();
        let a = a.floor();

        let n = n * fCHUNK_SIDE_LENGTH;
        let a = a * fCHUNK_SIDE_LENGTH;

        let x = n as i32;
        let z = a as i32;

        Block {
            x,
            y: SEA_LEVEL,
            z
        }
    }

    fn ne_corner(&self, b: &Block) -> Block {
        let n = self.nw_corner(&b);

        let x = n.x + iCHUNK_SIDE_LENGTH;
        let y = n.y;
        let z = n.z;

        Block {
            x,
            y,
            z
        }
    }

    fn se_corner(&self, b: &Block) -> Block {
        let n = self.nw_corner(&b);

        let x = n.x + iCHUNK_SIDE_LENGTH;
        let y = n.y;
        let z = n.z + iCHUNK_SIDE_LENGTH;

        Block {
            x,
            y,
            z
        }
    }

    fn sw_corner(&self, b: &Block) -> Block {
        let n = self.nw_corner(&b);

        let x = n.x;
        let y = n.y;
        let z = n.z + iCHUNK_SIDE_LENGTH;

        Block {
            x,
            y,
            z
        }     
    }

    pub fn new_from_block(&self, b: Block) -> Self {
        let nw = self.nw_corner(&b);
        let ne = self.ne_corner(&b);
        let se = self.se_corner(&b);
        let sw = self.sw_corner(&b);

        Self {
            nw,
            ne,
            se,
            sw
        } 
    }
}


/*
pub fn make_portal_path(points: Vec<Block>) -> Vec<Block> {
}

pub fn make_water_channel(points: Vec<Block>) -> Vec<Block> {
}
*/

fn main() {
    let tree_house = Block {
        x: 6042,
        y: 78,
        z: 910
    };
}

