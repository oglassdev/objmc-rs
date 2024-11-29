#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Vector<T : Copy> {
    pub x: T,
    pub y: T
}

impl<T : Copy> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Position<T : Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T : Copy> Position<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertex_normals: Vec<(u32, u32)>,
}