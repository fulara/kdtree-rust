mod test_common;
mod partition;
mod bounds;

use ::std::cmp;
use self::bounds::*;

pub trait KdtreePointTrait {
    fn dims(&self) -> &[f64];
}

pub struct Kdtree<T> {
    nodes: Vec<KdtreeNode<T>>,
}

impl<T: KdtreePointTrait + Copy> Kdtree<T> {
    pub fn new(mut points: Vec<T>) -> Kdtree<T> {
        if points.len() == 0 {
            panic!("empty vector point not allowed");
        }

        let rect = Bounds::new_from_points(&points);

        let mut tree = Kdtree {
            nodes: vec![],
        };

        tree.build_tree(&mut points, &rect);

        tree
    }

    fn add_node(&mut self, p: T) -> usize {
        let node = KdtreeNode::new(p);

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn build_tree(&mut self, nodes: &mut [T], bounds: &Bounds) -> usize {
        let (splitting_index, pivot_value) = partition::partition_sliding_midpoint(nodes, bounds.get_midvalue_of_widest_dim(), bounds.get_widest_dim());

        let node_id = self.add_node(nodes[splitting_index]);
        let nodes_len = nodes.len();

        if splitting_index > 0 {
            let left_rect = bounds.clone_moving_max(bounds.get_midvalue_of_widest_dim(), bounds.get_widest_dim());
            let left_child_id = self.build_tree(&mut nodes[0..splitting_index], &left_rect);
            self.nodes[node_id].left_node = Some(left_child_id);
        }

        if splitting_index < nodes.len() - 1 {
            let right_rect = bounds.clone_moving_min(bounds.get_midvalue_of_widest_dim(), bounds.get_widest_dim());

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
}

impl<T: KdtreePointTrait> KdtreeNode<T> {
    fn new(p: T) -> KdtreeNode<T> {
        KdtreeNode {
            left_node: None,
            right_node: None,

            point: p,
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
        let empty_vec: Vec<Point2WithId> = vec![];

        Kdtree::new(empty_vec);
    }

    quickcheck! {
        fn tree_build_creates_tree_with_as_many_leafs_as_there_is_points(xs : Vec<f64>) -> bool {
            if(xs.len() == 0) {
                return true;
            }
            let mut vec : Vec<Point2WithId> = vec![];
            for i in 0 .. xs.len() {
                let p = Point2WithId::new(i as i32, xs[i], xs[i]);

                vec.push(p);
            }

            let tree = Kdtree::new(vec);

            let mut to_iterate : Vec<usize> = vec![];
            to_iterate.push(0);

            let mut str  = String::new();

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

                str.push_str(&format!("Index: {} has ln {} has rn {} \n", last_index, x.left_node.is_some(), x.right_node.is_some()));


            }

           // println!("str is: {}", str);

            xs.len() == tree.nodes.len()
        }
    }
}