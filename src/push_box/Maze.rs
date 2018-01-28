use super::Block::Block;
use super::Direction::Direction;

use std::collections::HashSet;

pub struct Maze {
    width: i32,
    height: i32,
    data: Vec<Block>,
    player_location: Option<(i32, i32)>,
    targets: HashSet<(i32, i32)>,
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
            targets: HashSet::new(),
        }
    }

    pub fn add_target(&mut self, x: i32, y: i32) -> Option<()> {
        if self.is_in_maze(x, y) && self.targets.insert((x, y)) {
            Some(())
        } else {
            None
        }
    }

    fn get_box(&mut self, x: i32, y: i32) -> Option<Block> {
        if self.is_in_maze(x, y) {
            let idx = x * self.height + y;
            Some(self.data[idx as usize].clone())
        } else {
            None
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

    fn make_move(&mut self, dir: Direction) -> Option<()> {
        match self.player_location {
            None => None,
            Some((x, y)) => {
                let (next_x, next_y) = dir.next_step(x, y);
                if let Some(ref next_box) = self.get_box(next_x, next_y) {
                    match next_box {
                        &Block::Empty => {
                            self.set_box(x, y, Block::Empty);
                            self.set_box(next_x, next_y, Block::Player);
                            Some(())
                        },
                        &Block::Wall => {
                            None
                        },
                        &Block::Boxtica => {
                            let (next_next_x, next_next_y) = dir.next_step(x, y);
                            if let Some(ref next_next_box) = self.get_box(next_next_x, next_next_y).clone() {
                                match next_next_box {
                                    &Block::Empty => {
                                        self.set_box(x, y, Block::Empty);
                                        self.set_box(next_x, next_y, Block::Player);
                                        self.set_box(next_next_x, next_next_y, Block::Boxtica);
                                        Some(())
                                    }
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        },
                        &Block::Player => panic!(),
                    }
                } else {
                    None
                }
            }
        }
    }
}
