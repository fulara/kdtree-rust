#[cfg(test)]
pub mod tests_utils {
    use ::kdtree::*;
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
            Point1WithId {
                dims: [x],
                id: id,
            }
        }
    }

    impl KdtreePointTrait for Point1WithId {
        fn dims(&self) -> &[f64] {
            return &self.dims;
        }
    }
}