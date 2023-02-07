use polars::prelude::*;

use icecube::*;

// #[derive(Debug)]
// struct Measurement {
//     sensor_id: usize,
//     time: usize,
//     charge: f64,
// }

#[derive(Debug)]
#[allow(dead_code)]
struct Event {
    // measurements: Vec<Measurement>,
    sensor_id: usize,
    time: usize,
    charge: f64,
    auxiliary: bool,
    event_id: usize,
}

impl Event {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        let (_, s) = s.split_at(1);
        let (s, _) = s.split_at(s.len() - 1);

        let mut items = s.split(", ");

        let sensor_id = items.next().unwrap().split(": ").last().unwrap().parse().unwrap();
        let time = items.next().unwrap().split(": ").last().unwrap().parse().unwrap();
        let charge = items.next().unwrap().split(": ").last().unwrap().parse().unwrap();
        let auxiliary = items.next().unwrap().split(": ").last().unwrap().parse().unwrap();
        let event_id = items.next().unwrap().split(": ").last().unwrap().parse().unwrap();

        Self {
            sensor_id,
            time,
            charge,
            auxiliary,
            event_id,
        }
    }
}

fn main() -> std::io::Result<()> {
    let data = dataset::DataSet::new(None)?;
    println!("Parsed Dataset: {data}");

    let path = &data.train_batch_paths()[0];
    println!("Using path {path:?}");

    let df = LazyFrame::scan_parquet(path, Default::default()).unwrap();
    let events = df.groupby([col("event_id")]).agg([
        col("charge").mean().alias("charge_mean"),
        col("time").min().alias("time_min"),
        col("time").max().alias("time_max"),
        col("sensor_id").count().alias("sensor_id_count"),
        col("auxiliary").first().alias("auxiliary"),
    ]).collect().unwrap();
    println!("{events:?}");

    Ok(())
}
