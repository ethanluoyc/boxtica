use super::Block::Block;

pub struct Maze {
    width: i32,
    height: i32,
    data: Vec<Block>,
}

impl Maze {
    pub fn new(width: i32, height: i32) -> Maze {
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

    fn set_box(&mut self, width: i32, height: i32, block: Block) {
        let idx = width * self.height + height;
        self.data[idx as usize] = block;
    }

    fn is_in_maze(&self, width: i32, height: i32) -> bool {
        width >= 0 && width < self.width
            && height >= 0 && height < self.height
    }
}
