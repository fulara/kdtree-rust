use kdtree::KdtreePointTrait;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point3WithId {
    dims: [f64; 3],
    pub id: i32,
}

impl Point3WithId {
    pub fn new(id: i32, x: f64, y: f64, z: f64) -> Point3WithId {
        Point3WithId {
            dims: [x, y, z],
            id: id,
        }
    }
}

impl KdtreePointTrait for Point3WithId {
    #[inline]
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point2WithId {
    dims: [f64; 2],
    pub id: i32,
}

impl Point2WithId {
    pub fn new(id: i32, x: f64, y: f64) -> Point2WithId {
        Point2WithId {
            dims: [x, y],
            id: id,
        }
    }
}

impl KdtreePointTrait for Point2WithId {
    #[inline]
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point1WithId {
    dims: [f64; 1],
    pub id: i32,
}

impl Point1WithId {
    pub fn new(id: i32, x: f64) -> Point1WithId {
        Point1WithId { dims: [x], id: id }
    }
}

impl KdtreePointTrait for Point1WithId {
    #[inline]
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}