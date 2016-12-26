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

benchmark_group!(benches, a);
benchmark_main!(benches);