
use std::{
    ffi::c_void,
    io::{Read, Write},
};

use cycloid::sensors;
use esp_idf_svc::{
    hal::{delay, peripheral::Peripheral, peripherals::Peripherals},
    nvs::{EspCustomNvsPartition, EspNvsPartition},
    sys::{
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_type_t_ESP_PARTITION_TYPE_ANY, esp_pm_config_esp32_t,
    },
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let periph = Peripherals::take().unwrap();

    let mut gpio = esp_idf_svc::hal::gpio::PinDriver::output(periph.pins.gpio16).unwrap();
    for i in 0..100 {
        gpio.toggle();
        delay::FreeRtos::delay_ms(50);
    }

    let mut wifi = cycloid::wifi_module::WiFiModule::new_access_point(
        periph.modem,
        (&"Hussein", "Hussein2K21"),
    );

    let mut spi_ffs_c =
        irrigation_core::logging::spiffs::FFSCore::open("spifs", "/my_partition", 3, false);
    if let Ok(mut spi_ffs) = spi_ffs_c {
        {
            let mut my_file = spi_ffs.open_file("Hello.txt").unwrap();
            println!("{:?}", my_file.write("Hello World!".as_bytes()));
        }
        {
            let mut my_file = spi_ffs.open_file("Hello.txt").unwrap();
            let mut data = Vec::with_capacity(100);
            println!("{:?}", my_file.read_to_end(&mut data));
            println!("{:?}", std::str::from_utf8(&data.as_slice()[0..data.len()]));
            
        }
    }

    loop {
        delay::FreeRtos::delay_ms(1000);
    }
}

fn main2() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("Hello from factory!");
    println!("Setting ota 0 as main!");
    unsafe {
        println!(
            "Setting result: {}",
            esp_idf_svc::sys::esp_ota_set_boot_partition(
                esp_idf_svc::sys::esp_partition_find_first(
                    esp_partition_type_t_ESP_PARTITION_TYPE_ANY as _,
                    esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY as _,
                    "ota_0\0".as_ptr() as _,
                )
            )
        );
    };

    log::info!("Hello, world!");
}
