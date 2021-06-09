use {
	crate::{
		model::{
			GridMesh,
			Triangle,
		},
		transform::{
			Transform,
		},
		Renderer,
	},
	sized_matrix::{
		Matrix,
		Vector,
	},
	higher_order_functions::{
		Section,
	},
	num_traits::{
		ops::{
			inv::{
				Inv,
			},
		},
	},
	image::{
		GenericImage,
	},
};

pub enum RayTracingRenderer { }

impl RayTracingRenderer {
	pub fn render_grid_mesh<
		B: GenericImage,
		const MW: usize,
		const MH: usize,
		const WW: usize,
		const WH: usize,
	>(
		mesh: &GridMesh<B::Pixel, MW, MH>,
		camera: Transform,
		bitmap: &mut B,
		black: B::Pixel,
	) {
		let transform: Matrix<f32, 3, 3> = camera.section([0, 0]).inv();
		let translate: Vector<f32, 3> = -(transform * camera.section([0, 3]));
		
		for y in 0..WH {
			let camera_y = y as f32 - (WH - 1) as f32 / 2.0;
			for x in 0..WW {
				let camera_x = x as f32 - (WW - 1) as f32 / 2.0;
				let ray_dir = transform * Vector::vector([camera_x, camera_y, 1.0]);
				let colour = RayTracingRenderer::ray_colour(mesh, translate, ray_dir);
				bitmap.put_pixel(x as u32, y as u32, colour.unwrap_or(black));
			}
		}
	}
	
	pub fn ray_colour<
		P: Copy,
		const MW: usize,
		const MH: usize,
	>(
		mesh: &GridMesh<P, MW, MH>,
		ray_loc: Vector<f32, 3>,
		ray_dir: Vector<f32, 3>,
	) -> Option<P> {
		let mut min_distance = f32::INFINITY;
		let mut cur_colour = None;
		
		for ((a, b, c), colour) in *mesh {
			let result = (ray_loc - a).div_left(Matrix::from_vectors([b - a, c - a, ray_dir]));
			if result[0] >= 0.0 &&
				result[1] >= 0.0 &&
				result[0] + result[1] <= 1.0 &&
				-result[2] < min_distance
			{
				min_distance = -result[2];
				cur_colour = Some(colour);
			}
		}
		
		cur_colour
	}
}
