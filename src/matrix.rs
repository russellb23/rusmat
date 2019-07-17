// Basic data structs
use std::fmt;
use std::fmt::{Debug, Display};

use std::ops::Range;

use std::marker::PhantomData;

use num::Float;
use num::traits::cast::FromPrimitive;

use super::vector_data::Vector;

//=============================================================================
//Matrix major axis
//=============================================================================
/// Matrix storage type: Column(default) and Row
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Axis {
    Column,
    Row,
}

impl Axis {
    fn transpose(&self) -> Axis {
        match self {
            Column => Axis::Row,
            Row => Axis::Column,
        }
    }

    fn t(&self) -> Axis {
        self.transpose()
    }
}

//=============================================================================
//Data
//=============================================================================
//Data structure using RefCell
//#[derive(Debug, Clone, Eq, PartialEq)]
//pub struct Data<T> {
//    data: Vec<T>,
//}
//
//impl<T> Data<T> 
//    where T: Float {
//        /// Borrow the underlying data
//        pub fn vals(self) -> Vec<T> {
//            self.data.clone()
//        }
//
//        /// Borrow mutable reference
//        pub fn vals_mut(&mut self) -> &mut [T] {
//            self.data.as_mut_slice()
//        }
//
//        /// Pointer to the data
//        pub fn vals_as_ptr(&self) -> *const T {
//            self.data.as_ptr()
//        }
//
//        /// Check is data has no values
//        pub fn is_empty(&self) -> bool {
//            self.data.len() == 0
//        }
//    }

//=============================================================================
//Matrix
//=============================================================================
/// Matrix struct
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<'a, T: Float> {
    data: Vector<T>,
    rows: usize, // number of rows
    cols: usize, // number of columns
    mode: Axis, // Major axis
    strd: usize, // Stride
    mark: PhantomData<&'a T>, // Phantom data for life time

}

impl<'a, T: Float> Matrix<'a, T> {

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_shape(&self) -> (usize,usize) {
        (self.rows, self.cols)
    }

    pub fn get_size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn mindim(&self) -> usize {
        self.rows.min(self.cols)
    }

    /// Matrix emptiness
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get matrix data
    pub fn get_data(&self) -> &Vec<T> {
        &self.data.get_data()
    }

    /// Get mutable matrix data
    pub fn get_mut_data(&mut self) -> &mut [T] {
        self.data.get_mut_data()
    }

    /// Get storage mode
    pub fn get_mode(&self) -> Axis {
        self.mode.clone()
    }

    /// get the element id without bounds checking
    pub unsafe fn uget_mut(&mut self, idx: [usize;2]) -> &mut T {
        &mut *(self.data.as_mut_ptr().offset((idx[0] * self.strd + idx[1]) as isize))
    }

    /// Get reference to an element without bounds  checking
    pub fn get_elem_ref(&mut self, id: [usize; 2]) -> Option<&T> {
        let rid = id[0];
        let cid = id[1];

        if rid >= self.get_rows() || cid >= self.get_cols() {
            None
        } else {
            unsafe { Some(self.uget_mut(id)) }
        }
    }

    /// Get specified column unchecked
//    unsafe fn ucol(&self, id: usize) -> Column<T> {
//        let pt = self.as_ptr().offset(id as isize);
//        Column { col: 


    /// Transpose of a matrix
    pub fn transpose(&mut self) -> Matrix<'a, T> 
        where T: Copy + Float {
            match self.get_mode() {
                Column => {
                    let mut _data = Vec::with_capacity(self.get_cols() * 
                                                       self.get_rows());

                    unsafe {
                        _data.set_len(self.get_cols() * self.get_rows());

                        for i in 0..self.get_cols() {
                            for j in 0..self.get_rows() {
                                *_data.get_unchecked_mut(i * self.get_rows() + j) =
                                    *self.uget_mut([j,i]);
                            }
                        }
                    }
                    Matrix {
                        data: Vector { data: _data.to_vec() },
                        rows: self.get_cols(),
                        cols: self.get_rows(),
                        strd: self.get_rows(),
                        mode: self.mode.t(),
                        mark: PhantomData::<&'a T>,
                    }
                },

                Row => {
                    let mut _data = Vec::with_capacity(self.get_cols() * 
                                                       self.get_rows());

                    unsafe {
                        _data.set_len(self.get_cols() * self.get_rows());

                        for i in 0..self.get_rows() {
                            for j in 0..self.get_cols() {
                                *_data.get_unchecked_mut(i * self.get_cols() + j) =
                                    *self.uget_mut([j,i]);
                            }
                        }
                    }
                    Matrix {
                        data: Vector { data:_data.to_vec() },
                        rows: self.get_cols(),
                        cols: self.get_rows(),
                        strd: self.get_cols(),
                        mode: self.mode.t(),
                        mark: PhantomData::<&'a T>,
                    }
                }
            }
        }

    /// Get the index for the specified row and column ids
    #[inline]
    pub fn index(&self, rid: usize, cid: usize) -> Option<usize> {
        match self.mode {
            Axis::Column => {
                let (r, c) = (rid, cid);
                self.tridx(c * self.get_rows() + r)
            },
            Axis::Row => {
                let (r, c) = (cid, rid);
                Some(c * self.get_rows() + r)
            }
        }
    }
    
    #[inline]
    fn tridx(&self, id: usize) -> Option<usize> {
        let i = (id % self.get_cols()) * self.get_rows() + (id / self.get_cols()) as usize;
        Some(i)
    }

    /// Set the value at the specified location
    pub fn set(&mut self, rid: usize, cid: usize, val: T) {
        let i = self.index(rid, cid);
        let mut vals = self.data.as_mut_slice();
//        assert!(i < vals.len(), "Index out of bounds");
        
        match i {
            Some(value) => { vals[value] = val },
            None => { panic!("Index out of bounds") },
        }

    }

    /// Get the value from the specified location
    pub fn get(&self, rid: usize, cid: usize) -> Option<T> {
        let i = self.index(rid, cid);
        let vals = self.data.as_slice();
//        assert!(i < vals.len(), "Index out of bounds");
        match self.index(rid, cid) {
            Some(i) => { self.data.as_slice().get(i).map(|&n| n) },
            None => { panic!("Index out of bound") },
        }
    }


    ///Matrix constructor
    pub fn from_vec(dat: Vec<T>, rows: usize, cols: usize) -> Matrix<'a, T> {
        assert!(rows * cols == dat.len());
        Matrix {
            data: Vector { data: dat, },
            rows: rows,
            cols: cols,
            strd: cols,
            mode: Axis::Row,
            mark: PhantomData::<&'a T>,
        }
    }

    /// Matrix from function
    pub fn from_fn<F>(rows: usize, cols: usize, f: F) -> Matrix<'a, T> 
        where F: Fn(usize, usize) -> T {
            let mut dat = Vec::with_capacity(rows * cols);
            for i in 0..rows {
                for j in 0..cols {
                    dat.push(f(i,j))
                }
            }

            Matrix {
                data: Vector { data: dat, },
                rows: rows,
                cols: cols,
                strd: cols,
                mode: Axis::Row,
                mark: PhantomData::<&'a T>,
            }
        }

    /// Matrix with all 1's
    pub fn unit(rows: usize, cols: usize) -> Matrix<'a, T> 
        where T: Float {
            Matrix {
                data: Vector { data: vec![T::one(); rows * cols], },
                rows: rows,
                cols: cols,
                strd: cols,
                mode: Axis::Row,
                mark: PhantomData::<&'a T>,
            }
        }

    /// Zero Matrix
    pub fn zero(rows: usize, cols: usize) -> Matrix<'a, T> 
        where T: Float {
            Matrix {
                data: Vector { data: vec![T::zero(); rows * cols], },
                rows: rows,
                cols: cols,
                strd: cols,
                mode: Axis::Row,
                mark: PhantomData::<&'a T>,
            }
        }

    /// Diagonal matrix
    pub fn diag(vec: &Vec<T>, rows: usize, cols: usize) -> Matrix<'a, T> {
        let n = vec.len();
        let mut mat = Matrix {
            data: Vector { data: vec![T::zero(); n * n], },
            rows: n,
            cols: n,
            strd: n,
            mode: Axis::Row,
            mark: PhantomData::<&'a T>,
        };

        for i in 0..n {
            mat.set(i, i, vec[i]);
        }
        mat
    }

    /// Eigen matrix: Main diagonal with 1s
    pub fn eye(dim: usize) -> Matrix<'a, T> 
        where T: Float {
            Matrix::diag(&vec![T::one(); dim], dim, dim)
        }
}


    ///Print the matrix
    impl<'a, T: Float + Display + Debug> fmt::Display for Matrix<'a, T>
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.get_rows() {
                for j in 0..self.get_cols() {
                    write!(f, "{:1.5} ", self.get(i, j).unwrap());
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }





//=============================================================================
//Matrix Slice
//=============================================================================
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MatrixSlice<'a, T> {
    pt: *const T,
    nr: usize,
    nc: usize,
    rs: usize,
    _m: PhantomData<&'a T>,
}

//=============================================================================
//Mutable Matrix Slice
//=============================================================================
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MatrixMutSlice<'a, T> {
    pt: *mut T,
    nr: usize,
    nc: usize,
    rs: usize,
    _m: PhantomData<&'a T>,
}

//=============================================================================
//Immutable Row slice from matrix 
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct Row<'a, T> {
    row: MatrixSlice<'a, T>,
}

//=============================================================================
//Mutable Row Slice from matrix
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct RowMut<'a, T> {
    row: MatrixMutSlice<'a, T>,
}

//=============================================================================
//Immutable Row Iter
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct RowsIter<'a, T> {
    start_pos: *const T,
    row_pos: usize,
    row_slice: usize,
    col_slice: usize,
    row_stride: usize,
    _markr: PhantomData<&'a T>,
}

//=============================================================================
//Mutable Row Iter
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct RowsMutIter<'a, T> {
    start_pos: *mut T,
    row_pos: usize,
    row_slice: usize,
    col_slice: usize,
    row_stride: usize,
    _markr: PhantomData<&'a T>,
}

//=============================================================================
//Immutable Column slice from matrix
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct Col<'a, T> {
    col: MatrixSlice<'a, T>,
}

//=============================================================================
//Mutable column slice from matrix
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct ColMut<'a, T> {
    col: MatrixMutSlice<'a, T>,
}

//=============================================================================
//Immutable column iter
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct ColIter<'a, T> {
    start_pos: *const T,
    col_pos: usize,
    row_slice: usize,
    col_slice: usize,
    col_stride: usize,
    _markr: PhantomData<&'a T>,
}

//=============================================================================
//Mutable column iter
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct ColMutIter<'a, T> {
    start_pos: *mut T,
    col_pos: usize,
    row_slice: usize,
    col_slice: usize,
    col_stride: usize,
    _markr: PhantomData<&'a T>,
}

//=============================================================================
//Iterate over slice data immutably
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct SliceIter<'a, T> {
    slice: *const T,
    row_pos: usize,
    col_pos: usize,
    row_slice: usize,
    col_slice: usize,
    _markr: PhantomData<&'a T>,
}

//=============================================================================
//Iterate over slice data mutably
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct SliceMutIter<'a, T> {
    slice: *mut T,
    row_pos: usize,
    col_pos: usize,
    row_slice: usize,
    col_slice: usize,
    _markr: PhantomData<&'a T>,
}
