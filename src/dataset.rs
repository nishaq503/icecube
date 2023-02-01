use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DataSet {
    train_batches: Vec<PathBuf>,
    test_batches: Vec<PathBuf>,
    train_meta: PathBuf,
    sensor_geometry: PathBuf,
    sample_submission: PathBuf,
}

impl std::fmt::Display for DataSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let train_batches = self.train_batches.len();
        writeln!(f, "\n    train_batches: {train_batches}").map_err(|reason| panic!("{reason}"))?;

        let test_batches = self.test_batches.len();
        writeln!(f, "    test_batches: {test_batches}").map_err(|reason| panic!("{reason}"))?;

        let train_meta = get_file_name(&self.train_meta).map_err(|reason| panic!("{reason}"))?;
        writeln!(f, "    train_meta: {train_meta}").map_err(|reason| panic!("{reason}"))?;

        let sensor_geometry =
            get_file_name(&self.sensor_geometry).map_err(|reason| panic!("{reason}"))?;
        writeln!(f, "    sensor_geometry: {sensor_geometry}")
            .map_err(|reason| panic!("{reason}"))?;

        let sample_submission =
            get_file_name(&self.sample_submission).map_err(|reason| panic!("{reason}"))?;
        write!(f, "    sample_submission: {sample_submission}")
            .map_err(|reason| panic!("{reason}"))?;

        Ok(())
    }
}

impl DataSet {
    pub fn new(root: Option<PathBuf>) -> std::io::Result<Self> {
        let root = {
            let mut root = root.unwrap_or(std::env::current_dir()?);
            root.pop();

            root.push("input");
            assert!(root.exists(), "Path not found: {root:?}");

            root.push("icecube-neutrinos-in-deep-ice");
            assert!(root.exists(), "Path not found: {root:?}");

            root
        };

        let train = {
            let mut train = root.clone();
            train.push("train");
            assert!(train.exists(), "Path not found: {train:?}");
            train
        };
        let train_batches = get_batch_paths(&train)?;

        let test = {
            let mut test = root.clone();
            test.push("test");
            assert!(test.exists(), "Path not found: {test:?}");
            test
        };
        let test_batches = get_batch_paths(&test)?;

        let train_meta = {
            let mut train_meta = root.clone();
            train_meta.push("train_meta.parquet");
            assert!(train_meta.exists(), "File not found: {train_meta:?}");
            train_meta
        };

        let sensor_geometry = {
            let mut sensor_geometry = root.clone();
            sensor_geometry.push("sensor_geometry.csv");
            assert!(
                sensor_geometry.exists(),
                "File not found: {sensor_geometry:?}"
            );
            sensor_geometry
        };

        let sample_submission = {
            let mut sample_submission = root;
            sample_submission.push("sample_submission.parquet");
            assert!(
                sample_submission.exists(),
                "File not found: {sample_submission:?}"
            );
            sample_submission
        };

        Ok(Self {
            train_batches,
            test_batches,
            train_meta,
            sensor_geometry,
            sample_submission,
        })
    }
}

fn get_batch_paths(batch_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut train_batches = batch_dir
        .read_dir()?
        .map(|v| {
            v.map(|v| {
                let path = v.path();
                let i_str = *path
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .split("batch_")
                    .last()
                    .unwrap()
                    .split('.')
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap();
                let i: usize = i_str.parse().unwrap();
                (i, path)
            })
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    train_batches.sort();
    Ok(train_batches.drain(..).map(|(_, path)| path).collect())
}

fn get_file_name(path: &Path) -> Result<&str, String> {
    if let Some(Some(name)) = path.file_name().map(|v| v.to_str()) {
        Ok(name)
    } else {
        panic!("Could not get file name")
    }
}
