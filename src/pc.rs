//! Program counter.

extern crate rand;

use consts;
use self::rand::Rng;

/// Possible directions of program counter.
#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

/// Program counter, with position and direction.
///
/// # Examples
///
/// ```
/// use befrust::pc::Pc;
///
/// let mut pc = Pc::new();
/// pc.step();
///
/// assert_eq!((1, 0), (pc.x, pc.y));
/// ```
pub struct Pc {
    pub x: usize,
    pub y: usize,
    dir: Direction,
}

impl Pc {
    /// Creates a new program counter in the top-left corner, heading right.
    pub fn new() -> Pc {
        Pc {x: 0, y: 0, dir: Direction::Right}
    }

    /// Steps the program counter forward in current direction.
    pub fn step(&mut self) {
        match self.dir {
            Direction::Right   => self.x = (self.x + 1) % consts::WIDTH,
            Direction::Left
                if self.x == 0 => self.x = consts::WIDTH - 1,
            Direction::Left    => self.x -= 1,
            Direction::Down    => self.y = (self.y + 1) % consts::HEIGHT,
            Direction::Up
                if self.y == 0 => self.y = consts::HEIGHT - 1,
            Direction::Up      => self.y -= 1,
        }
    }

    /// Sets direction to right.
    pub fn right(&mut self) {
        self.dir = Direction::Right;
    }

    /// Sets direction to left.
    pub fn left(&mut self) {
        self.dir = Direction::Left;
    }

    /// Sets direction to down.
    pub fn down(&mut self) {
        self.dir = Direction::Down;
    }

    /// Sets direction to up.
    pub fn up(&mut self) {
        self.dir = Direction::Up;
    }

    /// Sets direction randomly.
    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        let dirs = [
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up
        ];
        self.dir = *rng.choose(&dirs).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use consts;
    use super::*;

    #[test]
    fn step_x_wrap() {
        let mut pc = Pc::new();

        pc.left();
        pc.step();
        assert_eq!(pc.x, consts::WIDTH - 1);

        pc.right();
        pc.step();
        assert_eq!(pc.x, 0);
    }

    #[test]
    fn step_y_wrap() {
        let mut pc = Pc::new();

        pc.up();
        pc.step();
        assert_eq!(pc.y, consts::HEIGHT - 1);

        pc.down();
        pc.step();
        assert_eq!(pc.y, 0);
    }
}
