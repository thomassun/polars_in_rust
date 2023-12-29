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
    // let expr = col("nrs").sort(Default::default()).head(Some(2));
    // let mask = df.column("nrs")?.i32()?.gt(2);
    let df_num = df
        .clone()
        .lazy()
        .select([col("nrs"), col("nrs").gt(2).alias("conditional")])
        .collect()?;
    println!("{:?}", df_num);
    // println!("{:?}", mask);
    Ok(())
}
