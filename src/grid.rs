use std::{fmt::Display, num::TryFromIntError, ops};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SignedPosition(pub isize, pub isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(pub isize, pub isize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Actor {
    pub pos: Position,
    pub vector: Vector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelativeDirection {
    Forward,
    Backward,
    Left,
    Right,
}

impl Actor {
    pub fn do_move(&mut self) -> Result<(), TryFromIntError> {
        self.pos = (self.pos + self.vector)?;

        Ok(())
    }

    pub fn turn(&mut self, rel_dir: RelativeDirection) {
        if let Some(dir) = self.vector.direction() {
            let magnitude = self.vector.manhattan_distance();
            self.vector = dir.turned(rel_dir).unit_vector() * magnitude;
        } else {
            unimplemented!("only cardinal vectors support turning at the moment")
        }
    }
}

impl Vector {
    pub fn manhattan_distance(&self) -> isize {
        self.0.abs() + self.1.abs()
    }

    pub fn direction(&self) -> Option<Direction> {
        if self.0 > 0 && self.1 == 0 {
            Some(Direction::East)
        } else if self.0 < 0 && self.1 == 0 {
            Some(Direction::West)
        } else if self.0 == 0 && self.1 < 0 {
            Some(Direction::North)
        } else if self.0 == 0 && self.1 > 0 {
            Some(Direction::South)
        } else {
            None
        }
    }
}

impl Direction {
    pub fn all() -> &'static [Direction; 4] {
        &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    pub fn unit_vector(&self) -> Vector {
        match self {
            Direction::North => Vector(0, -1),
            Direction::South => Vector(0, 1),
            Direction::East => Vector(1, 0),
            Direction::West => Vector(-1, 0),
        }
    }

    pub fn turned(self, direction: RelativeDirection) -> Self {
        match direction {
            RelativeDirection::Forward => self,
            RelativeDirection::Backward => match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            },
            RelativeDirection::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            RelativeDirection::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
        }
    }
}

impl<T> Grid<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Grid<T> {
        Grid(inner)
    }

    pub fn get_pos(&self, pos: Position) -> Option<&T> {
        self.0.get(pos.1).and_then(|row| row.get(pos.0))
    }

    pub fn get_pos_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.0.get_mut(pos.1).and_then(|row| row.get_mut(pos.0))
    }

    pub fn wrapped_position(&self, signed_pos: SignedPosition) -> Position {
        let y = (signed_pos.1.rem_euclid(self.len() as isize)) as usize;
        let x = (signed_pos.0.rem_euclid(self[y].len() as isize)) as usize;
        Position(x, y)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.iter().enumerate() {
            for cell in row.iter() {
                write!(f, "{}", *cell)?;
            }
            if i != self.len() - 1 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

impl<T> ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Position {
    pub fn to_signed(&self) -> SignedPosition {
        SignedPosition(self.0 as isize, self.1 as isize)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Vector;

    fn sub(self, rhs: Position) -> Self::Output {
        Vector(
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}

impl ops::Add<Vector> for Position {
    type Output = Result<Position, TryFromIntError>;

    fn add(self, rhs: Vector) -> Self::Output {
        let x = usize::try_from(self.0 as isize + rhs.0)?;
        let y = usize::try_from(self.1 as isize + rhs.1)?;
        Ok(Position(x, y))
    }
}

impl ops::Sub<Vector> for Position {
    type Output = Result<Position, TryFromIntError>;

    fn sub(self, rhs: Vector) -> Self::Output {
        let x = usize::try_from(self.0 as isize - rhs.0)?;
        let y = usize::try_from(self.1 as isize - rhs.1)?;
        Ok(Position(x, y))
    }
}

impl ops::Sub<SignedPosition> for SignedPosition {
    type Output = Vector;

    fn sub(self, rhs: SignedPosition) -> Self::Output {
        Vector(
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}

impl ops::Add<Vector> for SignedPosition {
    type Output = SignedPosition;

    fn add(self, rhs: Vector) -> Self::Output {
        let x = self.0 as isize + rhs.0;
        let y = self.1 as isize + rhs.1;
        SignedPosition(x, y)
    }
}

impl ops::Sub<Vector> for SignedPosition {
    type Output = SignedPosition;

    fn sub(self, rhs: Vector) -> Self::Output {
        let x = self.0 as isize - rhs.0;
        let y = self.1 as isize - rhs.1;
        SignedPosition(x, y)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}
