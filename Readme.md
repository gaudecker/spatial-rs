# spatial [![Build Status](https://travis-ci.org/gaudecker/spatial.svg?branch=master)](https://travis-ci.org/gaudecker/spatial)

A library for generic spatial data structures.

## Documentation

Documentation can be found on [rust-ci](http://www.rust-ci.org/gaudecker/spatial/doc/spatial/).

## Quadtree

In order for an object to be inserted into a quadtree, the
`quadtree::Index`-trait must be implemented.

``` rust
extern crate spatial;
use spatial::quadtree::{Quadtree, Index, Volume};

struct Object {
    x: f32,
    y: f32
}

impl Index<f32> for Object {
    fn quadtree_index(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}
```

To construct a quadtree, a bounding volume is needed.

``` rust
// arguments are in format `[x, y], [width, height]`
let volume = Volume::new([0, 0], [640, 480]);
let mut tree = Quadtree::new(volume);
```

Now the quadtree is ready for insertion and querying.

```rust
if tree.insert(Object { x: 68.0, y: 194.0 }) {
    println!("object inserted successfully!");
}

let objects = tree.get_in_volume(Volume::new([0.0, 0.0], [200.0, 200.0]));
```
