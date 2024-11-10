use super::{Circle, Polygon, Rectangle};

#[derive(Debug)]
pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
    Rectangle(Rectangle),
}

impl Shape {
    pub fn moment_of_inertia(&self) -> f64 {
        match self {
            Shape::Circle(_) => Circle::MOMENT_OF_INERTIA,
            Shape::Polygon(_) => Polygon::MOMENT_OF_INERTIA,
            Self::Rectangle(_) => Rectangle::MOMENT_OF_INERTIA,
        }
    }

    pub fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }

    pub fn is_polygon(&self) -> bool {
        matches!(self, Shape::Polygon(_))
    }

    pub fn is_rectangle(&self) -> bool {
        matches!(self, Shape::Rectangle(_))
    }

    pub fn circle(&self) -> Option<&Circle> {
        if let Shape::Circle(circle) = &self {
            return Some(circle);
        }

        None
    }

    pub fn polygon(&self) -> Option<&Polygon> {
        if let Shape::Polygon(polygon) = &self {
            return Some(polygon);
        }

        None
    }

    pub fn rectangle(&self) -> Option<&Rectangle> {
        if let Shape::Rectangle(rectangle) = &self {
            return Some(rectangle);
        }

        None
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}
