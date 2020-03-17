use kdtree::KdtreePointTrait;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
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

use crate::distance::euclidean;
use crate::distance::squared_euclidean;
use crate::Kdtree;
#[test]
fn doc_test() {
    let a = Point3WithId {
        dims: [0.0, 0.0, 0.0],
        id: 0,
    };
    let b = Point3WithId {
        dims: [1.0, 0.0, 0.0],
        id: 1,
    };
    let c = Point3WithId {
        dims: [1.0, 1.0, 0.0],
        id: 2,
    };
    let d = Point3WithId {
        dims: [1.0, 1.0, 1.0],
        id: 3,
    };
    let mut pts = vec![a, b, c, d];
    let tree = Kdtree::new(&mut pts);

    // Query the same point
    assert_eq!(tree.nearest_search(&a).id, a.id);

    assert_eq!(tree.within(&a, 0.001, euclidean).len(), 1);
    assert_eq!(tree.within(&a, 1.001, euclidean).len(), 2);
    assert_eq!(tree.within(&a, 1.415, euclidean).len(), 3);
    assert_eq!(tree.within(&a, 1.733, euclidean).len(), 4);
}
