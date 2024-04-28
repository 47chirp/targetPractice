pub type Coord = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Duck,
    Obstacle,
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    storage: Box<[T]>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, cells: impl IntoIterator<Item = T>) -> Self {
        let cells: Vec<T> = cells.into_iter().collect();
        assert_eq!(
            cells.len(),
            width * height,
            "Not the right number of cells for the given width and height"
        );
        Self {
            width,
            height,
            storage: cells.into_boxed_slice(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.xy_to_index(x, y)?;
        self.storage.get_mut(index)
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = self.xy_to_index(x, y)?;
        self.storage.get(idx)
    }
    
}

// Implementations of Index and IndexMut for convenient access to grid cells
impl<T> std::ops::Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        let index = self.xy_to_index(x, y).expect("Index out of bounds");
        &self.storage[index]
    }
}

impl<T> std::ops::IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Self::Output {
        let index = self.xy_to_index(x, y).expect("Index out of bounds");
        &mut self.storage[index]
    }
}
