extern crate kdtree;
extern crate rand;

use rand::Rng;

use kdtree::kdtree::*;

//these could be taken from test_common, but I dont fully understand the module thingy yet.
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

impl KdtreePointTrait for Point3WithId {
    fn dims(&self) -> &[f64] {
        return &self.dims;
    }
}

fn gen_random() -> f64 {
    rand::thread_rng().gen_range(0., 10000.)
}

fn gen_random_usize( max_value : usize) -> usize {
    rand::thread_rng().gen_range(0usize, max_value)
}

fn find_nn_with_linear_search<'a>(points : &'a Vec<Point3WithId>, find_for : Point3WithId) -> &Point3WithId {
    let distance_fun = kdtree::kdtree::distance::squared_euclidean;

    let mut best_found_distance =  distance_fun(find_for.dims(), points[0].dims());
    let mut closed_found_point = &points[0];

    for p in points {
        let dist = distance_fun(find_for.dims(), p.dims());

        if dist < best_found_distance {
            best_found_distance = dist;
            closed_found_point = &p;
        }
    }

    closed_found_point
}

#[test]
fn test_against_1000_random_points() {
    let mut points : Vec<Point3WithId> = vec![];

    let point_count = 1000usize;
    for i in 0 .. point_count {
        points.push(Point3WithId::new(i as i32, gen_random(),gen_random(),gen_random()));
    }

    let tree = kdtree::kdtree::Kdtree::new(points.clone());

    //test points pushed into the tree, id should be equal.
    for i in 0 .. point_count {
        let p = &points[i];

        assert_eq!(p.id, tree.nearest_search(p).id );
    }

    //test randomly generated points within the cube. and do the linear search. should match
    for _ in 0 .. 500 {
        let p = Point3WithId::new(0i32, gen_random(), gen_random(), gen_random());

        let found_by_linear_search = find_nn_with_linear_search(&points, p);
        let point_found_by_kdtree = tree.nearest_search(&p);

        assert_eq!(point_found_by_kdtree.id, found_by_linear_search.id);
    }
}