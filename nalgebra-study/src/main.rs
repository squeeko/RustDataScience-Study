use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, Matrix, Matrix3x4, OMatrix, Vector3};
extern crate nalgebra as na;
use nalgebra::SMatrix;

// Statically sized and statically allocated 2x3 matrix using 32-bit floats.
type Matrix2x3f = SMatrix<f32, 2, 3>;

// Half-dynamically sized and dynamically allocated matrix with
// two rows using 64-bit floats.
//
// The `OMatrix` alias is a matrix that owns its data (as opposed to
// matrix slices which borrow their data from another matrix).
type Matrix2xXf64 = OMatrix<f64, U2, Dynamic>;

// Dynamically sized and dynamically allocated matrix with
// two rows and using 32-bit signed integers.
type DMatrixi32 = OMatrix<i32, Dynamic, Dynamic>;

// Statically sized and statically allocated 1000x3 matrix using 32-bit floats.
type Matrix1000x3f = SMatrix<f32, 1000, 3>;



fn main() {

    // A Vector wtih three components
    let v = Vector3::new(1,2,3);
    println!("{:#?}", v);

    // A Matrix with three lines(rows) and four columns.
    // We chose values such that, for example, 23 is at row 2 and column 3
    let m = Matrix3x4::new(11, 12, 13, 14,
                           21, 22, 23, 24,
                           31, 32, 33, 34);
                           println!("{:#?}", m);
    
}
