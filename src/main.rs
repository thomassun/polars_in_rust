// use chrono::prelude::*;
use polars::lazy::dsl::*;
use polars::prelude::*;
// use rand::{thread_rng, Rng};
fn main() -> PolarsResult<()> {
    let df = LazyCsvReader::new("./dataset/apple_stock.csv")
        .with_try_parse_dates(true)
        .finish()?
        .sort(
            "Date",
            SortOptions {
                ..Default::default()
            },
        );
    let out = df
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
        // .explain(true)?;
        .with_streaming(true)
        .collect()?;

    println!("{}", &out);
    // println!("{}", &df.collect()?);
    Ok(())
}
