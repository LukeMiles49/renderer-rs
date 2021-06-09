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
	image::{
		GenericImage,
	},
};

pub enum RasterisingRenderer { }

impl RasterisingRenderer {
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
		depth_buffer: &mut [[f32; WW]; WH],
	) {
		let transform: Matrix<f32, 3, 3> = camera.section([0, 0]);
		let translate: Vector<f32, 3> = camera.section([0, 3]);
		
		let view = GridMesh {
			vertices: mesh.vertices.map(|row| row.map(|mut vertex| {
				// Apply transformation to screen space
				vertex = transform * vertex + translate;
				
				// Set to reciprocal of depth to preserve linearity of depth buffer
				vertex[2] = 1.0 / vertex[2];
				
				// Apply perspective transformation
				vertex[0] *= vertex[2];
				vertex[1] *= vertex[2];
				
				vertex
			})),
			colours: mesh.colours,
		};
		
		for (triangle, colour) in view {
			RasterisingRenderer::render_triangle(triangle, colour, bitmap, depth_buffer);
		}
	}
	
	pub fn render_triangle<
		B: GenericImage,
		const WW: usize,
		const WH: usize,
	>(
		triangle: Triangle,
		colour: B::Pixel,
		bitmap: &mut B,
		depth_buffer: &mut [[f32; WW]; WH],
	) {
		let (mut a, mut b, mut c) = triangle;
		
		let depth_step = {
			let denominator = (c[1] - a[1]) * (b[0] - a[0]) - (b[1] - a[1]) * (c[0] - a[0]);
			if denominator == 0.0 { 0.0 }
			else { ((c[1] - a[1]) * (b[2] - a[2]) - (b[1] - a[1]) * (c[2] - a[2])) / denominator }
		};
		
		// Sort by y coordinate
		if b[1] < a[1] { (a, b) = (b, a); }
		if c[1] < a[1] { (a, c) = (c, a); }
		if c[1] < b[1] { (b, c) = (c, b); }
		
		if b[1].ceil() > a[1].ceil() {
			let b_step = (b[0] - a[0]) / (b[1] - a[1]);
			let c_step = (c[0] - a[0]) / (c[1] - a[1]);
			
			// Select left and right arms for first half
			let (left, left_step, right, right_step) =
				if b_step < c_step { (b, b_step, c, c_step) }
				else { (c, c_step, b, b_step) };
			
			let left_depth_step = (left[2] - a[2]) / (left[1] - a[1]);
			
			let mut left_x = a[0] + left_step * (a[1].ceil() - a[1]);
			let mut right_x = a[0] + right_step * (a[1].ceil() - a[1]);
			let mut left_depth = a[2] + left_depth_step * (a[1].ceil() - a[1]);
			
			for y in a[1].ceil() as i32..b[1].ceil() as i32 {
				let mut depth = left_depth + depth_step * (left_x.ceil() - left_x);
				
				for x in left_x.ceil() as i32..=right_x.floor() as i32 {
					let screen_x = x + WW as i32 / 2;
					let screen_y = y + WH as i32 / 2;
					
					// Draw (x, y)
					if 0 <= screen_x && screen_x < WW as i32
						&& 0 <= screen_y && screen_y < WH as i32
						&& depth > depth_buffer[screen_y as usize][screen_x as usize]
					{
						bitmap.put_pixel(screen_x as u32, screen_y as u32, colour);
						depth_buffer[screen_y as usize][screen_x as usize] = depth;
					}
					
					depth += depth_step;
				}
				
				left_x += left_step;
				right_x += right_step;
				left_depth += left_depth_step;
			}
		}
		
		if c[1].floor() >= b[1].ceil() {
			let a_step = (c[0] - a[0]) / (c[1] - a[1]);
			let b_step = (c[0] - b[0]) / (c[1] - b[1]);
			
			// Select left and right arms for second half
			let (left, left_step, right, right_step) =
				if a_step > b_step { (a, a_step, b, b_step) }
				else { (b, b_step, a, a_step) };
			
			let left_depth_step = (c[2] - left[2]) / (c[1] - left[1]);
			
			let mut left_x = left[0] + left_step * (b[1].ceil() - left[1]);
			let mut right_x = right[0] + right_step * (b[1].ceil() - right[1]);
			let mut left_depth = left[2] + left_depth_step * (b[1].ceil() - left[1]);
			
			for y in b[1].ceil() as i32..=c[1].floor() as i32 {
				let mut depth = left_depth + depth_step * (left_x.ceil() - left_x);
				
				for x in left_x.ceil() as i32..=right_x.floor() as i32 {
					let screen_x = x + WW as i32 / 2;
					let screen_y = y + WH as i32 / 2;
					
					// Draw (x, y)
					if 0 <= screen_x && screen_x < WW as i32
						&& 0 <= screen_y && screen_y < WH as i32
						&& depth > depth_buffer[screen_y as usize][screen_x as usize]
					{
						bitmap.put_pixel(screen_x as u32, screen_y as u32, colour);
						depth_buffer[screen_y as usize][screen_x as usize] = depth;
					}
					
					depth += depth_step;
				}
				
				left_x += left_step;
				right_x += right_step;
				left_depth += left_depth_step;
			}
		}
	}
}

/*
impl Renderer<GridMesh<MW, MH>> for RasterisingRenderer {
	fn render<B: GenericImage>(model: &GridMesh<MW, MH>, camera: Transform, bitmap: &mut B) {
		
	}
}
*/
