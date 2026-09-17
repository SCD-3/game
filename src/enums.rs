#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)] // idrc
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
impl Direction {

    pub fn offset_pos(self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up        => (pos.0,     pos.1 + 1),
            Direction::Down      => (pos.0,     pos.1 - 1),
            Direction::Left      => (pos.0 - 1, pos.1    ),
            Direction::Right     => (pos.0 + 1, pos.1    ),
            Direction::UpLeft    => (pos.0 - 1, pos.1 + 1),
            Direction::UpRight   => (pos.0 + 1, pos.1 + 1),
            Direction::DownLeft  => (pos.0 - 1, pos.1 - 1),
            Direction::DownRight => (pos.0 + 1, pos.1 - 1),
        }
    }

}