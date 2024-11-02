mod adapters;
mod animator;
mod errors;
mod game;
mod game_config;
mod scene;

use adapters::InputProcessorWrapper;
use adapters::RendererAdapter;
use animator::Animator;
use scene::Scene;

pub mod components;

pub use game::Game;
pub use game_config::GameConfig;
pub use kec as ec;
pub use kmath as math;
pub use kutils as utils;
pub use kwindow as window;
