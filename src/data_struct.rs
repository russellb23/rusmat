// Basic data structures:i Matrix and Vector

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use std::ops::Range;

use std::iter::{Iterator, IntoIterator};

//use std::error::Error;
//use std::io::Result;
//use std::io::ErrorKind;
use std::fmt;
use std::fmt::{Debug, Display};

use std::marker::PhantomData;

use num::{Float};
use num::traits::cast::FromPrimitive;

//=============================================================================
// Implementation of traits for Matrix and Vector
// ============================================================================

pub trait Features<T: Float + Debug + Display + FromPrimitive> {
    fn shape(&self) -> (usize, usize);
    fn size(&self) -> usize;
    fn mindim(&self) -> usize;
    fn is_vec(&self) -> bool;
    fn is_rvec(&self) -> bool;
    fn is_cvec(&self) -> bool;
    fn transpose(&self) -> Self;
    fn t(&self) -> Self;
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
/// Matrix storage pattern: Column major or Row major
pub enum Axes {
    /// Column major
    Column,
    /// Row major
    Row,
}

impl Axes {
    #[inline]
    pub fn t(&self) -> Axes {
        match *self {
            Axes::Column => Axes::Row,
            Axes::Row => Axes::Column,
        }
    }
}

//=============================================================================
//Matrix dimensions
//=============================================================================
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dim (pub Range<(usize, usize)>);

impl Dim {
    /// Generate matrix with entire range
    pub fn full(r: usize, c: usize) -> Dim {
        Dim((0,0)..(r,c))
    }

    fn nrows(&self) -> usize {
        self.0.end.0 - self.0.start.0
    }

    fn ncols(&self) -> usize {
        self.0.end.1 - self.0.start.1
    }

    fn start_row(&self) -> usize {
        self.0.start.0
    }

    fn start_col(&self) -> usize {
        self.0.start.1
    }
}

//=============================================================================
//Matrix data
//=============================================================================
/// Fundamental matrix structure using RefCell: shareable mutable container ->
/// interior mutability using references
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MatData<T> {
    r: usize,
    c: usize,
    d: RefCell<Vec<T>>,
}

impl<T> MatData<T> 
    where T: Float {
    /// borrow the underlying matrix data
    pub fn vals(&self) -> Ref<Vec<T>> {
        self.d.borrow()
    }
    /// borrow the underlying matrix data mutably
    pub fn vals_mut(&self) -> RefMut<Vec<T>> {
        self.d.borrow_mut()
    }
    }

//=============================================================================
//Matrix
//=============================================================================
/// Matrix strucure
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T: Float + FromPrimitive> {
    /// Reference to the underlying data storage
    pub data: Rc<MatData<T>>,
    pub vdim: Dim,
    pub axis: Axes,
}

//=============================================================================
//Matrix traits
//=============================================================================
impl< T: Float + FromPrimitive + Debug + Display> Features<T> for Matrix<T> {
    fn shape(&self) -> (usize, usize) {
        (self.vdim.nrows(), self.vdim.ncols())
    }

    fn size(&self) -> usize {
        self.data.vals().len()
    }

    fn mindim(&self) -> usize {
        (self.vdim.nrows()).min(self.vdim.ncols())
    }

    fn is_vec(&self) -> bool {
        if self.vdim.nrows() == 0 || self.vdim.ncols() == 0 { true } else 
                                                            { false }
    }

    fn is_rvec(&self) -> bool {
        if self.vdim.nrows() == 1 { true } else { false }
    }

    fn is_cvec(&self) -> bool {
        if self.vdim.ncols() == 1 { true } else { false }
    }

    fn transpose(&self) -> Self {
        let nr = self.nrows();
        let nc = self.ncols();
        assert_eq!(nr * nc, self.data.vals().len());
        let l = nr * nc - 1;
        let mut _data = self.data.vals().clone();

        match self.axis {
            Axes::Column => {
                for i in 0..l {
                    _data[i] = self.data.vals()[(i * nc) % l];
                }
                Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(_data),
                    r: nc,
                    c: nr,
                }),
                vdim: Dim::full(self.nrows(), self.ncols()),
                axis: self.axis.t(),
            }
            }
            Axes::Row => {
                for i in 0..l {
                    _data[i] = self.data.vals()[(i * nr) % l];
                }
                Matrix {
                    data: Rc::new(MatData {
                        d: RefCell::new(_data),
                        r: nc,
                        c: nr,
                    }),
                    vdim: Dim::full(self.ncols(), self.nrows()),
                    axis: self.axis.t(),
            }
            }
        }
    }

    fn t(&self) -> Self {
        self.transpose()
    }
}

//=============================================================================
//Matrix constrction
//=============================================================================
impl< T: Float + FromPrimitive + Clone> Matrix< T> {
    /// Construct matrix from given vector of specified dimension
    pub fn from_vec(data: Vec<T>, nrow: usize, ncol: usize) -> Matrix< T> {
        assert_eq!(data.len(), nrow * ncol);
        Matrix {
            data: Rc::new(MatData {
                d: RefCell::new(data),
                r: nrow,
                c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            axis: Axes::Column,
        }
    }
    /// Construct matrix from given function of specified dimension
    pub fn from_fn<F>(nrow: usize, ncol: usize, mut f: F) -> Matrix< T> 
        where F: FnMut(usize, usize) -> T {
            let mut d = Vec::with_capacity(nrow * ncol);
            for i in 0..nrow {
                for j in 0..ncol {
                    d.push(f(i,j))
                }
            }
            Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(d),
                    r: nrow,
                    c: ncol,
                }),
                vdim: Dim::full(nrow, ncol),
                axis: Axes::Column,
            }
        }

//=============================================================================
//Predefined matrices of required dimensions
//=============================================================================
    /// Construct an unit Matrix: Matrix with all its contents 1
    pub fn unit(nrow: usize, ncol: usize) -> Matrix< T> {
            Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(vec![T::one(); nrow * ncol]),
                    r: nrow,
                    c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            axis: Axes::Column,
            }
        }
    /// Construct a zero matrix of specified dimension
    pub fn zero(nrow: usize, ncol: usize) -> Matrix< T> {
            Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(vec![T::zero(); nrow * ncol]),
                    r: nrow,
                    c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            axis: Axes::Column,
            }
        }
    /// Construct a diagonal matrix with elements of a vector
    pub fn diag(vec: &Vec<T>) -> Matrix<T> {
            let n = vec.len();
            let mut mat = Matrix {
                data: Rc::new(MatData {
                    d: (RefCell::new(vec![T::zero(); n * n])),
                    r: n,
                    c: n,
            }),
            vdim: Dim::full(n, n),
            axis: Axes::Column,
            };
            for i in 0..n {
                mat.set(i, i, vec[i]);//.expect("index not valid");
            }
            mat
        }
    /// Construct an eigen matrix(Identity matrix): 1s at main diagonal
    pub fn eye(n: usize) -> Matrix< T> {
            Matrix::diag(&vec![T::one(); n])
        }
    
//=============================================================================
//Accessing matrix elements and methods for matrix rows and columns
//=============================================================================
    /// Set a value at specified location of the matrix
    pub fn set(&mut self, rid: usize, cid: usize, val: T) {
        let i = self.index(rid, cid);
        let mut vals = self.data.vals_mut();
        assert!(i < vals.len(), "Index out of bounds");

        vals[i] = val;
    }
    /// Get the value from the specified location of the matrix
    pub fn get(&self, rid: usize, cid: usize) -> Option<T> {
            let i = self.index(rid, cid);
            let vals = self.data.vals();
            assert!(i < vals.len(), "Index out of bounds");

            let val = self.data.vals().get(self.index(rid, cid)).map(|&n| n);
            val
        }
    /// Get the index providing row and column in a column major matrix
    #[inline]
    pub fn index(&self, r: usize, c: usize) -> usize {
        match self.axis {
            Axes::Column => {
                let (r, c) = (self.vdim.start_col() + r, 
                                                    self.vdim.start_row() + c);
                self.tridx(c * self.data.c + r)
            },
            Axes::Row => {
                let (r, c) = (self.vdim.start_row() + c, 
                                                    self.vdim.start_col() + r);
                c * self.data.r + r
            }
        }
    }

    pub fn tridx(&self, idx: usize) -> usize {
        (idx % self.data.c) * self.data.r + 
                ( idx / self.data.c ) as usize
    }

    pub fn nrows(&self) -> usize {
        match self.axis {
            Axes::Column => { self.vdim.ncols() },
            Axes::Row => { self.vdim.nrows() },
        }
    }

    pub fn ncols(&self) -> usize {
        match self.axis {
            Axes::Column => { self.vdim.nrows() },
            Axes::Row => { self.vdim.ncols() },
        }
    }
}

//=============================================================================
// Matrix Print display 
// TODO: Pretty print
// ============================================================================
    /// Print the matrix TODO: Pretty print
    impl<T: Float + Display + Debug + FromPrimitive> fmt::Display for Matrix<T>
    {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
        where T: Float + Debug {
        for i in 0..self.vdim.nrows() {
            for j in 0..self.vdim.ncols() {
                write!(f, "{:1.5} ", self.get(i, j).unwrap());
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

//=============================================================================
// Matrix into interator
// ============================================================================
impl<T: Float + Debug + FromPrimitive> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = MatIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MatIntoIterator {
            mat: self,
            index: (0,0),
        }
    }
}

pub struct MatIntoIterator<T: Float + Debug + FromPrimitive> {
    mat: Matrix<T>,
    index: (usize, usize),
}


impl< T: Float + Debug + FromPrimitive> Iterator for MatIntoIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.index.1 >= self.mat.vdim.ncols() { return None }

            let val = self.mat.get(self.index.0, self.index.1);

            self.index.0 += 1;
            if self.index.0 >= self.mat.vdim.nrows() {
                self.index.0 = 0;
                self.index.1 += 1;
            }
            val
        }
}
//=============================================================================
//Matrix Slice
//=============================================================================

#[derive(Debug, Clone, Copy)]
pub struct MatrixSlice<'a, T> {
    ptr: *const T,
    nr: usize,
    nc: usize,
    r_stride: usize,
    _markr: PhantomData<&'a T>,
}

#[derive(Debug, Clone, Copy)]
pub struct MatrixSliceMut<'a, T> {
    ptr: *mut T,
    nr: usize,
    nc: usize,
    r_stride: usize,
    _markr: PhantomData<&'a T>,
}

#[derive(Debug, Clone, Copy)]
pub struct Row<'a, T> {
    row: MatrixSlice<'a, T>,
}

#[derive(Debug, Clone, Copy)]
pub struct RowMut<'a, T> {
    row: MatrixSliceMut<'a, T>,
}

/// Immutable row interator
pub struct RowIter<'a, T> {
    start: *const T,
    r_pos: usize,
    sr: usize,
    sc: usize,
    r_stride: usize,
    _markr: PhantomData<&'a T>,
}

/// Mutable row iterator
pub struct RowIterMut<'a, T> {
    start: *mut T,
    r_pos: usize,
    sr: usize,
    sc: usize,
    r_stride: usize,
    _markr: PhantomData<&'a T>,
}

