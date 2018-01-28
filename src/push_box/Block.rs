pub enum Block {
    Empty,
    Player,
    Wall,
    Boxtica,
}

impl Block {
    pub fn can_place_item(&self) -> bool {
        match *self {
            Block::Empty => true,
            _ => false,
        }
    }

    pub fn can_be_moved(&self) -> bool {
        match *self {
            Block::Boxtica => true,
            _ => false,
        }
    }
}
