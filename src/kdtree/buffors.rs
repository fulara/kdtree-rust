use ::kdtree::KdtreePointTrait;

pub trait KdtreeBuffor<KdtreePoint: KdtreePointTrait> {
    fn nearest_search(&self, node : &KdtreePoint) -> Option<KdtreePoint>;
    fn has_neighbor_in_range(&self, node : &KdtreePoint) -> bool;
}

pub struct FakeKdtreeBuffor {
}

impl FakeKdtreeBuffor {
    pub fn new() -> FakeKdtreeBuffor {
        FakeKdtreeBuffor {}
    }
}

impl<KdtreePoint: KdtreePointTrait> KdtreeBuffor<KdtreePoint> for FakeKdtreeBuffor {
    fn nearest_search(&self, node : &KdtreePoint) -> Option<KdtreePoint> {
        None
    }
    fn has_neighbor_in_range(&self, node : &KdtreePoint) -> bool {
        false
    }
}
