pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn distance(&self, rhs: &Vector2) -> f64 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        (x * x + y * y).sqrt()
    }

    pub fn distance_squared(&self, rhs: &Vector2) -> f64 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        x * x + y * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approximately(lhs: f64, rhs: f64) -> bool {
        (lhs - rhs).abs() < f64::EPSILON
    }

    #[test]
    fn distance_test() {
        let a = Vector2::new(1.0, 5.0);
        let b = Vector2::new(-2.0, 1.0);

        let result = a.distance(&b);
        assert!(approximately(result, 5.0), "{result} was not equal to 5.0");

        let result = b.distance(&a);
        assert!(approximately(result, 5.0), "{result} was not equal to 5.0");
    }

    #[test]
    fn distance_squared_test() {
        let a = Vector2::new(0.0, 0.0);
        let b = Vector2::new(3.0, 4.0);

        let result = a.distance_squared(&b);
        assert!(
            approximately(result, 25.0),
            "{result} was not equal to 25.0"
        );

        let result = b.distance_squared(&a);
        assert!(
            approximately(result, 25.0),
            "{result} was not equal to 25.0"
        );
    }
}
