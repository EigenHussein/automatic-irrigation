use std::{ffi::CString, io::Read, net::TcpStream};

use esp_idf_svc::sys;

use crate::{
    esp_connection::send_struct,
    sha256hash::{Sha256Context, TLSHash},
};

#[derive(Debug)]
pub struct PartitionIO {
    initialized: bool,
    partition: *const esp_idf_svc::sys::esp_partition_t,
    tellg: u32,
}

impl PartitionIO {
    /// ## Description
    /// * get `*esp_partition_t` for the partition with given lable
    /// ## Parameters
    /// * `label`: A null-terminated label i.e. "ota_1\0"
    /// ## Output
    /// ```
    /// Result<*esp_partition_t, ()>
    /// ```
    pub fn get_partition_by_label(
        label: &'static str,
    ) -> Result<*const esp_idf_svc::sys::esp_partition_t, ()> {
        let partition_ = unsafe {
            esp_idf_svc::sys::esp_partition_find_first(
                esp_idf_svc::sys::esp_partition_type_t_ESP_PARTITION_TYPE_ANY as _,
                esp_idf_svc::sys::esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY as _,
                label.as_ptr() as _,
            )
        };
        if partition_ == 0 as _ {
            Err(())
        } else {
            Ok(partition_)
        }
    }

    /// ## Description
    /// * Returns a `PartitionIO` for the specific null-terminated label
    /// ## Parameters
    /// * `label`: A null-terminated label i.e. "ota_1\0"
    /// ## Output
    /// ```
    /// Result<PartitionIO, ()>
    /// ```
    pub fn open_partition_by_label(label: &str) -> Result<Self, ()> {
        let partition_ = unsafe {
            esp_idf_svc::sys::esp_partition_find_first(
                esp_idf_svc::sys::esp_partition_type_t_ESP_PARTITION_TYPE_ANY as _,
                esp_idf_svc::sys::esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY as _,
                label.as_ptr() as _,
            )
        };
        if partition_ == 0 as _ {
            Err(())
        } else {
            Ok(Self {
                initialized: true,
                partition: partition_,
                tellg: 0,
            })
        }
    }
    /// ## Description
    /// * Returns a `PartitionIO` for the specific null-terminated label
    /// ## Parameters
    /// * `label`: A null-terminated label i.e. "ota_1\0"
    /// ## Output
    /// ```
    /// Result<PartitionIO, ()>
    /// ```
    pub fn get_partition(&self) -> *const esp_idf_svc::sys::esp_partition_t {
        self.partition
    }
    /// ## Description
    /// * Returns a `String` of formatted output of the opened `esp_partition_t`
    pub fn print_partition_info(&self) -> String {
        unsafe { format!("{:?}", *self.partition) }
    }
    /// ## Description
    /// * Cleans `n_pages` pages after specific `offset`
    /// * Each page is `4096` bytes
    /// ## Notes
    /// * `offset` must be multiple of 4096
    pub fn clean_page(&self, offset: usize, n_pages: u32) -> Result<i32, ()> {
        unsafe {
            if (offset % 4096 != 0) {
                println!("clean offset isn't aligned!");
                return Err(());
            }
            let res = esp_idf_svc::sys::esp_partition_erase_range(
                self.partition,
                offset as _,
                4096 * n_pages as usize,
            );
            if res == 0 {
                Ok(res)
                // println!("Error erasing range!");
                // return;
            } else {
                println!("Error erasing range: {}", res);
                Err(())
                //println!("erasing range!");
            }
        }
    }
    /// ## Description
    /// * Write `data` to flash with specific address `offset`
    /// ## Parameters
    /// * `offset`: The offset at which `data` would be written
    /// * `data`: The data slice
    /// ## Notes
    /// * The page must be erased using `esp_partition_erase` or `clean_page` member function before writing
    pub fn write(&self, offset: usize, data: &[u8]) -> i32 {
        unsafe {
            esp_idf_svc::sys::esp_partition_write(
                self.partition,
                offset,
                data.as_ptr() as _,
                data.len(),
            )
        }
    }
    /// ## Description
    /// * Read `data` from flash with specific address `offset`
    /// ## Parameters
    /// * `offset`: The offset at which `data` would be read
    /// * `data`: Slice refernece to store data
    pub fn read(&self, offset: usize, data: &mut [u8]) -> i32 {
        unsafe {
            esp_idf_svc::sys::esp_partition_read(
                self.partition,
                offset,
                data.as_mut_ptr() as _,
                data.len(),
            )
        }
    }
}

//networking stuff
impl PartitionIO {
    //be careful, partition pages will be erased, starting from offset, and the erased range is multiple of 4096 bytes
    /// ## Description
    /// Receives `[length]` bytes and write them into flash (with `offset`)
    /// ## Parameters
    /// * `offset`: Flash offset
    /// * `length`: size of buffer
    /// ## Output
    /// Returns an operation Result <`length`,Err()>
    /// ## Notes
    /// Data is received as 128 bytes chunks
    /// Before using this method, the size of comming data must be know (Negotiaiton must be done)
    pub fn write_buf_from_socket(
        &self,
        offset: usize,
        length: usize,
        socket: &mut TcpStream,
    ) -> Result<u32, i32> {
        let mut buf = [0u8; 128];

        let mut remaining = length;
        let mut partition_ptr = offset;
        let mut packet_number = 0;
        while remaining > 0 {
            let packet_size = 128.min(remaining);
            let recv_res = socket.read_exact(&mut buf[0..packet_size]);
            if packet_number % 64 == 0 {
                println!("Remaining: {}", remaining);
            }

            if let Ok(recv_size) = recv_res {
                self.write(partition_ptr, &buf[0..packet_size]);
                partition_ptr += packet_size;
                remaining -= packet_size;
            } else {
                return Err(-3);
            }
            packet_number += 1;
        }
        return Ok(length as _);
    }

    /// ## Description
    /// Receives `[length]` bytes and write them into flash (with `offset`)
    /// It also validates the data using `SHA256` checksum
    /// ## Parameters
    /// * `offset`: Flash offset
    /// * `length`: Size of buffer
    ///
    /// ## Output
    /// Result of `SHA256` checksum
    /// ## Notes
    /// Data is received as 128 bytes chunks
    /// Before using this method, the size and `SHA256` checksum of comming data must be know (Negotiaiton must be done)
    pub fn write_hashed_buf_from_socket(
        &self,
        offset: usize,
        length: usize,
        socket: &mut TcpStream,
    ) -> Result<[u8; 32], i32> {
        let mut buf = [0u8; 128];

        let mut remaining = length;
        let mut partition_ptr = offset;

        //SHA256

        let mut sha256hash = Sha256Context::new().expect("Error creating SHA256CTX");

        while remaining > 0 {
            let recv_res = socket.read_exact(&mut buf[0..128.min(remaining)]);
            if let Ok(recv_size) = recv_res {
                // if recv_size == 0 {
                //     //Connection closed
                //     return Err(-2);
                // }
                self.write(partition_ptr, &buf[0..128.min(remaining)]);
                sha256hash.update(&buf[0..128.min(remaining)]);
                partition_ptr += 128;
                remaining -= 128;
            } else {
                //Connection ERROR
                return Err(-3);
            }
        }
        return Ok(sha256hash.finish().expect("Error finishing SHA256"));
    }
}
