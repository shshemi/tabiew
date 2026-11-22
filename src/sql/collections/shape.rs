#[derive(Debug, Clone, Copy)]
pub struct Shape<const DIMS: usize> {
    dims: [usize; DIMS],
}

impl<const D: usize> Shape<D> {
    pub fn numel(&self) -> usize {
        self.dims.iter().product()
    }
}

impl Shape<1> {
    pub fn new_1d(dim: usize) -> Self {
        Self { dims: [dim] }
    }
}

impl Shape<2> {
    pub fn new_2d(rows: usize, cols: usize) -> Self {
        Self { dims: [rows, cols] }
    }

    pub fn rows(&self) -> usize {
        self.dims[0]
    }

    pub fn cols(&self) -> usize {
        self.dims[1]
    }
}
