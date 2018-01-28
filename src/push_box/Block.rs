pub enum Block {
    Empty,
    Player,
    Wall,
    Boxtica,
    // Whether target has a box or not
    Target(bool),
}
