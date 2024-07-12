use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct SIConfiguration {
    configuration_verison: u32,
    firmware_version: (u16, u16),
}

pub struct ConfigurationIO {
    m_file: File,
}

impl ConfigurationIO {
    /// ## Description
    /// Opens a configuration file, if not exist.. create one,
    /// ## Parameters
    /// * `filename`: filename of the configuration file
    pub fn new_from_filename(filename: &str) -> Result<Self, std::io::Error> {
        let tmp = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(filename);
        if let Ok(file) = tmp {
            return Ok(Self { m_file: file });
        } else {
            return Err(tmp.err().unwrap());
        }
    }

    /// ## Description
    /// Serialize `SIConfiguration` and write it content as json to the currently opened file
    /// ## Parameters
    /// * conf: `SIConfiguration` structure
    pub fn write_config(&mut self, conf: &SIConfiguration) -> &mut Self {
        self.m_file
            .write_all(
                serde_json::to_string(&conf)
                    .expect("Couldn't serialize configuration")
                    .as_bytes(),
            )
            .expect("Error writing the configuration file");
        return self;
    }
    /// ## Description
    /// Read the content of the configuraion file and Deserialize it into `SIConfiguration`
    pub fn read_config(&mut self) -> Result<SIConfiguration, i32> {
        let mut conf_string = String::new();
        if let Ok(_size) = self.m_file.read_to_string(&mut conf_string) {
            if let Ok(deserialized_struct) = serde_json::from_str::<SIConfiguration>(&conf_string) {
                return Ok(deserialized_struct);
            }
        }
        return Err(-1);
    }

}
