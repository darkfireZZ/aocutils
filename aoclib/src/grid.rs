use std::ops::Index;

/// A type to deal with the 2D ASCII grids that are very common in Advent of Code.
///
/// The grid is internally stored in row-major order.
pub struct Grid<V> {
    values: Vec<V>,
    width: usize,
    height: usize,
}

impl Grid<u8> {
    /// Parses an ASCII grid.
    ///
    /// Lines are separated by newline characters (`'\n'`) and all lines are expected to be of the
    /// same length.
    ///
    /// # Panics
    ///
    /// Panics if there are lines with different length.
    pub fn parse(grid: &[u8]) -> Self {
        let mut lines = grid.split(|c| *c == b'\n');

        let first_line = match lines.next() {
            Some(first_line) => first_line,
            None => return Self::empty(),
        };
        let width = first_line.len();

        let mut values = first_line.to_owned();
        for line in &mut lines {
            if line.len() != width {
                // The last line is allowed to be terminated by a newline
                if line.is_empty() && lines.next().is_none() {
                    break;
                } else {
                    panic!("Invalid grid, found lines with different lengths");
                }
            }
            values.extend_from_slice(line);
        }

        let height = values.len() / width;

        Self {
            values,
            width,
            height,
        }
    }

    /// Interprets the values of the grid as ASCII and formats the grid to a [`String`].
    ///
    /// Rows are separated by newline characters (`'\n'`).
    ///
    /// ```
    /// # use aoclib::Grid;
    /// let ascii_grid = b"\
    ///     123\n\
    ///     456\n\
    ///     789\n";
    /// let grid = Grid::parse(ascii_grid);
    ///
    /// let formatted_grid = grid.format_ascii();
    ///
    /// assert_eq!(formatted_grid.as_bytes(), ascii_grid);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the grid contains values that are not in the ASCII range.
    ///
    /// ```should_panic
    /// # use aoclib::Grid;
    /// let mut grid = Grid::parse(b"12\n34\n");
    /// *grid.get_mut(0, 1) = 255;
    /// grid.format_ascii(); // This will panic!
    /// ```
    pub fn format_ascii(&self) -> String {
        if !self.values.is_ascii() {
            panic!("Grid contains non-ASCII bytes");
        }

        let mut formatted = String::with_capacity((self.width + 1) * self.height);

        let mut remainder = self.values.as_slice();
        while !remainder.is_empty() {
            formatted.push_str(
                std::str::from_utf8(&remainder[..self.width])
                    .expect("checked before that all chars are ASCII"),
            );
            formatted.push('\n');
            remainder = &remainder[self.width..];
        }

        formatted
    }
}

impl<V> Grid<V> {
    /// Returns the width of this grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Creates an empty grid with 0 width and 0 height.
    fn empty() -> Self {
        Self {
            values: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn coords_to_slice_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);

        y * self.width + x
    }

    fn slice_index_to_coords(&self, index: usize) -> (usize, usize) {
        assert!(index < self.width * self.height);

        let x = index % self.width;
        let y = index / self.width;

        (x, y)
    }

    /// Gets a reference to the grid element at column `x` and row `y`.
    ///
    /// # Panics
    ///
    /// Panics if any of the indices is out of range.
    pub fn get(&self, x: usize, y: usize) -> &V {
        &self.values[self.coords_to_slice_index(x, y)]
    }

    /// Gets a mutable reference to the grid element at column `x` and row `y`.
    ///
    /// # Panics
    ///
    /// Panics if any of the indices is out of range.
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut V {
        let index = self.coords_to_slice_index(x, y);
        &mut self.values[index]
    }

    /// Returns a view to a specific row of the grid.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of range.
    pub fn row(&self, row: usize) -> GridRow<V> {
        assert!(row < self.height);

        GridRow { grid: self, row }
    }

    /// Returns a view to a specific column of the grid.
    ///
    /// # Panics
    ///
    /// Panics if `col` is out of range.
    pub fn col(&self, col: usize) -> GridCol<V> {
        assert!(col < self.width);

        GridCol { grid: self, col }
    }

    /// Creates a new [`Grid`] by applying a function to every element of the grid.
    pub fn map<M, Vnew>(&self, map: M) -> Grid<Vnew>
    where
        M: Fn(&V) -> Vnew,
    {
        Grid::<Vnew> {
            values: self.values.iter().map(map).collect(),
            width: self.width,
            height: self.height,
        }
    }

    /// Creates a new [`Grid`] by applying a function to every element and its position.
    pub fn map_indexed<M, Vnew>(&self, map: M) -> Grid<Vnew>
    where
        M: Fn(&V, usize, usize) -> Vnew,
    {
        Grid::<Vnew> {
            values: self
                .values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let (x, y) = self.slice_index_to_coords(index);
                    map(value, x, y)
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    /// Returns an [`Iterator`] over all rows of this grid.
    pub fn rows(&self) -> Rows<V> {
        Rows { row: 0, grid: self }
    }

    /// Returns an [`Iterator`] over all cols of this grid.
    pub fn cols(&self) -> Cols<V> {
        Cols { col: 0, grid: self }
    }
}

#[derive(Clone)]
pub struct GridRow<'a, V> {
    row: usize,
    grid: &'a Grid<V>,
}

impl<'a, V> Index<usize> for GridRow<'a, V> {
    type Output = V;

    fn index(&self, col: usize) -> &Self::Output {
        self.grid.get(col, self.row)
    }
}

impl<'a, V> IntoIterator for GridRow<'a, V> {
    type Item = &'a V;
    type IntoIter = GridRowIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        GridRowIter {
            grid: self.grid,
            row: self.row,
            col: 0,
        }
    }
}

#[derive(Clone)]
pub struct GridCol<'a, V> {
    col: usize,
    grid: &'a Grid<V>,
}

impl<'a, V> Index<usize> for GridCol<'a, V> {
    type Output = V;

    fn index(&self, row: usize) -> &Self::Output {
        self.grid.get(self.col, row)
    }
}

impl<'a, V> IntoIterator for GridCol<'a, V> {
    type Item = &'a V;
    type IntoIter = GridColIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        GridColIter {
            grid: self.grid,
            row: 0,
            col: self.col,
        }
    }
}

#[derive(Clone)]
pub struct GridRowIter<'a, V> {
    grid: &'a Grid<V>,
    row: usize,
    col: usize,
}

impl<'a, V> Iterator for GridRowIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.grid.width {
            let val = self.grid.get(self.col, self.row);
            self.col += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct GridColIter<'a, V> {
    grid: &'a Grid<V>,
    row: usize,
    col: usize,
}

impl<'a, V> Iterator for GridColIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.height {
            let val = self.grid.get(self.col, self.row);
            self.row += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Rows<'a, V> {
    row: usize,
    grid: &'a Grid<V>,
}

impl<'a, V> Iterator for Rows<'a, V> {
    type Item = GridRow<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.height {
            let row = self.grid.row(self.row);
            self.row += 1;
            Some(row)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Cols<'a, V> {
    col: usize,
    grid: &'a Grid<V>,
}

impl<'a, V> Iterator for Cols<'a, V> {
    type Item = GridCol<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.grid.width {
            let col = self.grid.col(self.col);
            self.col += 1;
            Some(col)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    const SMILEY_GRID: &[u8] = b"\
            s...........\n\
            ..XX....XX..\n\
            ............\n\
            ..oo....oo..\n\
            ...oooooo...\n\
            ...........e";

    #[test]
    fn dimensions() {
        let grid = Grid::parse(SMILEY_GRID);

        assert_eq!(grid.width(), 12);
        assert_eq!(grid.height(), 6);
    }

    #[test]
    fn indexing() {
        let grid = Grid::parse(SMILEY_GRID);

        assert_eq!(*grid.get(0, 0), b's');
        assert_eq!(*grid.get(11, 5), b'e');
        assert_eq!(*grid.get(3, 1), b'X');
        assert_eq!(*grid.get(7, 4), b'o');
    }

    #[test]
    fn row() {
        let grid = Grid::parse(SMILEY_GRID);

        let row_1: Vec<_> = grid.row(1).into_iter().copied().collect();
        let row_2: Vec<_> = grid.row(4).into_iter().copied().collect();

        assert_eq!(row_1, b"..XX....XX..");
        assert_eq!(row_2, b"...oooooo...");
    }

    #[test]
    fn col() {
        let grid = Grid::parse(SMILEY_GRID);

        let col_2: Vec<_> = grid.col(2).into_iter().copied().collect();
        let col_11: Vec<_> = grid.col(11).into_iter().copied().collect();

        assert_eq!(col_2, b".X.o..");
        assert_eq!(col_11, b".....e");
    }

    #[test]
    fn rows_cols_iter() {
        let grid = Grid::parse(b"ab\ncd");

        let rows: Vec<_> = grid
            .rows()
            .map(|row| row.into_iter().copied().collect::<Vec<_>>())
            .collect();
        let cols: Vec<_> = grid
            .cols()
            .map(|col| col.into_iter().copied().collect::<Vec<_>>())
            .collect();

        assert_eq!(rows, [[b'a', b'b'], [b'c', b'd']]);
        assert_eq!(cols, [[b'a', b'c'], [b'b', b'd']]);
    }

    #[test]
    fn map() {
        let grid = Grid::parse(SMILEY_GRID);

        let grid = grid.map(|c| match c {
            b'.' => "skin",
            b'o' => "mouth",
            b'X' => "eye",
            _ => "other",
        });

        assert_eq!(*grid.get(0, 0), "other");
        assert_eq!(*grid.get(8, 1), "eye");
        assert_eq!(*grid.get(8, 3), "mouth");
        assert_eq!(*grid.get(5, 2), "skin");
    }

    #[test]
    fn map_indexed() {
        let grid = Grid::parse(SMILEY_GRID);

        let grid = grid.map_indexed(|c, x, y| {
            let value = match c {
                b'.' => "skin",
                b'o' => "mouth",
                b'X' => "eye",
                _ => "other",
            };

            (x + y, value)
        });

        assert_eq!(*grid.get(0, 0), (0, "other"));
        assert_eq!(*grid.get(8, 1), (9, "eye"));
        assert_eq!(*grid.get(8, 3), (11, "mouth"));
        assert_eq!(*grid.get(5, 2), (7, "skin"));
    }

    #[test]
    fn row_index() {
        let grid = Grid::parse(SMILEY_GRID);

        for row in 0..grid.height() {
            let row_view = grid.row(row);
            for col in 0..grid.width() {
                assert_eq!(row_view[col], *grid.get(col, row));
            }
        }
    }

    #[test]
    fn col_index() {
        let grid = Grid::parse(SMILEY_GRID);

        for col in 0..grid.width() {
            let col_view = grid.col(col);
            for row in 0..grid.height() {
                assert_eq!(col_view[row], *grid.get(col, row));
            }
        }
    }

    #[test]
    #[should_panic]
    fn parse_invalid_grid() {
        Grid::parse(b"..\n.");
    }

    #[test]
    fn last_line_terminated_by_newline() {
        Grid::parse(b"...\n...\n");
    }
}
