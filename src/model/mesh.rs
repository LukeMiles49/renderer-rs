use {
	super::{
		Vertex,
		Triangle,
	},
};

#[derive(Clone, Copy)]
pub struct GridMesh<C: Copy, const W: usize, const H: usize> {
	pub vertices: [[Vertex; W]; H],
	// TODO: Decrease size by 1 in each direction once expressions are allowed in array lengths
	pub colours: [[(C, C); W]; H],
}

pub struct IntoIter<C: Copy, const W: usize, const H: usize> {
	mesh: GridMesh<C, W, H>,
	x: usize,
	y: usize,
	top: bool,
}

impl<C: Copy, const W: usize, const H: usize> IntoIterator for GridMesh<C, W, H> {
	type IntoIter = IntoIter<C, W, H>;
	type Item = (Triangle, C);
	
	fn into_iter(self) -> IntoIter<C, W, H> {
		IntoIter {
			mesh: self,
			x: 0,
			y: 0,
			top: false,
		}
	}
}

impl<C: Copy, const W: usize, const H: usize> Iterator for IntoIter<C, W, H> {
	type Item = (Triangle, C);
	
	fn next(&mut self) -> Option<(Triangle, C)> {
		if self.y < H - 1 {
			if !self.top {
				let result = (
					(
						self.mesh.vertices[self.y][self.x],
						self.mesh.vertices[self.y+1][self.x+1],
						self.mesh.vertices[self.y+1][self.x],
					),
					self.mesh.colours[self.y][self.x].0,
				);
				self.top = true;
				Some(result)
			} else {
				let result = (
					(
						self.mesh.vertices[self.y][self.x],
						self.mesh.vertices[self.y][self.x+1],
						self.mesh.vertices[self.y+1][self.x+1],
					),
					self.mesh.colours[self.y][self.x].1,
				);
				if self.x < W - 2 {
					self.x += 1;
				} else {
					self.x = 0;
					self.y += 1;
				}
				self.top = false;
				Some(result)
			}
		} else {
			None
		}
	}
}
