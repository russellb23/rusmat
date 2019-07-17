extern crate num;

use num::traits::Float;
use num::traits::FromPrimitive;

use std::marker::PhantomData;

//use super::matrix::{Matrix, MatrixSlice, MatrixMutSlice};
use super::matrix::{Matrix};

//=============================================================================
//Matrix Slice
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct MatrixSlice<'a, T> {
    pt: *const T,
    nr: usize,
    nc: usize,
    rs: usize,
    _m: PhantomData<&'a T>,
}

impl<'a, T> MatrixSlice<'a, T> {

    fn get_rows(&self) -> usize {
        self.nr
    }

    fn get_cols(&self) -> usize {
        self.nc
    }

    fn row_stride(&self) -> usize {
        self.rs
    }
    fn as_ptr(&self) -> *const T {
        self.pt
    }


    pub fn from_matrix(mat: &'a Matrix<T>, begin: [usize; 2], nr: usize, 
                                                                    nc: usize)
        -> MatrixSlice<'a, T> 
        where T: Float + FromPrimitive {
            assert!(begin[0] + nr <= nr, "View dimensions exceed matrix
                                                                dimensions");
            assert!(begin[1] + nc <= nc, "View dimentions exceed matrix
                                                                dimensions");

            unsafe {
                MatrixSlice {
                    pt: mat.get_data().get_unchecked(begin[0] *
                                        mat.get_cols() + begin[1]) as *const T,
                    nr: nr,
                    nc: nc,
                    rs: mat.get_cols(),
                    _m: PhantomData::<&'a T>,
                }
            }
        }

    pub unsafe fn from_raw_parts(ptr: *const T, nr: usize, nc: usize, 
                                 row_stride: usize) -> MatrixSlice<'a, T> {
        MatrixSlice {
            pt: ptr,
            nr: nr,
            nc: nc,
            rs: row_stride,
            _m: PhantomData::<&'a T>,
        }
    }

}
//=============================================================================
//Mutable matrix slice
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct MatrixMutSlice<'a, T> {
    pt: *mut T,
    nr: usize,
    nc: usize,
    rs: usize,
    _m: PhantomData<&'a T>,
}

impl<'a, T> MatrixMutSlice<'a, T> {
    fn get_rows(&self) -> usize {
        self.nr
    }

    fn get_cols(&self) -> usize {
        self.nc
    }

    fn row_stride(&self) -> usize {
        self.rs
    }

    fn as_ptr(&self) -> *mut T {
        self.pt
    }

    pub fn from_matrix(mat: &'a mut Matrix<T>, begin: [usize; 2], 
                       nr: usize, nc: usize) -> MatrixMutSlice<'a, T> 
        where T: Float + FromPrimitive {
            assert!(begin[0] + nr <= nr, "View dimensions exceed matrix
                                dimensions");
            assert!(begin[1] + nc <= nc, "View dimentions exceed matrix
                                dimensions");
            let ncols = mat.clone().get_cols();
            unsafe {
                MatrixMutSlice {
                    pt: mat.get_mut_data().get_unchecked_mut(begin[0] * 
                                    ncols + begin[1]) as *mut T,
                    nr: nr,
                    nc: nc,
                    rs: mat.get_cols(),
                    _m: PhantomData::<&'a T>,
                }
            }
        }

    pub unsafe fn from_raw_parts(ptr: *mut T, nr: usize, nc: usize, row_stride: usize) -> MatrixMutSlice<'a, T> {
        MatrixMutSlice {
            pt: ptr,
            nr: nr,
            nc: nc,
            rs: row_stride,
            _m: PhantomData::<&'a T>,
        }
    }

}

//=============================================================================
//Row as a slice from matrix
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct Row<'a, T> {
    row: MatrixSlice<'a, T>,
}

impl<'a, T> Row<'a, T> {
    /// Returns a complete row as a slice
    fn row_slice(&self) -> &'a [T] {
        unsafe {
            std::slice::from_raw_parts(self.row.as_ptr(), self.row.get_cols())
        }
    }
}

//=============================================================================
//Mutable row as a slice from matrix
//=============================================================================
#[derive(Debug, Clone, Copy)]
pub struct RowMut<'a, T> {
    row: MatrixMutSlice<'a, T>,
}

impl<'a, T> RowMut<'a, T> {
    /// Returns the specified row as a mutable slice
    fn row_mut_slice(&self) -> &'a [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.row.as_ptr(), 
                                                        self.row.get_cols())
        }
    }
}

/// Immutable Column Iter
#[derive(Debug, Clone, Copy)]
pub struct Col<'a, T> {
    col: MatrixSlice<'a, T>,
}

/// Mutable column iter
#[derive(Debug, Clone, Copy)]
pub struct ColMut<'a, T> {
    col: MatrixMutSlice<'a, T>,
}

