use std::{ops, str::FromStr};

pub mod grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Line3D<T>(pub Point3D<T>, pub Point3D<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T>(pub T, pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T>(pub T, pub T);

impl<T> Vector<T>
where
    T: ops::Div<T, Output = T> + PartialEq + Default + Copy,
{
    pub fn slope_equals(&self, other: &Vector<T>) -> bool {
        let zero = T::default();
        (self.0 == zero && other.0 == zero)
            || (self.1 == zero && other.1 == zero)
            || (self.1 != zero && other.1 != zero && self.0 / self.1 == other.0 / other.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line<T>(pub Point<T>, pub Point<T>);

impl Line<f64> {
    pub fn vector(&self) -> Vector<f64> {
        self.1 - self.0
    }

    pub fn unit_vector(&self) -> Vector<f64> {
        let total_dist = self.0.distance(self.1);
        let factor = 1.0 / total_dist;

        let v = self.vector();
        Vector(v.0 * factor, v.1 * factor)
    }

    pub fn y_intercept_and_unit_slope(&self) -> (Point<f64>, Vector<f64>) {
        let unit_vector = self.unit_vector();
        let factor = self.0 .1 / unit_vector.1;

        (
            Point(self.0 .0 - (unit_vector.0 * factor), 0.0),
            unit_vector,
        )
    }
}

impl PartialEq for Line<f64> {
    fn eq(&self, other: &Self) -> bool {
        let (self_int, self_slope) = self.y_intercept_and_unit_slope();
        let (other_int, other_slope) = other.y_intercept_and_unit_slope();

        self_int == other_int && self_slope == other_slope
    }
}

impl<T, U> ops::Add<Vector<U>> for Point<T>
where
    T: ops::Add<U, Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Vector<U>) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> ops::Sub<Point<T>> for Point<T>
where
    T: ops::Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox<T>(pub Point<T>, pub Point<T>);

impl<T> Line<T>
where
    T: ops::Sub<T, Output = T>
        + ops::Mul<T, Output = T>
        + ops::Div<T, Output = T>
        + PartialEq<T>
        + Default
        + Copy,
{
    pub fn intersection(&self, other: &Line<T>) -> Option<Point<T>> {
        let p1 = &self.0;
        let p2 = &self.1;
        let p3 = &other.0;
        let p4 = &other.1;

        let denom = ((p1.0 - p2.0) * (p3.1 - p4.1)) - ((p1.1 - p2.1) * (p3.0 - p4.0));

        if denom == T::default() {
            return None;
        }

        let px = ((p1.0 * p2.1 - p1.1 * p2.0) * (p3.0 - p4.0)
            - (p1.0 - p2.0) * ((p3.0 * p4.1) - (p3.1 * p4.0)))
            / denom;
        let py = ((p1.0 * p2.1 - p1.1 * p2.0) * (p3.1 - p4.1)
            - (p1.1 - p2.1) * ((p3.0 * p4.1) - (p3.1 * p4.0)))
            / denom;

        Some(Point(px, py))
    }
}

impl Point<f64> {
    pub fn distance(self, other: Point<f64>) -> f64 {
        ((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2)).sqrt()
    }
}

impl<T> Point<T>
where
    T: PartialOrd + Copy + Sized,
{
    pub fn is_in_box(&self, bound: &BoundingBox<T>) -> bool {
        // let min_x = if bound.0 .0 <= bound.1 .0 {
        //     bound.0 .0
        // } else {
        //     bound.1 .0
        // };
        // let max_x = if bound.0 .0 >= bound.1 .0 {
        //     bound.0 .0
        // } else {
        //     bound.1 .0
        // };
        // let min_y = if bound.0 .1 <= bound.1 .1 {
        //     bound.0 .1
        // } else {
        //     bound.1 .1
        // };
        // let max_y = if bound.0 .1 >= bound.1 .1 {
        //     bound.0 .1
        // } else {
        //     bound.1 .1
        // };

        bound.0 .0 <= self.0 && self.0 <= bound.1 .0 && bound.0 .1 <= self.1 && self.1 <= bound.1 .1
    }
}

impl<T, U> ops::Add<Vector3D<U>> for Point3D<T>
where
    T: ops::Add<U, Output = T>,
{
    type Output = Point3D<T>;

    fn add(self, rhs: Vector3D<U>) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> FromStr for Point3D<T>
where
    T: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s
            .split(',')
            .map(|s| T::from_str(&s.trim()).map_err(|_| format!("invalid point {s}")));
        Ok(Point3D {
            x: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            y: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            z: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
        })
    }
}

impl<T> FromStr for Vector3D<T>
where
    T: FromStr,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s
            .split(',')
            .map(|s| s.trim().parse().map_err(|_| format!("invalid point {s}")));
        Ok(Vector3D {
            x: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            y: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            z: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
        })
    }
}
