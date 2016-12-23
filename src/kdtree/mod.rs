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

impl<T: KdtreePointTrait> Kdtree<T> {
    pub fn new(points: Vec<T>) -> Kdtree<T> {
        if points.len() == 0 {
            panic!("empty vector point not allowed");
        }

        let rect = Bounds::new_from_points(&points);

        Kdtree {
            nodes: vec![],
        }
    }

    fn add_node(&mut self, p: T) {
        let node = KdtreeNode::new(p);

        self.nodes.push(node);
    }

    fn add_left_node(&mut self, for_node: usize, ) {
        {
            let len = self.nodes.len();
            let node = self.nodes.get_mut(for_node).unwrap();
            node.left_node = Some(len);
        }
        //self.nodes.push(KdtreeNode::new());
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

    #[test]
    fn test2() {
        let p1 = Point2WithId::new(1, 1., 2.);
        let p2 = Point2WithId::new(1, 1., 2.);
        let vec = vec![p1, p2];

        Kdtree::new(vec);
    }
}