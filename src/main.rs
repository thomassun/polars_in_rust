// use chrono::prelude::*;
use polars::lazy::dsl::*;
use polars::prelude::{Literal, *};
// use rand::{thread_rng, Rng};
fn main() -> PolarsResult<()> {
    let df = CsvReader::from_path("./dataset/apple_stock.csv")
        .unwrap()
        .with_try_parse_dates(true)
        .finish()
        .unwrap()
        .sort(["Date"], false, true)?;
    let out = df
        .clone()
        .lazy()
        .group_by_dynamic(
            col("Date"),
            [],
            DynamicGroupOptions {
                every: Duration::parse("6mo"),
                period: Duration::parse("6mo"),
                offset: Duration::parse("0"),
                ..Default::default()
            },
        )
        .agg([
            col("Close").mean(),
            col("Date").first().alias("Start"),
            col("Date").last().alias("End"),
        ])
        .with_columns([
            // col("Date").first().alias("Start"),
            // col("Date").last().alias("End"),
        ])
        .collect()?;
    println!("{}", &out);
    Ok(())
}
