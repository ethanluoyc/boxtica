pub enum Block {
    Empty,
    Player,
    Wall,
    Boxtica,
    // Whether target has a box or not
    Target(bool),
}

impl Block {
    pub fn can_place_item(&self) -> bool {
        match *self {
            Block::Empty | Block::Target(false) => true,
            _ => false,
        }
    }

    pub fn can_move(&self) -> bool {
        match *self {
            Block::Boxtica | Block::Target(true) => true,
            _ => false,
        }
    }
}
