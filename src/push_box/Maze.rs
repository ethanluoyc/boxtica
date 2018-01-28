use super::Block::Block;

pub struct Maze {
    width: i32,
    height: i32,
    data: Vec<Block>,
    player_location: Option<(i32, i32)>,
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

    fn set_box(&mut self, x: i32, y: i32, block: Block) {
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

    fn is_in_maze(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width
            && y >= 0 && y < self.height
    }
}
