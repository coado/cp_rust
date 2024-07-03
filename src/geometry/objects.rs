use std::cmp::Ordering;

const EPS: f64 = 1e-9;

#[derive(Debug, Clone, Default, Copy)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &f64 {
        &self.x
    }

    pub fn y(&self) -> &f64 {
        &self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn rotate(&mut self, angle: f64) {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        self.x = x;
        self.y = y;
    }

    pub fn translate(&mut self, vector: &Vector2D) {
        self.x += vector.x();
        self.y += vector.y();
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x - other.x).abs() < EPS {
            if (self.y - other.y).abs() < EPS {
                Some(Ordering::Equal)
            } else if self.y < other.y {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else if self.x < other.x {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]

pub struct Line2D {
    a: f64,
    b: f64,
    c: f64,
}

impl Line2D {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn from_points(p1: &Point2D, p2: &Point2D) -> Self {
        if (p1.x - p2.x).abs() < EPS {
            return Self::new(1.0, 0.0, -p1.x);
        }

        let a = -(p1.y - p2.y) / (p1.x - p2.x);
        let b = 1.0;
        let c = -(a * p1.x - p1.y);

        Self::new(a, b, c)
    }

    pub fn a(&self) -> &f64 {
        &self.a
    }

    pub fn b(&self) -> &f64 {
        &self.b
    }

    pub fn c(&self) -> &f64 {
        &self.c
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        (self.a - other.a).abs() < EPS && (self.b - other.b).abs() < EPS
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.is_parallel(other) && (self.c - other.c).abs() < EPS
    }

    pub fn intersection(&self, other: &Self) -> Option<Point2D> {
        if self.is_parallel(other) {
            return None;
        }

        let x = (other.b * self.c - self.b * other.c) / (other.a * self.b - self.a * other.b);
        let y = if self.b.abs() > EPS {
            -(self.a * x + self.c) / self.b
        } else {
            -(other.a * x + other.c) / other.b
        };

        Some(Point2D::new(x, y))
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_points(p1: &Point2D, p2: &Point2D) -> Self {
        Self::new(p2.x - p1.x, p2.y - p1.y)
    }

    pub fn x(&self) -> &f64 {
        &self.x
    }

    pub fn y(&self) -> &f64 {
        &self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn scale(&self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let mut angle = self.dot(other) / (self.norm() * other.norm());
        if angle > 1.0 {
            angle = 1.0;
        } else if angle < -1.0 {
            angle = -1.0;
        }

        angle.acos()
    }

    pub fn rotate(&mut self, angle: f64) {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        self.x = x;
        self.y = y;
    }

    pub fn ccw(p: &Point2D, q: &Point2D, r: &Point2D) -> bool {
        let pq = Vector2D::from_points(p, q);
        let pr = Vector2D::from_points(p, r);
        pq.cross(&pr) > EPS
    }

    pub fn collinear(p: &Point2D, q: &Point2D, r: &Point2D) -> bool {
        let pq = Vector2D::from_points(p, q);
        let pr = Vector2D::from_points(p, r);
        pq.cross(&pr).abs() < EPS
    }
}
