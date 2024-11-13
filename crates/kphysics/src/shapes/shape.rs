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
            Shape::Circle(_) => (),
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
