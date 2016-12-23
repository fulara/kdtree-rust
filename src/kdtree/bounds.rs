
use ::kdtree::*;

pub struct Bounds {
    pub bounds: Vec<(f64, f64)>
}

impl Bounds {
    pub fn new_from_points<T: KdtreePointTrait>(points: &Vec<T>) -> Bounds {
        let mut bounds = Bounds {
            bounds: vec![],
        };

        bounds.bounds.resize(points[0].dims().len(), (0.,0.));

        for i in 0..points[0].dims().len() {
            bounds.bounds[i].0 = points[0].dims()[i];
            bounds.bounds[i].1 = points[0].dims()[i];
        }

        for v in points.iter() {
            for dim in 0..v.dims().len() {
                bounds.bounds[dim].0 = bounds.bounds[dim].0.min(v.dims()[dim]);
                bounds.bounds[dim].1 = bounds.bounds[dim].1.max(v.dims()[dim]);
            }
        }

        bounds
    }

    pub fn get_widest_dim(&self) -> usize {
        let mut widest_dimension = 0usize;
        let mut max_found_spread = self.bounds[0].1 - self.bounds[0].0;

        for i in 0..self.bounds.len() {
            let dimension_spread = self.bounds[i].1 - self.bounds[i].0;

            if dimension_spread > max_found_spread {
                max_found_spread = dimension_spread;
                widest_dimension = i;
            }
        }

        widest_dimension
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::kdtree::test_common::tests_utils::*;

    #[test]
    fn bounds_test() {
        let p1 = Point2WithId::new(1,1.0,0.5);
        let p2 = Point2WithId::new(1,3.0,4.0);
        let v = vec![p1,p2];

        let bounds = Bounds::new_from_points(&v);

        assert_eq!((1., 3.0), bounds.bounds[0]);
        assert_eq!((0.5, 4.0), bounds.bounds[1]);

        assert_eq!(1, bounds.get_widest_dim());

    }
}