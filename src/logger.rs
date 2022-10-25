use chrono::prelude::{DateTime, Datelike, Timelike, Utc};
use std::{fs::File, path::{Path, PathBuf}, io::Write};


pub struct Logger {
    path: PathBuf,  // logging location
    file_suffix: String,  // Common part of measurement file names
    file: Option<File>,  // Current measurement file
}

impl Logger {
    pub fn new(path: &Path, suffix: &str) -> Logger
    {
        let mut logger = Logger {
          path: path.to_path_buf(),
          file_suffix: suffix.to_owned(),
          file: None};

        let now = Utc::now();
        logger.open_file(&now);

        logger
    }

    /// Check if the current directory for the measurement data already exists.
    /// If not: create it.
    fn check_dir(&self)
    {
        // Check if path is there -> panic on error
        let exists = self.path.exists();
        // Create when necessary
        if !exists {
            std::fs::create_dir(&self.path).unwrap();
        }
    }

    pub fn log(&mut self, data: &str) {
        // Get current time and check if new file is needed
        let now = Utc::now();

        if self.file.is_some() {
            match self.file.as_ref().unwrap()
              .write_all(format!("{};{}", now, data).as_bytes())
            {
                Ok(_) => println!("Weight logged"),
                Err(e) => println!("Error logging data: {:?}", e)
            };
        } else {
            println!("Error logging data (No file opened, trying to open one)");
            self.open_file(&now);
        }
    }

    fn open_file(&mut self, now: &DateTime<Utc>) {
        let year = now.year();
        let month = now.month();
        let month_str = format!("{:0>2}", month);
        let day = format!("{:0>2}", now.day());
        let hour = format!("{:0>2}", now.hour());
        let minute = format!("{:0>2}", now.minute());
        let second = format!("{:0>2}", now.second());

        let f_name = format!("{}{}{}_{}{}{}_{}.txt",
                             year, month_str, day,
                             hour, minute, second, self.file_suffix);

        // Build path
        self.check_dir();

        self.file = match File::create(&self.path
                                         .join(&f_name))
        {
            Ok(f) => {
                Some(f)
            },
            Err(e) => {
                println!("Error creating measurement file {} at {:?} ({:?})",
                         &f_name, &self.path, e);
                None
            }
        };
    }
}
