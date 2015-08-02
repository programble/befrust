extern crate rand;

use std::ops;
use super::consts::{WIDTH, HEIGHT};
use self::rand::Rng;

/// Small 2D point for addressing the program space.
///
/// # Examples
///
/// ```
/// use befrust::Point;
///
/// let a = Point {x: 1, y: 1};
/// let b = Point {x: -1, y: 1};
///
/// assert_eq!(Point {x: 0, y: 2}, a + b);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

/// Program counter, with position and direction.
///
/// # Examples
///
/// ```
/// use befrust::{Point, Pc};
///
/// let mut pc = Pc::new();
/// pc.step();
///
/// assert_eq!(Point {x: 1, y: 0}, pc.pos);
/// ```
#[derive(Debug)]
pub struct Pc {
    pub pos: Point,
    dir: Point,
}

impl Pc {
    /// Creates a program counter at the top left corner of program space, heading right.
    pub fn new() -> Pc {
        Pc {pos: Point {x: 0, y: 0}, dir: Point {x: 1, y: 0}}
    }

    /// Steps the program counter forward in the current direction, wrapping around the program
    /// torus space.
    ///
    /// # Examples
    ///
    /// ```
    /// use befrust::{Point, Pc, consts};
    ///
    /// let mut pc = Pc::new();
    ///
    /// pc.left();
    /// pc.step();
    ///
    /// assert_eq!(Point {x: consts::WIDTH - 1, y: 0}, pc.pos);
    ///
    /// pc.right();
    /// pc.step();
    ///
    /// assert_eq!(Point {x: 0, y: 0}, pc.pos);
    /// ```
    ///
    /// ```
    /// use befrust::{Point, Pc, consts};
    ///
    /// let mut pc = Pc::new();
    ///
    /// pc.up();
    /// pc.step();
    ///
    /// assert_eq!(Point {x: 0, y: consts::HEIGHT - 1}, pc.pos);
    ///
    /// pc.down();
    /// pc.step();
    ///
    /// assert_eq!(Point {x: 0, y: 0}, pc.pos);
    /// ```
    pub fn step(&mut self) {
        self.pos = self.pos + self.dir;
        self.pos.x = (self.pos.x + WIDTH) % WIDTH;
        self.pos.y = (self.pos.y + HEIGHT) % HEIGHT;
    }

    /// Sets the direction to right.
    pub fn right(&mut self) {
        self.dir = Point {x: 1, y: 0}
    }

    /// Sets the direction to left.
    pub fn left(&mut self) {
        self.dir = Point {x: -1, y: 0}
    }

    /// Sets the direction to down.
    pub fn down(&mut self) {
        self.dir = Point {x: 0, y: 1}
    }

    /// Sets the direction to up.
    pub fn up(&mut self) {
        self.dir = Point {x: 0, y: -1}
    }

    /// Sets the direction randomly.
    pub fn rand(&mut self) {
        let mut rng = rand::thread_rng();
        let dirs = [
            Point {x:  1, y:  0},
            Point {x: -1, y:  0},
            Point {x:  0, y:  1},
            Point {x:  0, y: -1},
        ];
        self.dir = *rng.choose(&dirs).unwrap();
    }
}
