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

    fn set_box(&mut self, width: i32, height: i32, block: Block) {
        if !self.is_in_maze(width, height) {
            return
        }
        let idx = width * self.height + height;
        match &block {
            &Block::Player => {
                self.player_location = Some((width, height))
            },
            _ => {},
        }
        self.data[idx as usize] = block;
    }

    fn is_in_maze(&self, width: i32, height: i32) -> bool {
        width >= 0 && width < self.width
            && height >= 0 && height < self.height
    }
}
