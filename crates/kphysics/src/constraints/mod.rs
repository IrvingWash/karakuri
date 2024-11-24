mod constraint_base;

pub mod constraint;

mod distance_constraint;
mod penetration_constraint;

use constraint_base::ConstraintBase;
use distance_constraint::DistanceConstraint;
use penetration_constraint::PenetrationConstraint;
