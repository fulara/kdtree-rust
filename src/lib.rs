//! # Kdtree-Rust
//!
//! K-dimensional tree for Rust (sliding midpoint rule implemenation)
//!
//! ## Usage
//!
//! ```
//! use kdtree::Kdtree;
//! use kdtree::distance::euclidean;
//! use kdtree::distance::squared_euclidean;
//! use kdtree::KdtreePointTrait;
//!
//!
//! // This can by any object or point with its associated metadata
//! #[derive(Copy, Clone, PartialEq)]
//! pub struct Point3WithId {
//!     dims: [f64; 3],
//!     pub id: i32,
//! }
//!
//!
//! impl KdtreePointTrait for Point3WithId {
//!     #[inline]
//!     fn dims(&self) -> &[f64] {
//!         return &self.dims;
//!     }
//! }
//!
//! let a = Point3WithId { dims: [0.0, 0.0, 0.0], id: 0};
//! let b = Point3WithId { dims: [1.0, 0.0, 0.0], id: 1};
//! let c = Point3WithId { dims: [1.0, 1.0, 0.0], id: 2};
//! let d = Point3WithId { dims: [1.0, 1.0, 1.0], id: 3};
//! let mut pts = vec![a,b,c,d];
//! let tree = Kdtree::new(&mut pts);
//!
//! assert_eq!(tree.nearest_search(&a).id, a.id);
//!
//! assert_eq!(tree.within(&a, 0.001, &euclidean).len(), 1);
//! assert_eq!(tree.within(&a, 1.001, &euclidean).len(), 2);
//! assert_eq!(tree.within(&a, 1.415, &euclidean).len(), 3);
//! assert_eq!(tree.within(&a, 2.0, &euclidean).len(), 4);
//! assert_eq!(tree.within(&a, 4.0, &squared_euclidean).len(), 4);
//! ```
//!
//!
//! ## Notes
//!
//! This kdtree implementation uses a trait interface, useful for things like ECS and game engines.
//! Agents need to implement KdtreePointTrait to be used in the kdtree.
//!
//! ## Performance
//!
//! The sliding midpoint method is extremely fast.
//! Using (nanoflann)[https://github.com/jlblancoc/nanoflann] as the baseline,
//! Kdtree-rust comes in as the fastest for both constructing trees,
//! and querying them. On a 2.9 GHz i7, we get 19ms to construct a 100k node tree
//! and 318.74ns to query it.
//!
//! It should be noted that kdtrees are meant for a construct-once and query-often access patterns.
//! This might library might be fast enough for your use case,
//! but real-time nearest-neighbors is still a hard problem to solve.
//!
//!

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate rand;

mod bounds;
pub mod distance;
mod kdtree;
mod partition;
pub mod test_common;

pub use kdtree::Kdtree;
pub use kdtree::KdtreePointTrait;
pub use test_common::{Point1WithId, Point2WithId, Point3WithId};
