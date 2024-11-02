mod errors;
mod game;
mod game_config;
mod input_processor_wrapper;
mod renderer_adapter;
mod scene;

use input_processor_wrapper::InputProcessorWrapper;
use renderer_adapter::RendererAdapter;
use scene::Scene;

pub mod components;

pub use game::Game;
pub use game_config::GameConfig;
pub use kec as ec;
pub use kmath as math;
pub use kutils as utils;
pub use kwindow as window;
