use polars::prelude::*;


fn main() {
    let df = CsvReader::from_path("/Users/jallgood/RustDataScience/data/MalwareArtifacts.csv").expect("cannot read csv")
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
 let x = df.as_ref().expect("no slice_par").select_by_range(0..=7);
 println!("{:#?}", x.expect("No types").dtypes());

//  let y = df.as_ref().expect("no slice_par").select_by_range(8..=8);
//  println!("{:#?}", y);

 

   
}
