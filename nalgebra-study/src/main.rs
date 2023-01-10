use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, Matrix, Matrix2x3,
     Matrix3x4, OMatrix, RowVector3, Vector2, Vector3};

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
    let m = Matrix2x3::new(1.1, 1.2, 1.3,
                           2.1, 2.2, 2.3);
                           println!("{:#?}", m);


    // All the following matrices are equal but constructed in different ways.
    
    let m = Matrix2x3::new(1.1, 1.2, 1.3,
                           2.1, 2.2, 2.3);

    let m1 = Matrix2x3::from_rows(&[
        RowVector3::new(1.1, 1.2, 1.3),
        RowVector3::new(2.1, 2.2, 2.3)
    ]);

    let m2 = Matrix2x3::from_columns(&[
        Vector2::new(1.1, 2.1),
        Vector2::new(1.2, 2.2),
        Vector2::new(1.3, 2.3)
    ]);

    let m3 = Matrix2x3::from_row_slice(&[
        1.1, 1.2, 1.3,
        2.1, 2.2, 2.3
    ]);

    let m4 = Matrix2x3::from_column_slice(&[
        1.1, 2.1,
        1.2, 2.2,
        1.3, 2.3
    ]);

    let m5 = Matrix2x3::from_fn(|r,c| (r + 1) as f32 + (c + 1) as f32 / 10.0);

    let m6 = Matrix2x3::from_iterator([1.1f32, 2.1, 1.2, 2.2, 1.3, 2.3].iter().cloned());

    assert_eq!(m, m1);
    assert_eq!(m, m2);
    assert_eq!(m, m3);
    assert_eq!(m, m4);
    assert_eq!(m, m5);
    assert_eq!(m, m6);

// All the following matrices are equal but constructed in different ways.
//
// This time, we used a dynamically-sized matrix so we must specify the
// dimensions of the matrix with the first two arguments.


    
}
