extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use csv::ReaderBuilder;
use linfa::prelude::SingleTargetRegression;
use linfa::prelude::*;
use linfa::traits::{Fit, Predict};
use linfa::Dataset;
use linfa_bayes::{GaussianNbParams, GaussianNbValidParams};
use linfa_linear::LinearRegression;
use ndarray::{s, Array2};
use ndarray_csv::Array2Reader;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/squeeko/RustDataScience-Study/data/MalwareArtifacts.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let data_array: Array2<f64> = reader.deserialize_array2_dynamic()?;

    let data = data_array.slice(s![.., 0..8]).to_owned();
    let target = data_array.column(8).to_owned();

    // let ds = Dataset::new(data, target);
    let ds = DatasetView::new(data.view(), target.view());

    // let model = LinearRegression::default().fit(&ds).unwrap();
    // let pred = model.predict(&ds);
    // let r2 = pred.r2(&ds).unwrap();
    // println!("r2 from predictions {}", r2);

    Ok(())
}
