#[cfg(test)]
pub mod test_common;

pub mod distance;

mod partition;
mod bounds;

use self::bounds::*;
use self::distance::*;

pub trait KdtreePointTrait: Copy {
    fn dims(&self) -> &[f64];
}

pub struct Kdtree<KdtreePoint> {
    nodes: Vec<KdtreeNode<KdtreePoint>>,

    node_adding_dimension: usize
}

impl<KdtreePoint: KdtreePointTrait> Kdtree<KdtreePoint> {
    pub fn new(mut points: &mut [KdtreePoint]) -> Kdtree<KdtreePoint> {
        if points.len() == 0 {
            panic!("empty vector point not allowed");
        }

        let rect = Bounds::new_from_points(points);

        let mut tree = Kdtree {
            nodes: vec![],
            node_adding_dimension: 0,
        };

        tree.build_tree(&mut points, &rect);

        tree
    }

    pub fn nearest_search(&self, node: &KdtreePoint) -> KdtreePoint
    {
        let mut nearest_neighbor = 0usize;
        let mut best_distance = squared_euclidean(node.dims(), &self.nodes[0].point.dims());
        self.nearest_search_impl(node, 0usize, &mut best_distance, &mut nearest_neighbor);

        self.nodes[nearest_neighbor].point
    }

    pub fn has_neighbor_in_range(&self, node: &KdtreePoint, range: f64) -> bool {
        let squared_range = range * range;

        squared_euclidean(&self.nearest_search(node).dims(), node.dims()) <= squared_range
    }

    pub fn insert_node(&mut self, node_to_add : KdtreePoint) {

        let mut current_index = 0;
        let dimension = self.node_adding_dimension;
        let index_of_new_node = self.add_node(node_to_add,dimension,node_to_add.dims()[dimension]);
        self.node_adding_dimension = ( dimension + 1) % node_to_add.dims().len();

        loop {

            let current_node = &mut self.nodes[current_index];

            if node_to_add.dims()[current_node.dimension] <= current_node.split_on {
                if let Some(left_node_index) = current_node.left_node {
                    current_index = left_node_index
                } else {
                    current_node.left_node = Some(index_of_new_node);
                    break;
                }
            } else {
                if let Some(right_node_index) = current_node.right_node {
                    current_index = right_node_index
                } else {
                    current_node.right_node = Some(index_of_new_node);
                    break;
                }
            }
        }
    }

    fn nearest_search_impl(&self, p: &KdtreePoint, searched_index: usize, best_distance_squared: &mut f64, best_leaf_found: &mut usize) {
        let node = &self.nodes[searched_index];

        let splitting_value = node.split_on;
        let point_splitting_dim_value = p.dims()[node.dimension];

        let (closer_node, farther_node) = self.select_closer_farther_node(node, splitting_value, point_splitting_dim_value);

        if let Some(closer_node) = closer_node {
            self.nearest_search_impl(p, closer_node, best_distance_squared, best_leaf_found);
        }

        let distance = squared_euclidean(p.dims(), node.point.dims());
        if distance < *best_distance_squared {
            *best_distance_squared = distance;
            *best_leaf_found = searched_index;
        }

        if let Some(farther_node) = farther_node {
            let distance_on_single_dimension = squared_euclidean(&[splitting_value], &[point_splitting_dim_value]);

            if distance_on_single_dimension <= *best_distance_squared {
                self.nearest_search_impl(p, farther_node, best_distance_squared, best_leaf_found);
            }
        }
    }

    fn select_closer_farther_node(&self, parent : &KdtreeNode<KdtreePoint>, parent_splits_dimension_on : f64, node_value_on_dimension : f64) -> (Option<usize>,Option<usize>) {
        if node_value_on_dimension <= parent_splits_dimension_on {
            (parent.left_node, parent.right_node)
        } else {
            (parent.right_node, parent.left_node)
        }
    }


    fn add_node(&mut self, p: KdtreePoint, dimension: usize, split_on: f64) -> usize {
        let node = KdtreeNode::new(p, dimension, split_on);

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn build_tree(&mut self, nodes: &mut [KdtreePoint], bounds: &Bounds) -> usize {
        let (splitting_index, pivot_value) = partition::partition_sliding_midpoint(nodes, bounds.get_midvalue_of_widest_dim(), bounds.get_widest_dim());

        let node_id = self.add_node(nodes[splitting_index], bounds.get_widest_dim(), pivot_value);
        let nodes_len = nodes.len();

        if splitting_index > 0 {
            let left_rect = bounds.clone_moving_max(pivot_value, bounds.get_widest_dim());
            let left_child_id = self.build_tree(&mut nodes[0..splitting_index], &left_rect);
            self.nodes[node_id].left_node = Some(left_child_id);
        }

        if splitting_index < nodes.len() - 1 {
            let right_rect = bounds.clone_moving_min(pivot_value, bounds.get_widest_dim());

            let right_child_id = self.build_tree(&mut nodes[splitting_index + 1..nodes_len], &right_rect);
            self.nodes[node_id].right_node = Some(right_child_id);
        }

        node_id
    }
}

pub struct KdtreeNode<T> {
    left_node: Option<usize>,
    right_node: Option<usize>,

    point: T,
    dimension: usize,
    split_on: f64
}

impl<T: KdtreePointTrait> KdtreeNode<T> {
    fn new(p: T, splitting_dimension: usize, split_on_value: f64) -> KdtreeNode<T> {
        KdtreeNode {
            left_node: None,
            right_node: None,

            point: p,
            dimension: splitting_dimension,
            split_on: split_on_value
        }
    }
}

#[cfg(test)]
mod tests {
    use ::kdtree::test_common::tests_utils::Point2WithId;

    use super::*;

    #[test]
    #[should_panic(expected = "empty vector point not allowed")]
    fn should_panic_given_empty_vector() {
        let mut empty_vec: Vec<Point2WithId> = vec![];

        Kdtree::new(&mut empty_vec);
    }

    quickcheck! {
        fn tree_build_creates_tree_with_as_many_leafs_as_there_is_points(xs : Vec<f64>) -> bool {
            if xs.len() == 0 {
                return true;
            }
            let mut vec : Vec<Point2WithId> = vec![];
            for i in 0 .. xs.len() {
                let p = Point2WithId::new(i as i32, xs[i], xs[i]);

                vec.push(p);
            }

            let tree = Kdtree::new(&mut qc_value_vec_to_2d_points_vec(&xs));

            let mut to_iterate : Vec<usize> = vec![];
            to_iterate.push(0);

            while to_iterate.len() > 0 {
                let last_index = to_iterate.last().unwrap().clone();
                let ref x = tree.nodes.get(last_index).unwrap();
                to_iterate.pop();
                if x.left_node.is_some() {
                    to_iterate.push(x.left_node.unwrap());
                }
                if x.right_node.is_some() {
                    to_iterate.push(x.right_node.unwrap());
                }
            }
            xs.len() == tree.nodes.len()
        }
    }

    quickcheck! {
        fn nearest_neighbor_search_using_qc(xs : Vec<f64>) -> bool {
            if xs.len() == 0 {
                return true;
            }

            let point_vec = qc_value_vec_to_2d_points_vec(&xs);
            let tree = Kdtree::new(&mut point_vec.clone());

            for p in &point_vec {
                let found_nn = tree.nearest_search(p);

                assert_eq!(p.id,found_nn.id);
            }

            true
        }
    }

    #[test]
    fn has_neighbor_in_range() {
        let mut vec: Vec<Point2WithId> = vec![Point2WithId::new(0,2.,0.)];

        let tree = Kdtree::new(&mut vec);

        assert_eq!(false,tree.has_neighbor_in_range(&Point2WithId::new(0,0.,0.), 0.));
        assert_eq!(false,tree.has_neighbor_in_range(&Point2WithId::new(0,0.,0.), 1.));
        assert_eq!(true,tree.has_neighbor_in_range(&Point2WithId::new(0,0.,0.), 2.));
        assert_eq!(true,tree.has_neighbor_in_range(&Point2WithId::new(0,0.,0.), 300.));
    }

    #[test]
    fn incremental_add_adds_as_expected() {
        let mut vec = vec![Point2WithId::new(0,0.,0.)];

        let mut tree = Kdtree::new(&mut vec);

        tree.insert_node(Point2WithId::new(0,1.,0.));
        tree.insert_node(Point2WithId::new(0,-1.,0.));

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.nodes[0].dimension, 0);

        assert_eq!(tree.nodes[0].left_node.is_some(), true);
        assert_eq!(tree.nodes[1].point.dims()[0], 1.);
        assert_eq!(tree.nodes[2].point.dims()[0], -1.);

        assert_eq!(tree.nodes[0].right_node.is_some(), true);
    }

    fn qc_value_vec_to_2d_points_vec(xs: &Vec<f64>) -> Vec<Point2WithId> {
        let mut vec: Vec<Point2WithId> = vec![];
        for i in 0..xs.len() {
            let mut is_duplicated_value = false;
            for j in 0..i {
                if xs[i] == xs[j] {
                    is_duplicated_value = true;
                    break;
                }
            }
            if !is_duplicated_value {
                let p = Point2WithId::new(i as i32, xs[i], xs[i]);
                vec.push(p);
            }
        }

        vec
    }
}