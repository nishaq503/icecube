use std::fs::File;

use parquet::file::reader::{FileReader, SerializedFileReader};

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
    let dataset = dataset::DataSet::new(None)?;
    println!("Parsed Dataset: {dataset}");

    let file = File::open(&dataset.train_batch_paths()[0]).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    // let metadata = reader.metadata();
    let row_group_reader = reader.get_row_group(0).unwrap();

    let events = row_group_reader.get_row_iter(None).unwrap()
        .map(|row| {
            let mut row_str = format!("{row}");
            row_str.pop();
            Event::from_str(&row_str[1..])
        })
        .collect::<Vec<_>>();

    println!("Got {} events.", events.len());

    Ok(())
}
