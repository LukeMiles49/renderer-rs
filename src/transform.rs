use {
	sized_matrix::{
		Matrix,
		Vector,
	},
};

pub type Transform = Matrix<f32, 4, 4>;

pub fn scale_2d(scale_factor: f32) -> Transform {
	Matrix::rows([
		[scale_factor, 0.0, 0.0, 0.0],
		[0.0, scale_factor, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn scale_3d(scale_factor: f32) -> Transform {
	Matrix::rows([
		[scale_factor, 0.0, 0.0, 0.0],
		[0.0, scale_factor, 0.0, 0.0],
		[0.0, 0.0, scale_factor, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn scale_x(scale_factor: f32) -> Transform {
	Matrix::rows([
		[scale_factor, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn scale_y(scale_factor: f32) -> Transform {
	Matrix::rows([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, scale_factor, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn scale_z(scale_factor: f32) -> Transform {
	Matrix::rows([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, scale_factor, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn translate(offset: Vector<f32, 3>) -> Transform {
	Matrix::rows([
		[1.0, 0.0, 0.0, offset[0]],
		[0.0, 1.0, 0.0, offset[1]],
		[0.0, 0.0, 1.0, offset[2]],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_x(radians: f32) -> Transform {
	Matrix::rows([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, f32::cos(radians), -f32::sin(radians), 0.0],
		[0.0, f32::sin(radians), f32::cos(radians), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_y(radians: f32) -> Transform {
	Matrix::rows([
		[f32::cos(radians), 0.0, f32::sin(radians), 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[-f32::sin(radians), 0.0, f32::cos(radians), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_z(radians: f32) -> Transform {
	Matrix::rows([
		[f32::cos(radians), -f32::sin(radians), 0.0, 0.0],
		[f32::sin(radians), f32::cos(radians), 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}
