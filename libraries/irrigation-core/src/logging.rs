pub mod spiffs {
    use std::ffi::CString;

    use esp_idf_svc::sys::{esp_err_t, esp_vfs_spiffs_conf_t, esp_vfs_spiffs_unregister};

    pub struct FFSCore {
        partition_label: String,
        mount_point: String,
    }

    impl FFSCore {
        /// ## Description
        /// open an SPI-FFS partition given its `partition_label` and base path (`path`)
        /// ## Parameters
        /// * `partition_label`: The label of `spiffs` partition
        /// * `path`: Mounting path of ffs partition
        /// * `max_files`: The maximum number of files that could be opened (capped to 10)
        ///
        /// ## Outputs
        /// * On success mounting returns a `FFSCore` struct
        /// * On fail returns `esp_error_t`
        ///
        /// ## Example
        /// ```
        ///let spi_ffs = FFSCore::open(
        ///    &"spi_fft",
        ///    &"/my_partition",
        ///    3,
        ///    true
        ///)
        ///.unwrap();
        /// ```
        pub fn open(
            partition_label: &str,
            path: &str,
            max_files: usize,
            format_if_mount_failed: bool,
        ) -> Result<Self, esp_err_t> {
            let label = CString::new(partition_label).unwrap();
            let base_path = CString::new(path).unwrap();

            let esp_spiffs_conf = esp_vfs_spiffs_conf_t {
                partition_label: label.as_ptr(),
                base_path: base_path.as_ptr(),
                max_files: max_files.min(10),
                format_if_mount_failed: format_if_mount_failed,
            };
            let res = unsafe { esp_idf_svc::sys::esp_vfs_spiffs_register(&esp_spiffs_conf) };

            if res == 0 {
                return Ok(Self {
                    partition_label: partition_label.to_string(),
                    mount_point: path.to_string(),
                });
            }
            return Err(res);
        }

        /// ## Description
        /// open a file exists on root folder in read-write mode
        /// ## Example
        /// If the partition is mounted on `/my_partition` you can read `/my_partition/my_file.txt` by:
        ///```
        /// let spi_ffs = FFSCore::open(
        ///    &"spi_fft",
        ///    &"/my_partition",
        ///    3,
        ///    true
        ///)
        ///.unwrap();
        /// let mut file = spi_ffs.open_file(&"my_files.txt").unwrap();
        /// ```
        pub fn open_file(&mut self, filename: &str) -> Result<std::fs::File, std::io::Error> {
            let full_filename = self.mount_point.clone() + "/" + filename;
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&full_filename)
            {
                Ok(file) => return Ok(file),
                Err(_) => {
                    // If the file doesn't exist, create it
                    std::fs::File::create(&full_filename).unwrap();
                    // Reopen the file in read-write mode and return the result
                    return std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(&full_filename);
                }
            }
        }
    }

    impl Drop for FFSCore {
        fn drop(&mut self) {
            unsafe {
                esp_vfs_spiffs_unregister(
                    CString::new(self.partition_label.clone()).unwrap().as_ptr(),
                )
            };
        }
    }
}

pub mod logger {
    use std::{
        fs::File,
        io::{Read, Seek, Write},
        sync::{Arc, Mutex},
    };

    use crate::webview::WebModule;

    pub struct IrrigaitonLogger {
        mFile: Arc<Mutex<File>>,
    }
    impl IrrigaitonLogger {
        /// ## Description
        /// * Create a new log file (delete existing log file)
        /// ## Parameters
        /// * `filename`: The file to be opened or created
        pub fn new_logger(filename: &str) -> Result<Self, std::io::Error> {
            if std::fs::File::open(filename).is_ok() {
                std::fs::remove_file(filename).expect("Couldn't delete file");
            }
            return Self::open_logger(filename);
        }

        /// ## Description
        /// * Opens a logging file, if not exist, a file will be created!
        /// ## Parameters
        /// * `filename`: The file to be opened or created
        pub fn open_logger(filename: &str) -> Result<Self, std::io::Error> {
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(filename)
            {
                Ok(mut file) => {
                    println!(
                        "seeked {:?}",
                        file.seek(std::io::SeekFrom::End(0)).expect("Couldn't seek")
                    );
                    file.seek(std::io::SeekFrom::End(0)).expect("Couldn't seek");
                    file.write("[new_log]\n".as_bytes()).unwrap();
                    return Ok(Self {
                        mFile: Arc::new(Mutex::new(file)),
                    });
                }
                Err(_) => {
                    // If the file doesn't exist, create it
                    std::fs::File::create(&filename).unwrap();

                    // Reopen the file in read-write mode and return the result
                    let mut file = std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(&filename)
                        .unwrap();

                    file.seek(std::io::SeekFrom::End(0)).expect("Couldn't seek");
                    file.write("[new_log]\n".as_bytes());
                    return Ok(Self {
                        mFile: Arc::new(Mutex::new(file)),
                    });
                }
            }
        }

        /// ## Description
        /// * Write a line to the log file
        /// * The format is `[Date:Time]: $s \n`
        pub fn log_string(&mut self, s: &str) {
            self.mFile
                .lock()
                .unwrap()
                .write(
                    format!(
                        "[{:?}]: {}\n",
                        chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                        s
                    )
                    .as_bytes(),
                )
                .expect("Couldn't write log");
        }
        pub fn get_logfile_instance(&self) -> Arc<Mutex<File>> {
            return self.mFile.clone();
        }
    }

    impl WebModule for IrrigaitonLogger {
        /// ## Description
        /// Adds a hundler for `/get_log` http request which will send the log file
        fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {
            let file_mutex = self.mFile.clone();
            http_server
                .fn_handler("/get_log", esp_idf_svc::http::Method::Get, move |req| {
                    let mut file = file_mutex.lock().unwrap();
                    let file_size = file.seek(std::io::SeekFrom::Current(0)).unwrap();
                    file.seek(std::io::SeekFrom::Start(0)).unwrap();

                    let mut res = req
                        .into_response(200, None, &[("Content-Type", "text/plain")])
                        .unwrap();
                    let mut buf = [0u8; 1024];
                    for i in 0..(file_size) / 1024 {
                        file.read_exact(&mut buf);
                        res.write(&buf);
                    }
                    file.read_exact(&mut buf[0..(file_size % 1024) as usize]);
                    res.write(&buf[0..(file_size % 1024) as usize]);
                    Ok(())
                })
                .unwrap();
        }
    }
}
