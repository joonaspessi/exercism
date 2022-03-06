// The code below is a stub. Just enough to satisfy the compiler.
#[ignore]
// In order to pass the tests you can add-to or change any of this code.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_dir = match self.direction() {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            direction: new_dir,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_dir = match self.direction() {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            direction: new_dir,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (dx, dy) = match self.direction() {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        Self {
            x: self.x + dx,
            y: self.y + dy,
            direction: self.direction,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut temp = self;
        for command in instructions.chars() {
            temp = match command {
                'R' => temp.turn_right(),
                'L' => temp.turn_left(),
                'A' => temp.advance(),
                _ => panic!("should not happen!"),
            };
        }
        temp
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
