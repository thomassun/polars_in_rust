use polars::prelude::*;
fn main() -> PolarsResult<()> {
    let df = CsvReader::from_path("./dataset/apple_stock.csv")?
        .has_header(true)
        .finish()?;
    println!("{:?}", df.describe(None));
    Ok(())
}
