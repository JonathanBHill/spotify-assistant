use std::fs::File;

use polars::frame::column::ScalarColumn;
use polars::prelude::*;
use tracing::Level;

use spotify_assistant_core::utilities::general::pair_vector;

#[derive(Debug, Clone)]
pub struct DataFrameLoader {
    pub df: DataFrame,
}

impl DataFrameLoader {
    pub fn from_json(file_path: String) -> Result<Self, PolarsError> {
        let file = DataFrameLoader::from_file(file_path);
        let df = JsonReader::new(file).infer_schema_len(None).finish()?;
        Ok(DataFrameLoader { df })
    }
    pub fn from_parquet(file_path: String) -> Result<Self, PolarsError> {
        let file = DataFrameLoader::from_file(file_path);
        let df = ParquetReader::new(file).finish()?;
        Ok(DataFrameLoader { df })
    }
    pub fn from_dataframe(df: DataFrame) -> Self {
        DataFrameLoader { df }
    }
    fn from_file(file_path: String) -> File {
        match File::open(&file_path).map_err(|err| {
            PolarsError::ComputeError(format!("Failed to open file: {}", err).into())
        }) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    pub fn synchronize_dataframes(mut loaders: Vec<Self>) -> Result<Vec<Self>, PolarsError> {
        let pairs = pair_vector(loaders.len());
        let nulls = loaders
            .iter()
            .map(|loader| loader.null_dtype())
            .collect::<Vec<bool>>();
        for (first_index, second_index) in pairs {
            if nulls.get(first_index).unwrap() | nulls.get(second_index).unwrap() {
                let mut second = loaders.remove(second_index).clone();
                let mut first = loaders.remove(first_index).clone();
                first.correct_schema_nulls(&mut second)?;
                loaders.insert(first_index, first);
                loaders.insert(second_index, second);
            }
        }
        Ok(loaders)
    }
    pub fn null_dtype(&self) -> bool {
        self.df
            .schema()
            .iter_values()
            .any(|col| col == &DataType::Null)
    }
    fn get_snapshot(&self, other: Option<DataFrame>) -> Vec<(String, DataType)> {
        if let Some(other) = other {
            return other
                .get_columns()
                .iter()
                .map(|s| (s.name().to_string(), s.dtype().clone()))
                .collect();
        }
        self.df
            .get_columns()
            .iter()
            .map(|s| (s.name().to_string(), s.dtype().clone()))
            .collect()
    }
    fn is_equal_width(
        &self,
        self_cols: Vec<(String, DataType)>,
        other_cols: Vec<(String, DataType)>,
    ) -> bool {
        if self_cols.len() != other_cols.len() {
            tracing::debug!(
                "Column count mismatch: self={} other={}",
                self_cols.len(),
                other_cols.len()
            );
            false
        } else {
            true
        }
    }
    pub fn correct_schema_nulls(
        &mut self,
        other: &mut DataFrameLoader,
    ) -> Result<bool, PolarsError> {
        let span = tracing::span!(Level::DEBUG, "DataFrameLoader.correct_schema_nulls");
        let _enter = span.enter();

        // 1) Snapshot the current (name, dtype) pairs so we don't hold borrows
        let self_cols: Vec<(String, DataType)> = self.get_snapshot(None);
        let other_cols: Vec<(String, DataType)> = other.get_snapshot(Some(other.df.clone()));

        // Optional: early check for different widths
        let _ = self.is_equal_width(self_cols.clone(), other_cols.clone());

        // 2) Iterate over the snapshots; it’s now safe to mutate self/other inside the loop
        for ((self_name, self_dtype), (other_name, other_dtype)) in
            self_cols.iter().zip(other_cols.iter())
        {
            if self_name != other_name {
                tracing::debug!("Column names do not match: {} != {}", self_name, other_name);
                continue;
            }

            if self_dtype != other_dtype {
                if *self_dtype == DataType::Null && *other_dtype != DataType::Null {
                    tracing::debug!("Own {} is null. Must cast to {}", self_name, other_dtype);
                    self.cast_column(self_name, other_dtype.clone())?;
                } else if *other_dtype == DataType::Null && *self_dtype != DataType::Null {
                    tracing::debug!("Other {} is null. Must cast to {}", other_name, self_dtype);
                    other.cast_column(other_name, self_dtype.clone())?;
                } else if *self_dtype != DataType::Null && *other_dtype != DataType::Null {
                    tracing::debug!(
                        "Datatypes for {} do not match: {} != {}",
                        self_name,
                        self_dtype,
                        other_dtype
                    );
                } else {
                    tracing::debug!("Both column datatypes are null for {}", self_name);
                }
            } else {
                tracing::debug!("Datatypes for {} match ({})", self_name, self_dtype);
            }
        }
        Ok(self.df.schema().eq(&other.df.schema()))
    }
    pub fn cast_column(&mut self, alias: &str, dtype: DataType) -> Result<(), PolarsError> {
        // Validate the schema
        self.df = self
            .df
            .clone()
            .lazy()
            .with_column(lit(NULL).cast(dtype).alias(alias))
            .collect()?;
        Ok(())
    }
    pub fn fix_timestamp_column(&mut self) -> Result<DataFrame, PolarsError> {
        let mut df = self
            .df
            .clone()
            .lazy()
            .with_column(
                col("ts")
                    .str()
                    .strptime(
                        DataType::Datetime(TimeUnit::Milliseconds, None),
                        StrptimeOptions {
                            format: Some("%Y-%m-%dT%H:%M:%SZ".into()),
                            exact: true,
                            strict: true,
                            cache: true,
                        },
                        Expr::Column("ts".into()),
                    )
                    .alias("datetime"),
            )
            .with_column(col("datetime").dt().year().alias("year"))
            .with_column(col("datetime").dt().quarter().alias("quarter"))
            .with_column(col("datetime").dt().month().alias("month"))
            .with_column(col("datetime").dt().day().alias("day"))
            .with_column(col("datetime").dt().hour().alias("hour"))
            .with_column(col("datetime").dt().minute().alias("minute"))
            .with_column(col("datetime").dt().second().alias("second"))
            .collect()?;
        let cols_to_drop: Vec<PlSmallStr> = vec![
            "ts".into(),
            "datetime".into(),
            "username".into(),
            "conn_country".into(),
            "episode_name".into(),
            "episode_show_name".into(),
            "spotify_episode_uri".into(),
        ];
        df = df.drop_many(&mut cols_to_drop.into_iter());
        println!("{:?}", df);
        self.df = df.clone();
        Ok(df)
    }
    pub fn group_by(&self, columns: Vec<&str>) -> Result<DataFrame, PolarsError> {
        let df = self
            .df
            .clone()
            .lazy()
            .group_by_stable(columns)
            .agg(vec![
                col("master_metadata_track_name")
                    .n_unique()
                    .alias("unique_tracks played"),
                col("master_metadata_track_name")
                    .count()
                    .alias("tracks_played"),
                col("master_metadata_track_name")
                    .unique_stable()
                    .alias("track_name"),
                col("spotify_track_uri").unique_stable().alias("track_uri"),
            ])
            .collect()?;
        Ok(df)
    }

    pub fn get_year(&self, target: i32) -> Result<DataFrame, PolarsError> {
        let year = self
            .df
            .column("year")
            .expect("Failed to get the year column");
        let scalar = ScalarColumn::new(
            PlSmallStr::from("year_filter"),
            Scalar::from(target),
            self.df.height(),
        )
            .into_column();
        let mask = year.equal(&scalar)?;
        let filtered_df = self.df.filter(&mask)?;
        println!(
            "Self.df length: {:?} | Filtered df length: {:?}",
            self.df.shape(),
            filtered_df.shape()
        );
        Ok(filtered_df)
    }
    pub fn print_df(&self) -> Result<(), PolarsError> {
        println!("{}", self.df);
        println!("{:?}", self.df.schema());
        println!("Column names:\n{:?}", self.df.get_column_names_str());
        Ok(())
    }
}
