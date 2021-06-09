use {
	crate::{
		transform::{
			Transform,
		},
	},
	image::{
		GenericImage,
	},
};

pub trait Renderer<M> {
	fn render<B: GenericImage>(model: &M, camera: Transform, bitmap: &mut B);
}
