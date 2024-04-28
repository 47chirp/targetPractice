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

impl<T: Clone> Grid<T> {
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

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = self.xy_to_index(x, y)?;
        self.storage.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let idx = self.xy_to_index(x, y)?;
        self.storage.get_mut(idx)
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn clear(&mut self, value: T) {
        self.storage.fill(value);
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize, default: T) {
        let mut new_storage = vec![default; new_width * new_height];
        let min_width = usize::min(self.width, new_width);
        let min_height = usize::min(self.height, new_height);

        for y in 0..min_height {
            for x in 0..min_width {
                let old_idx = y * self.width + x;
                let new_idx = y * new_width + x;
                new_storage[new_idx] = self.storage[old_idx].clone();
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.storage = new_storage.into_boxed_slice();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.storage.iter_mut()
    }
}