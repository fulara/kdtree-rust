#[macro_use] extern crate bencher;
extern crate kdtree;
extern crate rand;

use bencher::Bencher;
use rand::Rng;


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

fn gen_random() -> f64 {
    rand::thread_rng().gen_range(0., 10000.)
}

fn generate_points(point_count : usize) -> Vec<Point3WithId> {
    let mut points : Vec<Point3WithId> = vec![];

    for i in 0 .. point_count {
        points.push(Point3WithId::new(i as i32, gen_random(),gen_random(),gen_random()));
    }

    points
}

fn bench_creating_1000_node_tree(b: &mut Bencher) {
    let len = 1000usize;
    let mut points = generate_points(len);

    b.iter(|| {
        kdtree::kdtree::Kdtree::new(&mut points.clone());
    });
}

fn bench_single_loop_times_for_1000_node_tree(b: &mut Bencher) {
    let len = 1000usize;
    let mut points = generate_points(len);

    let tree = kdtree::kdtree::Kdtree::new(&mut points.clone());


    b.iter(|| tree.nearest_search(&points[0]));
}

fn bench_creating_1000_000_node_tree(b: &mut Bencher) {
    let len = 1000_000usize;
    let mut points = generate_points(len);

    b.iter(|| {
        kdtree::kdtree::Kdtree::new(&mut points.clone());
    });
}

fn bench_adding_to_1000_tree(b: &mut Bencher) {
    let len = 1000usize;
    let mut points = generate_points(len);
    let mut tree = kdtree::kdtree::Kdtree::new(&mut points);

    let point = Point3WithId::new(-1 as i32, gen_random(),gen_random(),gen_random());
    println!("before ..");
    b.iter(|| {
        println!("in lam ..");
        tree.insert_node(point);
    });
}



benchmark_group!(benches, bench_creating_1000_node_tree,bench_single_loop_times_for_1000_node_tree, bench_adding_to_1000_tree);
benchmark_main!(benches);