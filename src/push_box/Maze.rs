use super::block::Block;

pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Block>,
    pub player_location: Option<(i32, i32)>,
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
            data,
            player_location: None,
        }
    }

    pub fn set_box(&mut self, x: i32, y: i32, block: Block) {
        if !self.is_in_maze(x, y) {
            return
        }
        let idx = x * self.height + y;
        match &block {
            &Block::Player => {
                self.player_location = Some((x, y))
            },
            _ => {},
        }
        self.data[idx as usize] = block;
    }

    pub fn is_in_maze(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width
            && y >= 0 && y < self.height
    }

}
