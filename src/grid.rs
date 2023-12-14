use std::{fmt::Display, num::TryFromIntError, ops};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid(pub Vec<Vec<char>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector(pub isize, pub isize);

impl Grid {
    pub fn get_pos(&self, pos: Position) -> Option<char> {
        self.0.get(pos.1).and_then(|row| row.get(pos.0)).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Grid {
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

impl ops::Deref for Grid {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl Vector {
    pub fn manhattan_distance(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}
