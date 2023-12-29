// use chrono::prelude::*;
use polars::lazy::dsl::*;
use polars::prelude::*;
use rand::{thread_rng, Rng};
fn main() -> PolarsResult<()> {
    let mut arr = [0f64; 5];
    thread_rng().fill(&mut arr);
    let df = df![
        "nrs" => &[Some(1),Some(2),Some(3),None,Some(5)],
        "names" => &[Some("foo"),Some("ham"),Some("spam"),Some("eggs"),None],
        "random" => &arr,
        "groups" => &["A","A","B","C","B"],

    ]?;
    let out = df
        .clone()
        .lazy()
        .group_by([col("groups")])
        .agg([
            sum("nrs"),
            col("random").count().alias("Count"),
            col("random")
                .filter(col("names").is_not_null())
                .sum()
                .name()
                .suffix("_sum"),
            // col("names").reverse().alias("reversed names"),
            col("names").alias("reversed names"),
        ])
        .collect()?;
    println!("{:?}", out);
    Ok(())
}
