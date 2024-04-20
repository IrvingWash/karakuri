pub use core::InputResult;
pub use engine::Engine;
pub use scene::{ComponentsPayload, Spawner};

pub mod components;
mod engine;
pub mod math;
pub mod utils;

use entity::Entity;

mod core;
mod entity;
mod scene;
