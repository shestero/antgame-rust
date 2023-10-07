use crate::field::*;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}
#[derive(Copy, Clone)]
pub struct AntState {
    pub x: i16,
    pub y: i16,
    d: Direction,
}

impl AntState {
    pub fn start() -> AntState {
        AntState {
            x: 511,
            y: 511,
            d: Direction::Top,
        }
    }

    // assume Y axis points down
    fn forward(&self) -> AntState {
        let (xx, yy) = match self.d {
            Direction::Top => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Bottom => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        };
        AntState {
            x: xx,
            y: yy,
            d: self.d,
        }
    }

    fn clockwise(&self) -> AntState {
        AntState {
            x: self.x,
            y: self.y,
            d: (match self.d {
                Direction::Top => Direction::Right,
                Direction::Right => Direction::Bottom,
                Direction::Bottom => Direction::Left,
                Direction::Left => Direction::Top,
            }),
        }
    }

    fn contra_clockwise(&self) -> AntState {
        AntState {
            x: self.x,
            y: self.y,
            d: (match self.d {
                Direction::Left => Direction::Bottom,
                Direction::Bottom => Direction::Right,
                Direction::Right => Direction::Top,
                Direction::Top => Direction::Left,
            }),
        }
    }

    pub fn step(&self, color: Color) -> AntState {
        (match color {
            Color::White => self.clockwise(),
            Color::Black => self.contra_clockwise(),
        })
        .forward()
    }

    pub fn is_in_field(&self) -> bool {
        0 <= self.x && 0 <= self.y && (self.x as usize) < Field::W && (self.y as usize) < Field::H
    }
}
