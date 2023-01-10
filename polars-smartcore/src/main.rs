use polars::frame::DataFrame;
use polars::prelude::*;
use polars_core::prelude::*;

fn main() {
    let df = CsvReader::from_path("/Users/squeeko/RustDataScience/data/MalwareArtifacts.csv")
        .expect("cannot read csv")
        .infer_schema(None)
        .has_header(true)
        .finish();
    //    println!("{:#?}", df.as_ref().expect("no columns").get_column_names_owned());
    // println!("{:?}", df.as_ref().expect("No shape").shape());
    // println!("{:?}", df.as_ref().expect("no height").height()); // rows
    // println!("{:?}", df.as_ref().expect("no width").width() - 1); // cols
    //    println!("{:?}", df.as_ref().expect("no slices").slice(0, 5));
    //    println!("{:?}", df.as_ref().expect("no series").select_series(["AddressOfEntryPoint"]));
    //    println!("{:#?}", df.as_ref().expect("no slice_par").select_by_range(0..=8).expect("shape").shape());
    // println!("{:#?}", df.as_ref().expect("no slice_par").select_by_range(0..=7));
    let x_df = df.as_ref().expect("no slice_par").select_by_range(0..=7);
    println!("{:#?}", x_df.as_ref().expect("No x_df head").head(Some(5)));
    // println!("{:#?}", x_df.as_ref().expect("No types").dtypes());

    let y_df = df.as_ref().expect("no slice_par").select_by_range(8..=8);
    println!("{:#?}", y_df.as_ref().expect("No x_df head").head(Some(5)));

    let x_ndarray = x_df
        .as_ref()
        .expect("Cannot convert to ndarray")
        .to_ndarray::<Float64Type>()
        .unwrap();
    println!("{:#?}", x_ndarray);

    let y_ndarray = y_df
        .as_ref()
        .expect("Cannot convert to ndarray")
        .to_ndarray::<Float64Type>()
        .unwrap();
    println!("{:#?}", y_ndarray);
}
