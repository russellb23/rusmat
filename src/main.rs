extern crate dev_matrixlib;

use std::mem;

use self::dev_matrixlib::matrix::Matrix;
//use self::dev_matrixlib::matrix::Features;
use self::dev_matrixlib::matrix::MatrixSlice;

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    loop {
        if a == 0 { return b }
        if b == 0 { return a }

        if a < b {
            mem::swap(&mut a, &mut b);
        } else {
            a = a % b;
        }
    }
}


fn main() {
    let vv = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.];
    let m = Matrix::from_vec(vv, 3,3);
    let i: Matrix<f32> = Matrix::eye(5);
    let j: Matrix<f32> = Matrix::unit(3,2);
    let k: Matrix<f32> = Matrix::zero(2,3);

    println!("{}", &m);
    println!("{}", i);
//    println!("{}", j);
//    println!("{}", k);
//    println!("{}", m.transpose());

    let x = 98;
    let y = 56;

    let p = gcd(x,y);
    println!("GCD of {}, {} == {}", x,y,p);

//    for i in m.into_iter() {
//        println!("I: {}", &i);
//    }

}
