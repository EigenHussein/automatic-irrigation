use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use cycloid::{esp_connection, firmware_updater::FirmwareUpdater};
use esp_idf_svc::{
    hal::{
        delay::{Delay, FreeRtos},
        peripherals::Peripherals,
    },
    sys::{
        esp_freertos_tick_cb_t, esp_partition_find_first,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_APP_OTA_0,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_DATA_OTA,
        esp_partition_type_t_ESP_PARTITION_TYPE_ANY, esp_partition_type_t_ESP_PARTITION_TYPE_APP,
        esp_restart,
    },
};

const WIFI_TIMER: u32 = 1 * 60;

fn set_bootable_partition(label: *const i8) -> Result<(), ()> {
    unsafe {
        let partition = esp_partition_find_first(
            esp_partition_type_t_ESP_PARTITION_TYPE_ANY,
            esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
            label,
        );
        if partition == 0 as _ {
            println!("Couldn't find partition!");
            return Err(());
        }
        if esp_idf_svc::sys::esp_ota_set_boot_partition(partition) != 0 {
            println!("Couldn't set the new partition!");
            return Err(());
        }
        println!("Done\nRebooting...");
        esp_restart();
    };
}

fn update_firmware(connection: TcpStream) {
    

    //set blocking property of the TcpStream
    connection
        .set_nonblocking(false)
        .expect("Expect blocking connection");

    // Wrap the TcpStream in an Arc<Mutex<>>
    let connection_mutex = Arc::new(Mutex::new(connection));

    //Initialize the updater
    let mut updater = FirmwareUpdater::new_from_socket(
        "ota_0\0",
        &connection_mutex,
        Some(|socket: &mut TcpStream| {
            let val = esp_connection::receive_struct::<u32>(socket);
            if let Ok(valid_val) = val {
                if valid_val == 0x8f900e9d {
                    println!("Received a magic number");
                    FreeRtos::delay_ms(200);
                }
            }
            FreeRtos::delay_ms(100);

            // socket.write_all("please send firmawre_info!".as_bytes());
        }),
    )
    .unwrap();

    //Start downloadig
    let updating_result = updater.start_updating();
    if let Ok(_good_res) = updating_result {
        println!("Done uploading the firmware to OTA0\nSetting the new boot partition...\n");
        set_bootable_partition("ota_0\0".as_ptr() as _);
    } else if let Err(bad_result) = updating_result {
        println!("Failed to update: {}", bad_result);
    }
}

/// ### Description
/// Create an access point and wait for update connection for 2 minutes:
/// * If there is a connection the new firmware is written to the OTA0 partition.
/// * If not, the partition will be swapped back to the old OTA0
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let periph = Peripherals::take().unwrap();

    // Start a new access-point
    let wifi = cycloid::wifi_module::WiFiModule::new_access_point(
        periph.modem,
        ("CNC", "OpeOpenoMi"),
    )
    .unwrap();

    // Start update server and wait for connection
    let mut update_server = TcpListener::bind("0.0.0.0:1610").unwrap();
    // update_server.set_nonblocking(true).unwrap();

    // Wait for connection, on timeout, the partitions are swapped and esp32 is restarted
    let mut timer1 = 0;
    for client in update_server.incoming() {
        if let Ok(valid_client) = client {
            println!("Got it");
            update_firmware(valid_client);
        } else if let Err(err) = client {
            println!("{err:?}");

            FreeRtos::delay_ms(1000);

            timer1 += 1;
            if timer1 > WIFI_TIMER {
                println!("{}", "Timeout");
                set_bootable_partition("ota_0\0".as_ptr() as _);
            }
            println!("timer: {} out of {}\n", timer1, WIFI_TIMER);
        }
    }
    print!("DEAD");
    // let is_connected = Arc::new(Mutex::new(false));
    // let handler = thread::spawn(|| {});
}
