//! Spatial data structures.
//!
//! Generic implementations for most common data structures for
//! storing and querying spatial data.
//!
//!
//! Currently implemented data structures are:
//!
//! * `Quadtree`, usually used for partitioning two-dimensional space.
//! * `Octree`, used for partitioning three-dimensional space.
//!
//! # Indexing
//!
//! In order for an *object* to be inserted into a quad- or an octree,
//! it must implement an appropriate `Index`-trait.
//!
//! ```
//! use spatial::quadtree;
//! use spatial::octree;
//!
//! // Our monster can exist in two- or three-dimensional space.
//! struct Monster {
//!     x: f32,
//!     y: f32,
//!     z: f32
//! }
//!
//! impl quadtree::Index<f32> for Monster {
//!     fn quadtree_index(&self) -> [f32; 2] {
//!         [self.x, self.y]
//!     }
//! }
//!
//! impl octree::Index<f32> for Monster {
//!     fn octree_index(&self) -> [f32; 3] {
//!         [self.x, self.y, self.z]
//!     }
//! }
//! ```
//!
//! The `Index`-traits are everything needed to start populating the
//! trees.

//#[unstable]
#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
#![feature(convert)]

pub use quadtree::Quadtree;
pub use octree::Octree;
pub mod quadtree;
pub mod octree;

extern crate core;
extern crate num;
use num::traits::Float;
use num::NumCast;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use std::fmt::Display;

pub trait SpatialKey : Float 
		+ Display 
		+ PartialOrd 
		+ Add<Self, Output=Self> 
		+ Sub<Self, Output=Self> 
		+ Mul<Self, Output=Self> 
		+ Div<Self, Output=Self> 
		+ NumCast
		+ Copy {}

impl SpatialKey for f32 {}

impl SpatialKey for f64 {}