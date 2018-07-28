use std::collections::HashSet;
use std::iter::FromIterator;

extern crate rand;

/// Constructs an [index](struct.Index.html).
///
/// ### Usage
/// This macro takes two arguments: a hoorizontal and vertical position (in that order).
///
/// ### Examples
/// ```
/// # #[macro_use] extern crate viv;
/// # use viv::Index;
/// let idx = index!(0, 0);
/// assert_eq!(idx, Index::origin());
/// ```
#[macro_export]
macro_rules! index {
    ($x:expr, $y:expr) => {
        Index { x: $x, y: $y }
    };
}

/// Used to index into a grid.
///
/// ### Construction
/// The [`index!`](macro.index.html) macro is recommended for construction of indices.
///
/// ### Notes
/// This type is intended to be more-or-less opaque.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Index {
    #[doc(hidden)]
    pub x: isize,
    #[doc(hidden)]
    pub y: isize,
}

impl Index {
    /// The index at the center of any given grid.
    pub fn origin() -> Self {
        index!(0, 0)
    }

    /// The neighboring indices for this index.
    pub fn neighbors(self) -> [Index; 8] {
        [
            self.up().left(),
            self.up(),
            self.up().right(),
            self.right(),
            self.down().right(),
            self.down(),
            self.down().left(),
            self.left(),
        ]
    }

    /// The neighboring indices for this index (in vector form).
    ///
    /// This helps when using iterators, since array iterators use slices extensively, and I
    /// haven't yet found a satisfactory workaround for the ensuing lifetime issues.
    // I really hope the optimizer can do something clever with this.
    pub fn neighbors_vec(self) -> Vec<Index> {
        self.neighbors()[0..].into()
    }

    /// The index above this one.
    pub fn up(self) -> Self {
        index!(self.x, self.y + 1)
    }

    /// The index below this one.
    pub fn down(self) -> Self {
        index!(self.x, self.y - 1)
    }

    /// The index right of this one.
    pub fn right(self) -> Self {
        index!(self.x + 1, self.y)
    }

    /// The index left of this one.
    pub fn left(self) -> Self {
        index!(self.x - 1, self.y)
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// An infinite, two-dimensional, orthogonal grid of (square) cells.
#[derive(Clone)]
pub struct Grid {
    /// The indices of the living cells in the grid.
    ///
    /// If an index is absent from this list, the corresponding cell is assumed to be dead.
    living: HashSet<Index>,
}

impl Grid {
    /// Creates a new grid with a random seed.
    ///
    /// ### Notes
    /// The seed, of course, is not truly random, so probably don't use it as an entropy source.
    pub fn generate() -> Self {
        let mut grid = Self {
            living: HashSet::new(),
        };
        for i in -50..50 {
            for j in -20..20 {
                if rand::random() {
                    grid.living.insert(index!(i, j));
                }
            }
        }
        grid
    }

    /// Creates a new grid with the given seed.
    ///
    /// The seed is a collection of indices indictating the positions of living cells at the
    /// beginning of time.
    pub fn new<I>(seed: I) -> Self
    where
        I: IntoIterator<Item = Index>,
    {
        Self {
            living: HashSet::from_iter(seed),
        }
    }

    /// Runs the next tick of the simulation, returning a copy of the grid with updated state.
    ///
    /// Looking at this pseudo-mathematically, this applies the next iteration of the *Life* function.
    ///
    /// ### Rules
    /// On each tick:
    /// 1) Living cells with fewer than two living neighbors die.
    /// 2) Living cells with two or three living neighbors live on.
    /// 3) Living cells with more than three living neighbors die.
    /// 4) Dead cells with exactly three living neighbors become living cells.
    pub fn tick(&self) -> Self {
        let mut new = self.clone();
        for index in self.living.iter() {
            let neighbors = self.living_neighbors(*index);
            if neighbors < 2 || neighbors > 3 {
                new.kill(*index);
            }
        }
        // Dead cells can only have three living neighbors if they have at least one living
        // neighbor.
        for index in self.living
            .iter()
            .flat_map(|index| index.neighbors_vec().into_iter())
            .filter(|n| !self.is_living(n))
        {
            if self.living_neighbors(index) == 3 {
                new.unkill(index);
            }
        }
        return new;
    }

    /// The number of live neighbors of the cell at the given index within the grid.
    pub fn living_neighbors(&self, index: Index) -> usize {
        index
            .neighbors()
            .iter()
            .filter(|index| self.is_living(*index))
            .count()
    }

    /// Whether the specified cell is living.
    pub fn is_living(&self, index: &Index) -> bool {
        self.living.contains(&index)
    }

    /// Kills the cell at the given index.
    pub fn kill(&mut self, index: Index) {
        self.living.remove(&index);
    }

    /// "Unkills" the cell at the given index.
    pub fn unkill(&mut self, index: Index) {
        self.living.insert(index);
    }

    /// The index at the center of this grid.
    ///
    /// ### Notes
    /// This method is an alias for [`Index::origin`](struct.Index.html#method.origin).
    pub fn origin() -> Index {
        Index::origin()
    }
}
