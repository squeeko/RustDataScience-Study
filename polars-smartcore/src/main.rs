
use polars::prelude::*;
use smartcore::linalg::basic::matrix::DenseMatrix;



// use smartcore::linalg::basic::{matrix::DenseMatrix, arrays::MutArray};


fn main() {
    let df = CsvReader::from_path("/Users/jallgood/RustDataScience-Study/data/MalwareArtifacts.csv")
        .expect("cannot read csv")
        .infer_schema(None)
        .has_header(true)
        .finish();

        // println!("{:#?}", df.as_ref().expect("Cannot show head").head(Some(5)));

        let features = df.as_ref().expect("Could not create vector").select(vec![
            "AddressOfEntryPoint",
            "MajorLinkerVersion",
            "MajorImageVersion",
            "MajorOperatingSystemVersion",
            "DllCharacteristics",
            "SizeOfStackReserve",
            "NumberOfSections",
            "ResourceSize",
            ]);

        // println!("{:#?}", features);

        let target = df.as_ref().expect("Could not create vector").select(vec![
            "legitimate",
            ]).unwrap();

            println!("{:#?}", target.dtypes());
        
        
            
            
            println!("{:#?}", target);

       


    //    println!("{:#?}", df.as_ref().expect("no columns").get_column_names_owned());
    // println!("{:?}", df.as_ref().expect("No shape").shape());
    // println!("{:?}", df.as_ref().expect("no height").height()); // rows
    // println!("{:?}", df.as_ref().expect("no width").width() - 1); // cols
    //    println!("{:?}", df.as_ref().expect("no slices").slice(0, 5));
    //    println!("{:?}", df.as_ref().expect("no series").select_series(["AddressOfEntryPoint"]));
    //    println!("{:#?}", df.as_ref().expect("no slice_par").select_by_range(0..=8).expect("shape").shape());
    // println!("{:#?}", df.as_ref().expect("no slice_par").select_by_range(0..=7));
    // let x_df = df.as_ref().expect("no slice_par").select_by_range(0..=7);
    // println!("{:#?}", x_df.as_ref().expect("No x_df head").head(Some(2)));
    // // println!("{:#?}", x_df.as_ref().expect("No types").dtypes());

    // let y_df = df.as_ref().expect("no slice_par").select_by_range(8..=8);
    // println!("{:#?}", y_df.as_ref().expect("No x_df head").head(Some(2)));

    //let xmatrix = Matrix::from_data(df);

    // let x_ndarray = x_df
    //     .as_ref()
    //     .expect("Cannot convert to ndarray")
    //     .to_ndarray::<Float64Type>()
    //     .unwrap();
    // println!("{:#?}", x_ndarray);

    // let y_ndarray = y_df
    //     .as_ref()
    //     .expect("Cannot convert to ndarray")
    //     .to_ndarray::<Float64Type>()
    //     .unwrap();
    // println!("{:#?}", y_ndarray);
}

   
