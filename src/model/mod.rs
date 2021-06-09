use {
	sized_matrix::{
		Vector,
	},
};

mod mesh;
pub use mesh::GridMesh;

pub type Vertex = Vector<f32, 3>;

pub type Triangle = (Vertex, Vertex, Vertex);
