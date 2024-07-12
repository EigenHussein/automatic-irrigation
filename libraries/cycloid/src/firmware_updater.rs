use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use esp_idf_svc::sys;
use serde::{Deserialize, Serialize};

use crate::{esp_connection::receive_struct, esp_partition::PartitionIO};

use super::{esp_partition, sha256hash};

const START_FLAG: u32 = 0x8f900e9d;

#[derive(Debug, Serialize, Deserialize)]
pub struct firmware_info {
    magic_number: u32,
    firmware_size: u32,
    firmware_version: (u16, u16), //major , minor
    firmware_sha256: [u8; 32],
}

impl Default for firmware_info {
    fn default() -> Self {
        firmware_info {
            magic_number: START_FLAG,
            firmware_size: 0,
            firmware_version: (0u16, 0u16),
            firmware_sha256: [0u8; 32],
        }
    }
}

/// ## Description
/// Firmware updater struct
pub struct FirmwareUpdater {
    info: firmware_info,
    partition: PartitionIO,
    socket_mutex: std::sync::Arc<std::sync::Mutex<TcpStream>>,
}

impl FirmwareUpdater {
    /// ## Description
    /// Create a new updater with a new `TcpStream`
    /// ## Parameters
    /// * `partition_label`: Null-terminated partition label i.e `"ota_1\0"`
    /// * `host`: host of updater i.e `"192.168.0.1:1610"`
    /// * `handshake`: An option for a function to start a communication before receiving `firmware_info` struct i.e
    /// ```
    /// let handshake = |socket : &mut TcpStream| {
    ///     socket.write_all("please send firmawre_info!".as_bytes());
    /// }
    /// ```
    pub fn new_from_host<F>(
        partition_label: &str,
        host: &str,
        handshake: Option<F>,
    ) -> Result<Self, i32>
    where
        F: Fn(&mut TcpStream) -> (),
    {
        //try to connect
        let mut potential_socket = TcpStream::connect(host);
        //if connected successfully
        if let Ok(mut valid_socket) = potential_socket {
            let sock_mutex = Arc::new(std::sync::Mutex::new(valid_socket));
            return Self::new_from_socket(partition_label, &sock_mutex, handshake);
        }
        Err(-1)
    }
    /// ## Description
    /// Create a new updater with already exist `TcpStream`
    /// ## Parameters
    /// * `partition_label`: Null-terminated partition label i.e `"ota_1\0"`
    /// * `stream`: A mutex for an already existing `TcpStream`
    /// * `handshake`: An option for a function to start a communication before receiving `firmware_info` struct
    /// * i.e
    /// ```
    /// let handshake = |socket : &mut TcpStream| {
    ///     socket.write_all("please send firmawre_info!".as_bytes());
    /// }
    /// ```
    pub fn new_from_socket<F>(
        partition_label: &str,
        stream: &Arc<std::sync::Mutex<TcpStream>>,
        handshake: Option<F>,
    ) -> Result<Self, i32>
    where
        F: Fn(&mut TcpStream) -> (),
    {
        //if there is an initialization required before receiving header (firmware_info struct)
        let sock_arc_mutex = stream.clone();
        let mut sock_mutex = sock_arc_mutex.try_lock().unwrap();
        if let Some(valid_function) = handshake {
            valid_function(&mut sock_mutex);
        }

        //receive firmware frame
        println!("Receiving firmware information!");
        let mut firm_info = receive_struct::<firmware_info>(&mut sock_mutex).unwrap();

        println!("{:?}", firm_info);

        {
            if let Ok(opened_partition) = PartitionIO::open_partition_by_label(partition_label) {
                let mut tmp = Self {
                    info: firm_info,
                    partition: opened_partition,
                    socket_mutex: stream.clone(),
                };
                //tmp.partition.clean_page(0, )
                return Ok(tmp);
            }
        }

        Err(-2)
        /* {
            Ok(Self {
                info: firm_info,
                partition:
                    .expect("Error opening label"),
            })
        } */
    }

    /// ## Description
    /// Start updating by sending magic number (4 bytes)
    /// The TcpStream mutex is locked then `START_FLAG` will be sent indicating (`READY_TO_RECEIVE_FIRMWARE`)
    /// ## Output
    /// Returns an operation Result <`length`,Err()>
    pub fn start_updating(&mut self) -> Result<u32, i32> {
        let mut updater_socket = self.socket_mutex.try_lock().unwrap();

        // Clean page
        let pages = (self.info.firmware_size as f32) / 4096f32;
        println!("Erasing Flash: {}", f32::ceil(pages) as u32);

        //if the pages cannot be cleaned
        if let Err(_) = self.partition.clean_page(0, f32::ceil(pages) as _) {
            println!("Error cleaning pages");
            return Err(-1);
        }
        
        println!("Done erasing flash");
        // Start downloading to flash
        updater_socket
            .write_all(&START_FLAG.to_le_bytes())
            .expect("Error sending start flag!");
        self.partition
            .write_buf_from_socket(0, self.info.firmware_size as _, &mut updater_socket)
    }
    /// ## Description
    /// Set current opened partition as boot partition
    fn set_boot_partition(&self) -> esp_idf_svc::sys::esp_err_t {
        return unsafe {
            esp_idf_svc::sys::esp_ota_set_boot_partition(self.partition.get_partition())
        };
    }

    /// ## Description
    /// Start updating by sending magic number (4 bytes).
    /// The TcpStream mutex is locked then `START_FLAG` will be sent indicating (`READY_TO_RECEIVE_FIRMWARE`).
    /// then try to swap boot partition after writing the firmware to flash and verify integrity using `SHA256` checksum
    /// ## Output
    /// Returns an operation Result <`length`,Err()>
    pub fn start_updating_with_validation(&mut self) -> Result<u32, i32> {
        let mut updater_socket = self.socket_mutex.try_lock().unwrap();

        let pages = (self.info.firmware_size as f32) / 4096f32;
        //if the pages cannot be cleaned
        if let Err(_) = self.partition.clean_page(0, f32::ceil(pages) as _) {
            return Err(-1);
        }

        updater_socket
            .write_all(&START_FLAG.to_le_bytes())
            .expect("Error sending start flag!");
        // self.partition
        //     .write_buf_from_socket(0, self.info.firmware_size as _, &mut updater_socket)
        if let Ok(res) = self.partition.write_hashed_buf_from_socket(
            0,
            self.info.firmware_size as _,
            &mut updater_socket,
        ) {
            for i in 0..32 {
                if res[i] != self.info.firmware_sha256[i] {
                    println!("Invalid firmware hash... not accepted!");
                    return Err(-4);
                }
            }
            println!(
                "Setting new bootable partition: {}",
                self.set_boot_partition()
            );
            Ok(0)
        } else {
            return Err(-5);
        }
    }
}
