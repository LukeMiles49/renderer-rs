#![feature(destructuring_assignment)]
#![feature(array_map)]

pub mod rasterising;
pub mod model;
pub mod transform;

mod renderer;
pub use {
	renderer::{
		Renderer,
	},
};
