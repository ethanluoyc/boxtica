use super::Block::Block;

pub struct Maze {
    width: u32,
    height: u32,
    data: Vec<Block>,
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        let mut data = Vec::new();
        for i in 0..width*height {
            data.push(Block::Empty)
        }
        Maze {
            width,
            height,
            data
        }
    }
}
