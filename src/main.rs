use std::{
    ffi::CString,
    fs::File,
    io::{Read, Seek, Write},
    marker::PhantomData,
    os::raw::c_void,
};

use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use cycloid::wifi_module::{self, WiFiModule};
use esp_idf_svc::{
    hal::{delay, peripherals::Peripherals},
    io::Write as _,
    nvs::{EspCustomNvsPartition, EspNvsPartition},
    sys::{
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_type_t_ESP_PARTITION_TYPE_ANY, esp_pm_config_esp32_t, esp_pm_config_t,
    },
};
use irrigation_core::{
    logging::{logger, spiffs::FFSCore},
    main_page::MainPage,
    sensors::atm_sensor::ATMSensorDriver,
    webview::WebModule,
};

struct FieldStatus {
    timestamp: i64,
    temperature: f32,
    humidity_air: f32,
    humidity_soil: f32,
    luminosity: f32,
}

fn criteria_example(stat: &FieldStatus) -> u32 {
    10
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let (sunrise, sunset) = sunrise::sunrise_sunset(31.0510, 46.2569, 2024, 2, 7);

    let baghdad_time = DateTime::from_timestamp(sunrise, 0)
        .unwrap()
        .checked_add_signed(Duration::hours(3));

    println!("{:?}", baghdad_time);
    let periph = Peripherals::take().unwrap();

    let wifi =
        wifi_module::WiFiModule::new_access_point(periph.modem, ("Hussein", "OpeOpenoMi")).unwrap();

    loop {
        println!("Hehe boiiii");
        delay::FreeRtos::delay_ms(1000);
    }
}

fn main2() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let periph = Peripherals::take().unwrap();
    let atm_module = ATMSensorDriver::init(21, 22, 400000);
    let spi_ffs = FFSCore::open(&"spifs", &"/my_partition", 3, true).unwrap();
    let mut logger_module =
        logger::IrrigaitonLogger::new_logger("/my_partition/main_log.txt").unwrap();

    let mut wifi = WiFiModule::new_client(periph.modem, ("Curl", "@OpenGL64@")).unwrap();
    let mut web_interface = irrigation_core::webview::WebView::new().unwrap();

    web_interface.add_module(MainPage);
    web_interface.add_module(atm_module);
    web_interface.add_module(logger_module);

    loop {
        delay::FreeRtos::delay_ms(100);
    }
}

// fn main2() {
//     // It is necessary to call this function once. Otherwise some patches to the runtime
//     // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
//     esp_idf_svc::sys::link_patches();
//     // Bind the log crate to the ESP Logging facilities
//     esp_idf_svc::log::EspLogger::initialize_default();
//     let d1 = chrono::prelude::Utc.with_ymd_and_hms(2023, 12, 16, 18, 50, 20);
//     let d2 = chrono::prelude::Utc.with_ymd_and_hms(2000, 02, 16, 00, 00, 00);
//     unsafe {
//         println!("{:?}", d1.unwrap().timestamp());
//         let t_now = esp_idf_svc::sys::timeval {
//             tv_sec: d1.unwrap().timestamp() as _,
//             tv_usec: 0,
//         };
//         let tz = esp_idf_svc::sys::timezone {
//             ..Default::default()
//         };
//         esp_idf_svc::sys::settimeofday(&t_now, &tz);
//     }
//     println!(
//         "{:?}",
//         d1.unwrap().signed_duration_since(d2.unwrap()).num_weeks()
//     );
//     let current_time: chrono::DateTime<chrono::Utc> = Utc::now();
//     println!("{:?}", current_time);
//     // let mut soil_sensor= soil_sensor::new();
//     // soil_sensor.init();
//     // let mut ht_sensor = ht_sensor::new();
//     // ht_sensor.init();
//     // let criteria = ||
//     read_sensor(Sensor);
//     let pm_conf = esp_pm_config_t {
//         light_sleep_enable: true,
//         min_freq_mhz: 10,
//         max_freq_mhz: 40,
//     };
//     println!("res= {}", unsafe {
//         esp_idf_svc::sys::esp_pm_configure(&pm_conf as *const _ as *const c_void)
//     });
//     for i in 0..10 {
//         println!("Hello");
//         delay::FreeRtos::delay_ms(1000);
//     }
//     let pm_conf = esp_pm_config_t {
//         light_sleep_enable: false,
//         min_freq_mhz: 10,
//         max_freq_mhz: 40,
//     };
//     println!("res= {}", unsafe {
//         esp_idf_svc::sys::esp_pm_configure(&pm_conf as *const _ as *const c_void)
//     });
//     // unsafe {
//     //     let mut k = esp_idf_svc::sys::esp_partition_find_first(
//     //         esp_idf_svc::sys::esp_partition_type_t_ESP_PARTITION_TYPE_ANY as _,
//     //         esp_idf_svc::sys::esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY as _,
//     //         "spifs\0".as_ptr() as _,
//     //     );
//     //     println!("partition: {:?}", k);
//     //     let conf = esp_idf_svc::sys::esp_vfs_spiffs_conf_t {
//     //         base_path: "/main\0".as_ptr() as _,
//     //         partition_label: "spifs\0".as_ptr() as _,
//     //         max_files: 3,
//     //         format_if_mount_failed: true,
//     //     };
//     //     println!("{:?}", esp_idf_svc::sys::esp_vfs_spiffs_register(&conf));
//     //     {
//     //         let mut file = std::fs::File::create("/main/k.png").unwrap();
//     //         file.write_all("Hello".as_bytes());
//     //     }
//     //     {
//     //         let mut buf = [0u8; 5];
//     //         let mut file = std::fs::File::open("/main/k.png").unwrap();
//     //         file.read_exact(&mut buf);
//     //         println!("{:?}",buf);
//     //     }
//     // }
//     // log::info!("Hello, world!");
// }
