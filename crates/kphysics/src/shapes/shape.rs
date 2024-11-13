use kmath::Vector2;

use super::{Circle, Polygon};

#[derive(Debug)]
pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}

impl Shape {
    #[inline]
    pub fn moment_of_inertia(&self) -> f64 {
        match self {
            Self::Circle(circle) => circle.moment_of_inertia(),
            Self::Polygon(polygon) => polygon.moment_of_inertia(),
        }
    }

    #[inline]
    pub fn is_circle(&self) -> bool {
        matches!(self, Self::Circle(_))
    }

    #[inline]
    pub fn is_polygon(&self) -> bool {
        matches!(self, Self::Polygon(_))
    }

    #[inline]
    pub fn circle(&self) -> Option<&Circle> {
        if let Self::Circle(circle) = &self {
            return Some(circle);
        }

        None
    }

    #[inline]
    pub fn polygon(&self) -> Option<&Polygon> {
        if let Self::Polygon(polygon) = &self {
            return Some(polygon);
        }

        None
    }

    #[inline]
    pub fn update_vertices(&mut self, position: &Vector2, rotation: f64) {
        match self {
            Shape::Circle(_) => klogger::terminate("Trying to update vertices on a `Circle`."),
            Shape::Polygon(polygon) => polygon.update_vertices(position, rotation),
        }
    }
}

impl From<Circle> for Shape {
    #[inline]
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl From<Polygon> for Shape {
    #[inline]
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

#[cfg(test)]
mod shape_tests {
    use kmath::Vector2;

    use crate::shapes::{Circle, Polygon};

    use super::Shape;

    #[test]
    fn test_is_shape() {
        let shape = Shape::Circle(Circle::new(1.0));

        assert!(shape.is_circle());
        assert!(!shape.is_polygon());

        let shape = Shape::Polygon(Polygon::new(vec![
            Vector2::ZERO,
            Vector2::ZERO,
            Vector2::ZERO,
        ]));

        assert!(shape.is_polygon());
        assert!(!shape.is_circle());
    }

    #[test]
    fn test_accessors() {
        let shape = Shape::Circle(Circle::new(1.0));

        assert!(shape.circle().is_some());
        assert!(shape.polygon().is_none());

        let shape = Shape::Polygon(Polygon::new(vec![
            Vector2::ZERO,
            Vector2::ZERO,
            Vector2::ZERO,
        ]));

        assert!(shape.polygon().is_some());
        assert!(shape.circle().is_none());
    }

    #[test]
    #[should_panic]
    fn test_circle_update_world_vertices() {
        let mut shape = Shape::Circle(Circle::new(1.0));

        shape.update_vertices(&Vector2::new(10.0, 10.0), 3.0);
    }

    #[test]
    fn test_polygon_update_world_vertices() {
        let mut shape = Shape::Polygon(Polygon::new(vec![
            Vector2::new(5.0, 0.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(0.0, 10.0),
        ]));

        shape.update_vertices(&Vector2::new(10.0, 10.0), 3.0);

        assert_eq!(
            shape.polygon().unwrap().world_vertices,
            vec![
                Vector2 {
                    x: 5.050037516997773,
                    y: 10.705600040299336
                },
                Vector2 {
                    x: -1.3111250466031255,
                    y: 1.511275114594218
                },
                Vector2 {
                    x: 8.588799919401328,
                    y: 0.10007503399554629
                }
            ]
        );
    }
}
