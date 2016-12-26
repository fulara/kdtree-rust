#[macro_use]
extern crate bencher;
extern crate kdtree;
extern crate rand;

use bencher::Bencher;


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

impl kdtree::kdtree::KdtreePointTrait for Point2WithId {
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

#[derive(Copy, Clone, PartialEq)]
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

impl kdtree::kdtree::KdtreePointTrait for Point3WithId {
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

fn a(b: &mut Bencher) {
    let len = 1000usize;
    let mut points : Vec<Point2WithId> = vec![];
    //let mut kdtree = KdTree::new_with_capacity(3, 16);
    for id in 0..len {
        let x : f64 = rand::random();
        points.push(Point2WithId::new(id as i32, x, x));
        //   points.push(rand_data());
    }

    b.iter(|| {
        let tree = kdtree::kdtree::Kdtree::new(points.clone());
    });
}

fn b(b: &mut Bencher) {
    let len = 1000usize;
    let mut points : Vec<Point3WithId> = vec![];
    //let mut kdtree = KdTree::new_with_capacity(3, 16);

    for i in 0..len {
        points.push(Point3WithId::new(i as i32, rand::random(),rand::random(),rand::random()))

    }

    let tree = kdtree::kdtree::Kdtree::new(points.clone());


    b.iter(||  tree.nearest_search(&points[0]));
}

benchmark_group!(benches, a,b);
benchmark_main!(benches);