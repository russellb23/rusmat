// Basic data structures:i Matrix and Vector

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

use std::rc::Rc;

use std::ops::Range;

use std::iter::Iterator;

use std::error::Error;
use std::io::Result;
use std::io::ErrorKind;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

use num::{Float, Zero, One};
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
pub enum Major {
    /// Column major
    Column,
    /// Row major
    Row,
}

impl Major {
    #[inline]
    pub fn t(&self) -> Major {
        match *self {
            Major::Column => Major::Row,
            Major::Row => Major::Column,
        }
    }
}

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

/// Matrix strucure
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T: Float + FromPrimitive> {
    /// Reference to the underlying data storage
    pub data: Rc<MatData<T>>,
    pub vdim: Dim,
    pub majr: Major,
}

impl<T: Float + FromPrimitive + Debug + Display> Features<T> for Matrix<T> {
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
        if self.vdim.nrows() == 0 || self.vdim.ncols() == 0 { true } else { false }
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

        match self.majr {
            Major::Column => {
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
                majr: self.majr.t(),
            }
            }
            Major::Row => {
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
                    majr: self.majr.t(),
            }
            }
        }
    }

    fn t(&self) -> Self {
        self.transpose()
    }
}

impl<T: Float + FromPrimitive> Matrix<T> {
    /// Construct matrix from given vector of specified dimension
    pub fn from_vec(data: Vec<T>, nrow: usize, ncol: usize) -> Matrix<T> {
        assert_eq!(data.len(), nrow * ncol);
        Matrix {
            data: Rc::new(MatData {
                d: RefCell::new(data),
                r: nrow,
                c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            majr: Major::Column,
        }
    }
    /// Construct matrix from given function of specified dimension
    pub fn from_fn<F>(nrow: usize, ncol: usize, mut f: F) -> Matrix<T> 
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
                majr: Major::Column,
            }
        }
    /// Construct an unit Matrix: Matrix with all its contents 1
    pub fn unit(nrow: usize, ncol: usize) -> Matrix<T> {
            Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(vec![T::one(); nrow * ncol]),
                    r: nrow,
                    c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            majr: Major::Column,
            }
        }
    /// Construct a zero matrix of specified dimension
    pub fn zero(nrow: usize, ncol: usize) -> Matrix<T> {
            Matrix {
                data: Rc::new(MatData {
                    d: RefCell::new(vec![T::zero(); nrow * ncol]),
                    r: nrow,
                    c: ncol,
            }),
            vdim: Dim::full(nrow, ncol),
            majr: Major::Column,
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
            majr: Major::Column,
            };
            for i in 0..n {
                mat.set(i, i, vec[i]);//.expect("index not valid");
            }
            mat
        }
    /// Construct an eigen matrix(Identity matrix): 1s at main diagonal
    pub fn eye(n: usize) -> Matrix<T> {
            Matrix::diag(&vec![T::one(); n])
        }
    
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
        match self.majr {
            Major::Column => {
                let (r, c) = (self.vdim.start_col() + r, self.vdim.start_row() + c);
                self.tridx(c * self.data.c + r)
            },
            Major::Row => {
                let (r, c) = (self.vdim.start_row() + c, self.vdim.start_col() + r );
                c * self.data.r + r
            }
        }
    }

    pub fn tridx(&self, idx: usize) -> usize {
        (idx % self.data.c) * self.data.r + 
                ( idx / self.data.c ) as usize
    }

    pub fn nrows(&self) -> usize {
        match self.majr {
            Major::Column => { self.vdim.ncols() },
            Major::Row => { self.vdim.nrows() },
        }
    }

    pub fn ncols(&self) -> usize {
        match self.majr {
            Major::Column => { self.vdim.nrows() },
            Major::Row => { self.vdim.ncols() },
        }
    }

//    /// Returns the shape/dimension of the matrix as a tuple (row, col)
//    pub fn shape(&self) -> (usize, usize) {
//        (self.vdim.nrows(), self.vdim.ncols())
//    }
//    /// Returns the total number of elements of the matrix
//    pub fn size(&self) -> usize {
//        self.data.vals().len()
//    }
//    /// Returns the minimum of the dimension among rows and columns
//    pub fn mindim(&self) -> usize {
//        self.vdim.nrows().min(self.vdim.ncols())
//    }
//
//    pub fn is_square(&self) -> bool { self.vdim.nrows() == self.vdim.ncols() }
//
//    pub fn is_vec(&self) -> bool { self.vdim.nrows() == 1 || self.vdim.ncols() == 1 }
//
//    pub fn is_rvec(&self) -> bool { self.vdim.nrows() == 1 }
//
//    pub fn is_cvec(&self) -> bool { self.vdim.ncols() == 1 }

//    pub fn transpose(&self) -> Matrix<T> {
//        let nr = self.nrows();
//        let nc = self.ncols();
//        assert_eq!(nr * nc, self.data.vals().len());
//        let l = nr * nc - 1;
//        let mut _data = self.data.vals().clone();
//
//        match self.majr {
//            Major::Column => {
//                for i in 0..l {
//                    _data[i] = self.data.vals()[(i * nc) % l];
//                }
//                Matrix {
//                data: Rc::new(MatData {
//                    d: RefCell::new(_data),
//                    r: nc,
//                    c: nr,
//                }),
//                vdim: Dim::full(self.nrows(), self.ncols()),
//                majr: self.majr.t(),
//            }
//            }
//            Major::Row => {
//                for i in 0..l {
//                    _data[i] = self.data.vals()[(i * nr) % l];
//                }
//                Matrix {
//                    data: Rc::new(MatData {
//                        d: RefCell::new(_data),
//                        r: nc,
//                        c: nr,
//                    }),
//                    vdim: Dim::full(self.ncols(), self.nrows()),
//                    majr: self.majr.t(),
//            }
//            }
//        }
//    }
//
//    #[inline]
//    pub fn t(&self) -> Matrix<T> { self.transpose() }
//
//    }
    
//    impl<T: Display> fmt::Display for Vec<T> {
//        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//            where T: Float {
//                for i in 0..self.len() {
//                    write!(f, "{:+1.5} ", self[i]);
//                }
//            }
//    }
}
    /// Print the matrix TODO: Pretty print
    impl<T: Float + Display + Debug + FromPrimitive > fmt::Display for Matrix<T> {
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

//impl<T: Float + Clone + Copy> Matrix<T> {
//    fn clone(&self) -> Matrix<T> 
//        where T: Float {
//            Matrix::from_vec(self.d.borrow().iter().collect(), self.r, self.c)
//        }
//}

/// Matrix Iterators
pub struct MatrixIter<'a, T: Float + Debug + FromPrimitive> {
    pub mat: &'a Matrix<T>,
    pub cur_loc: (usize, usize),
}


impl<'a, T: Float + Debug + FromPrimitive> Iterator for MatrixIter<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.cur_loc.1 >= self.mat.vdim.ncols() { return None }

            let val = self.mat.get(self.cur_loc.0, self.cur_loc.1);

            self.cur_loc.0 += 1;
            if self.cur_loc.0 >= self.mat.vdim.nrows() {
                self.cur_loc.0 = 0;
                self.cur_loc.1 += 1;
            }
            val
        }
    }
