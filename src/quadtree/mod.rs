pub use self::volume::Volume;
use SpatialKey;
use num::NumCast;

mod volume;

/// The default capacity of a quadtree's node until it's subdivided.
static DEFAULT_CAPACITY: usize = 8;

/// A trait that must be implemented by types that are going to be
/// inserted into a `Quadtree`.
pub trait Index<T: SpatialKey> {
    /// This method returns the position for `self` in 2D-space. The
    /// return format should be in order of `[x, y]`.
    fn quadtree_index(&self) -> [T; 2];
}

pub struct Quadtree<T: SpatialKey, P: Index<T> + Clone> {
    /// Maximum number of items to store before subdivision.
    capacity: usize,
    /// Items in this quadtree node.
    items: Vec<P>,
    /// Bounding volume of this node.
    volume: Volume<T>,
    /// The four quadrants of this node, in order of NW, NE, SW, SE.
    quadrants: Option<[Box<Quadtree<T, P>>; 4]>
}

impl<T: SpatialKey, P: Index<T> + Clone> Quadtree<T, P> {
    /// Constructs a new, empty `Quadtree` with bounding volume `vol`
    /// and default node capacity of `DEFAULT_CAPACITY`.
    #[inline]
    pub fn new(vol: Volume<T>) -> Quadtree<T, P> {
        Quadtree {
            capacity: DEFAULT_CAPACITY,
            items: Vec::with_capacity(DEFAULT_CAPACITY),
            volume: vol,
            quadrants: None
        }
    }

    /// Creates an empty quadtree with volume `vol` and `capacity`.
    #[inline]
    pub fn with_capacity(vol: Volume<T>, capacity: usize) -> Quadtree<T, P> {
        Quadtree {
            capacity: capacity,
            items: Vec::with_capacity(capacity),
            volume: vol,
            quadrants: None
        }
    }

    /// Returns the number of items in the tree.
    #[inline]
    pub fn len(&self) -> usize {
        let mut len = self.items.len();
        match self.quadrants {
            Some(ref quadrants) => for ref node in quadrants.iter() {
                len += node.len();
            },
            None => {}
        }
        len
    }

    /// Inserts an `item` into the quadtree, subdividing it if
    /// necessary.
    #[inline]
    pub fn insert(&mut self, item: P) -> bool {
        // item must exist inside this quads' space.
        if !self.volume.contains(&item.quadtree_index()) {
            return false;
        }
        
        // Insert item it there's room.
        if self.items.len() < self.capacity {
            self.items.push(item.clone());
            return true;
        }
        
        match self.quadrants {
            Some(ref mut quadrants) => for node in quadrants.iter_mut() {
                if node.insert(item.clone()) {
                    return true;
                }
            },
            None => self.subdivide()
        }
        
        false
    }
    
    /// Returns all items inside the volume `vol`.
    #[inline]
    pub fn get_in_volume<'a>(&'a self, vol: &Volume<T>) -> Vec<&'a P> {
        let mut items = Vec::new();

        // Return empty vector if vol does not intersect.
        if !self.volume.intersects(vol) {
            return items;
        }

        // Add items for this node.
        for item in self.items.iter() {
            if vol.contains(&item.quadtree_index()) {
                items.push(item);
            }
        }
        
        match self.quadrants {
            Some(ref quadrants) => {
                for ref node in quadrants.iter() {
                    items.push_all(node.get_in_volume(vol).as_slice());
                }
                items
            },
            None => items
        }
    }
    
    #[inline]
    pub fn get_in_radius<'a>(&'a self, center: [T; 2] , radius: T) -> Vec<&'a P> {
        let min = [center[0] - radius, center[1] - radius];
        let max = [center[0] + radius, center[1] + radius];
        
        println!("Got bounding box between {}.{} and {}.{}", 
        		min[0], min[1], max[0], max[1] );
        
        let volume = Volume::new(min, max);
        let mut in_box = self.get_in_volume( &volume );
        println!("Got {} in box", in_box.len() );
        let mut in_sphere = Vec::new();
        
        let val2 : T = NumCast::from(2).unwrap();
        
        for item in in_box.drain() {
        	let index = item.quadtree_index();
        	let d0 = (index[0] - center[0]).powf(val2);
        	let d1 = (index[1] - center[1]).powf(val2);
        	
        	let distance : T = (d0 + d1).sqrt();
        	
        	
        	println!("Got distance {} between {}.{} and {}.{}", distance,
        		index[0], index[1], center[0], center[1] );
        	if distance <= radius  {
        		println!( "Including {}.{}", index[0], index[1] );
        		in_sphere.push( item );
        	}
        }
        
        return in_sphere;
    }
    
    /// Creates four equal sized subtrees for this node.
    #[inline]
    fn subdivide(&mut self) {
        let min = self.volume.min;
        let max = self.volume.max;
        
        let val2 = NumCast::from(2).unwrap();
        
        let (hw, hh) = (max[0].div(val2), max[1].div(val2));
        
        self.quadrants = Some([
            box Quadtree::with_capacity(Volume::new([min[0], min[1]], [hw, hh]), self.capacity),
            box Quadtree::with_capacity(Volume::new([min[0] + hh, min[1]], [max[0], hh]), self.capacity),
            box Quadtree::with_capacity(Volume::new([min[0], min[1] + hh], [hw, max[1]]), self.capacity),
            box Quadtree::with_capacity(Volume::new([min[0] + hw, min[1] + hh], [max[0], max[1]]), self.capacity)
                ]);
    }
}
