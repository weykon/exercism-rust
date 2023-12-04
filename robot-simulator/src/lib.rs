// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        use Direction::*;
        self.dir = match self.dir {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        use Direction::*;
        self.dir = match self.dir {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let old_pos = self.position();
        use Direction::*;
        self.pos = match self.dir {
            North => (old_pos.0, old_pos.1 + 1),
            East => (self.position().0 + 1, self.position().1),
            South => (self.position().0, self.position().1 - 1),
            West => (self.position().0 - 1, self.position().1),
        };
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'A' => robot.advance(),
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
