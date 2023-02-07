use nalgebra::*;
use ndarray::array;
use polars::prelude::*;
use std::error::Error;

extern crate nalgebra as na;

use smartcore::linalg::basic::arrays::Array2;
use smartcore::linalg::basic::matrix::DenseMatrix;
// KNNClassifier
use smartcore::neighbors::knn_classifier::*;
// Various distance metrics
use smartcore::metrics::distance::*;

fn main() {
    let df = CsvReader::from_path("/Users/squeeko/RustDataScience-Study/data/MalwareArtifacts.csv")
        .expect("cannot read csv")
        .infer_schema(None)
        .has_header(true)
        .finish();

    // println!("{:#?}", df);
    // let x = df.expect("This is not workin").select_at_idx(0);

    // let binding = df.expect("This is not workin");
    // let x = binding.select_at_idx(0);

    let x_matrix = df.as_ref().expect("This is working").select_by_range(0..=7);
    let y_vector = df.as_ref().expect("This is working").select_by_range(8..9);

    let y_df = y_vector
        .expect("this is working")
        .to_ndarray::<Int64Type>()
        .unwrap();

    let x_df = x_matrix
        .expect("this is working")
        .to_ndarray::<Int64Type>()
        .unwrap();

    let x = DMatrix::from_iterator(137444, 8, x_df);
    println!("{}", x);
    let y = vec![y_df];
    //println!("{:#?}", y);
    // let knn = KNNClassifier::fit(&x, &y, Default::default()).unwrap();
}

// let x = array![x_df];

// println!("{}", x);

//

// ndarray::data_repr::OwnedRepr<i64>
