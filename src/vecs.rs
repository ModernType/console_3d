use std::ops::{Add, Sub, Mul, Div};

trait Vector {
    fn dot(&self, other: Self) -> f64;
    fn length(&self) -> f64;
}

#[derive(Debug, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector for Vector2 {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2{ x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add<f64> for Vector2 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vector2 {x: self.x + rhs, y: self.y + rhs}
    }
} 

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2{ x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Sub<f64> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Vector2 {x: self.x - rhs, y: self.y - rhs}
    }
} 

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2{ x: self.x * rhs.x, y: self.y * rhs.y}
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {x: self.x * rhs, y: self.y * rhs}
    }
} 

impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2{ x: self.x / rhs.x, y: self.y / rhs.y}
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2 {x: self.x / rhs, y: self.y / rhs}
    }
} 

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 1.0, y: 1.0 }
    }
}


#[derive(Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector for Vector3 {
    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<f64> for Vector3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vector3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub<f64> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Vector3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 { x: 1.0, y: 1.0, z: 1.0 }
    }
}


#[cfg(test)]
mod tests {
    use crate::vecs::*;

    #[test]
    fn test_vec_fn() {
        assert_eq!(Vector3 {x: 5.0, y: 4.0, z: 3.0} * Vector3 {x: 1.0, y: 1.0, z: 3.0}, Vector3 { x: 5.0, y: 4.0, z: 9.0})
    }
}
