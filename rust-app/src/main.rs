use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::model_selection::train_test_split;

// metrics
use smartcore::metrics::mean_squared_error;
use smartcore::metrics::accuracy;

use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

use polars::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader;


// CSVファイルを読み込んでDataFrameを返す
fn read_csv_with_schema<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    let schema = Schema::new(vec![
        Field::new("species", DataType::Utf8),
        Field::new("island", DataType::Utf8),
        Field::new("culmen_length_mm", DataType::Float64),
        Field::new("culmen_depth_mm", DataType::Float64),
        Field::new("flipper_length_mm", DataType::Float64),
        Field::new("body_mass_g", DataType::Float64),
        Field::new("sex", DataType::Utf8)
    ]);
    let file = File::open(path).expect("Cannot open file.");
    CsvReader::new(file)
        .with_schema(Arc::new(schema))
        .has_header(true)
        .with_ignore_parser_errors(true) //エラーが出ても処理継続
        .finish()
}