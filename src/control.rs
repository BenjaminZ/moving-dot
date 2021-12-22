use ggez::event::KeyCode;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(key: &KeyCode) -> Option<Direction> {
        let input = match key {
            KeyCode::W => Some(Direction::Up),
            KeyCode::Up => Some(Direction::Up),
            KeyCode::S => Some(Direction::Down),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::A => Some(Direction::Left),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::D => Some(Direction::Right),
            KeyCode::Right => Some(Direction::Right),
            _ => None
        };

        input
    }
}
