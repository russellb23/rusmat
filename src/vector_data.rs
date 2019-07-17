
use std::slice::Iter;
use std::slice::IterMut;

use std::vec::IntoIter;

use num::Float;

// Vector and Vector storage structure
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector<T: Float> {
    pub data: Vec<T>,
}


impl<T: Float> Vector<T> {
    /// Vector vector: constructor for Vector vector: Requires vector data
    pub fn new(data: Vec<T>) -> Vector<T> {
        let _data = data.into();

        Vector {
            data: _data,
        }
    }

    /// Vector vector from function
    pub fn from_fn<F>(mut f: F, length: usize) -> Vector<T> 
        where F: FnMut(usize) -> T {
            let _data = (0..length).into_iter().map(|i| f(i)).collect();

            Vector {
                data: _data,
            }
        }

    /// Get the length of the data
    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    /// Get the data from Vector Vector
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    /// Get data mutably from data vector
    pub fn get_mut_data(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Get data as an iter: Consumes the data vector
    pub fn into_vec(&self) -> Vec<T> {
        self.data.clone()
    }

    /// Safe mutable pointer to the element without bound checking
    pub unsafe fn uget(&self, idx: usize) -> &T {
        self.data.get_unchecked(idx)
    }

    /// Mutable reference pointer to the element
    pub unsafe fn uget_mut(&mut self, idx: usize) -> &mut T {
        self.data.get_unchecked_mut(idx)
    }

    /// Return immutable iterator over the data
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Return mutable iterator over the data
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.get_mut_data().iter_mut()
    }

    /// Check emptiness of a data vector
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Return pointer to the data
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Return a mutable pointer to the data
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Return slice of the vector data
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Return mutable slice of the data vector
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }



}
/// Return an iterator of the data
impl<T: Float> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Return an iterator of the data without consuming the data
impl<'a, T: Float> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Apply a function over the data
impl<T: Float> Vector<T> {
    pub fn apply<F>(mut self, f: &Fn(T) -> T) -> Vector<T> {
        for val in &mut self.data {
            *val = f(*val);
        }
        Vector {
            data: self.data,
        }
    }

    pub fn argsort(&self) -> usize {
        let v = self.clone();
        let m = self.clone().into_iter().fold(T::min_value(), |x,y| x.max(y));
        v.into_iter().position(|x| x == m).unwrap()
    }

}


