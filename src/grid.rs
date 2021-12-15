use std::{
    ops::{Index, IndexMut},
    slice,
};

use itertools::Itertools;

#[macro_export]
macro_rules! grid {
    [$e:expr; $width:expr, $height:expr] => {
        ::std::iter::repeat_with(|| ::std::iter::repeat($e).take($width))
            .take($height)
            .collect::<crate::grid::Grid<_>>()
    };
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width * height);
        grid.resize_with(width * height, T::default);

        Self {
            width,
            height,
            grid,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        // In flipped order for more efficient traversal I guess
        Itertools::cartesian_product(
            y.saturating_sub(1)..=y.saturating_add(1).min(self.height() - 1),
            x.saturating_sub(1)..=x.saturating_add(1).min(self.width() - 1),
        )
        .map(|(y, x)| (x, y))
        .filter(move |&(xi, yi)| xi != x || yi != y)
        .sorted()
        .dedup()
    }

    pub fn neighbours_orthogonal(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        [
            x.checked_sub(1).map(|xi| (xi, y)),
            x.checked_add(1)
                .filter(|&xi| xi < self.width())
                .map(|xi| (xi, y)),
            y.checked_sub(1).map(|yi| (x, yi)),
            y.checked_add(1)
                .filter(|&yi| yi < self.height())
                .map(|yi| (x, yi)),
        ]
        .into_iter()
        .filter_map(|v| v)
    }

    pub fn into_flat_iter(self) -> impl Iterator<Item = T> {
        self.grid.into_iter()
    }

    pub fn flat_iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.grid.iter_mut()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y * self.width + x]
    }
}

impl<T, I> FromIterator<I> for Grid<T>
where
    I: IntoIterator<Item = T>,
{
    fn from_iter<S: IntoIterator<Item = I>>(iter: S) -> Self {
        let mut height = 0;

        let grid = iter
            .into_iter()
            .flat_map(|row_iter| {
                height += 1; // hacky but it works!
                row_iter
            })
            .collect_vec();

        Self {
            width: grid.len() / height, // TODO: check that this is actually true?
            height,
            grid,
        }
    }
}
