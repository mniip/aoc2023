use std::{
    iter::FusedIterator,
    ops::{Index, IndexMut},
    slice, vec,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    width: usize,
    row_major: Vec<Vec<T>>, // invariant: all rows have len() equal to width
}

impl<T> Rect<T> {
    pub fn new_wide(width: usize) -> Self {
        Rect {
            width,
            row_major: Vec::new(),
        }
    }

    pub fn new_tall(height: usize) -> Self {
        Rect {
            width: 0,
            row_major: (0..height).map(|_| Vec::new()).collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.row_major.len()
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        assert_eq!(row.len(), self.width);
        self.row_major.push(row);
    }

    pub fn pop_row(&mut self) -> Option<Vec<T>> {
        self.row_major.pop()
    }

    pub fn push_col(&mut self, col: Vec<T>) {
        assert_eq!(col.len(), self.row_major.len());
        for (row, elem) in self.row_major.iter_mut().zip(col) {
            row.push(elem)
        }
        self.width += 1;
    }

    pub fn pop_col(&mut self) -> Option<Vec<T>> {
        if self.width == 0 {
            return None;
        }
        self.width -= 1;
        let mut col = Vec::with_capacity(self.row_major.len());
        for row in self.row_major.iter_mut() {
            col.push(row.pop().unwrap())
        }
        Some(col)
    }

    pub fn iter(&self) -> slice::Iter<'_, Vec<T>> {
        self.row_major.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Vec<T>> {
        self.row_major.iter_mut()
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.row_major
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, value)| (x, y, value)))
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.row_major.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, value)| (x, y, value))
        })
    }

    pub fn get<I: TryInto<usize>>(&self, (x, y): (I, I)) -> Option<&T> {
        self.row_major
            .get(y.try_into().ok()?)?
            .get(x.try_into().ok()?)
    }

    pub fn get_mut<I: TryInto<usize>>(&mut self, (x, y): (I, I)) -> Option<&mut T> {
        self.row_major
            .get_mut(y.try_into().ok()?)?
            .get_mut(x.try_into().ok()?)
    }
}

impl<T, I: TryInto<usize>> Index<(I, I)> for Rect<T> {
    type Output = T;

    fn index(&self, pos: (I, I)) -> &Self::Output {
        self.get(pos).unwrap()
    }
}

impl<T, I: TryInto<usize>> IndexMut<(I, I)> for Rect<T> {
    fn index_mut(&mut self, pos: (I, I)) -> &mut Self::Output {
        self.get_mut(pos).unwrap()
    }
}

impl<T> IntoIterator for Rect<T> {
    type Item = Vec<T>;
    type IntoIter = vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row_major.into_iter()
    }
}

impl<T> FromIterator<Vec<T>> for Rect<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Rect {
                width: 0,
                row_major: Vec::new(),
            },
            Some(first_row) => {
                let mut rect = Rect {
                    width: first_row.len(),
                    row_major: vec![first_row],
                };
                rect.row_major.reserve(iter.size_hint().0);
                for row in iter {
                    rect.push_row(row)
                }
                rect
            }
        }
    }
}

pub struct Transposed<T>(pub T);

pub struct TransposedIter<T> {
    remaining: usize,
    iterators: Vec<vec::IntoIter<T>>,
}

impl<T> Iterator for TransposedIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.next().unwrap())
                    .collect(),
            )
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.remaining <= n {
            None
        } else {
            self.remaining -= n;
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.nth(n).unwrap())
                    .collect(),
            )
        }
    }
}

impl<T> FusedIterator for TransposedIter<T> {}

impl<T> ExactSizeIterator for TransposedIter<T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<T> DoubleEndedIterator for TransposedIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.next_back().unwrap())
                    .collect(),
            )
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.remaining <= n {
            None
        } else {
            self.remaining -= n;
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.nth_back(n).unwrap())
                    .collect(),
            )
        }
    }
}

impl<T> IntoIterator for Transposed<Rect<T>> {
    type Item = Vec<T>;
    type IntoIter = TransposedIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        TransposedIter {
            remaining: self.0.width,
            iterators: self
                .0
                .row_major
                .into_iter()
                .map(|row| row.into_iter())
                .collect(),
        }
    }
}

impl<'a, T> Transposed<&'a Rect<T>> {
    pub fn iter(self) -> impl Iterator<Item = impl Iterator<Item = &'a T>> {
        (0..self.0.width).map(|x| self.0.row_major.iter().map(move |row| &row[x]))
    }
}

pub struct TransposedIterMut<'a, T> {
    remaining: usize,
    iterators: Vec<slice::IterMut<'a, T>>,
}

impl<'a, T> Iterator for TransposedIterMut<'a, T> {
    type Item = Vec<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.next().unwrap())
                    .collect(),
            )
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.remaining <= n {
            None
        } else {
            self.remaining -= n;
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.nth(n).unwrap())
                    .collect(),
            )
        }
    }
}

impl<'a, T> FusedIterator for TransposedIterMut<'a, T> {}

impl<'a, T> ExactSizeIterator for TransposedIterMut<'a, T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a, T> DoubleEndedIterator for TransposedIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.next_back().unwrap())
                    .collect(),
            )
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.remaining <= n {
            None
        } else {
            self.remaining -= n;
            self.remaining -= 1;
            Some(
                self.iterators
                    .iter_mut()
                    .map(|row| row.nth_back(n).unwrap())
                    .collect(),
            )
        }
    }
}

impl<'a, T> Transposed<&'a mut Rect<T>> {
    pub fn iter_mut(self) -> TransposedIterMut<'a, T> {
        TransposedIterMut {
            remaining: self.0.width,
            iterators: self
                .0
                .row_major
                .iter_mut()
                .map(|row| row.iter_mut())
                .collect(),
        }
    }
}

impl<T> FromIterator<Vec<T>> for Transposed<Rect<T>> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Transposed(Rect {
                width: 0,
                row_major: Vec::new(),
            }),
            Some(first_col) => {
                let (width_hint, _) = iter.size_hint();
                let mut rect = Rect {
                    width: 1,
                    row_major: first_col
                        .into_iter()
                        .map(|cell| {
                            let mut vec = vec![cell];
                            vec.reserve(width_hint);
                            vec
                        })
                        .collect(),
                };
                for col in iter {
                    rect.push_col(col)
                }
                Transposed(rect)
            }
        }
    }
}
