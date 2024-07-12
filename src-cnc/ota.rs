use std::{net::TcpListener, time::Duration};

use cycloid::esp_connection;
use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
    sys::{
        esp_ota_set_boot_partition, esp_partition_find_first,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_APP_FACTORY,
        esp_partition_type_t_ESP_PARTITION_TYPE_ANY, esp_partition_type_t_ESP_PARTITION_TYPE_APP,
    },
};

fn swap_partitions() {
    unsafe {
        let partition = esp_partition_find_first(
            esp_partition_type_t_ESP_PARTITION_TYPE_ANY,
            esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
            "factory\0".as_ptr() as _,
        );
        println!("{:?}", partition);
        esp_ota_set_boot_partition(partition);
    }
}

fn factory_reset_handler(connection_timout: u32) {
    let mut reset_server = TcpListener::bind("0.0.0.0:1611").unwrap();

    let time_start = std::time::SystemTime::now().elapsed().unwrap().as_secs();
    loop {
        println!("Waiting for new connection!");
        let mut client = reset_server.accept().unwrap();
        println!("New client is connected!");
        // client.0.set_nonblocking(true).unwrap();
        client
            .0
            .set_read_timeout(Some(std::time::Duration::from_secs(60)));
        if let Ok(val) = esp_connection::receive_struct::<u32>(&mut client.0) {
            if val == 0x8f900e9d {
                FreeRtos::delay_ms(10000);
                swap_partitions();
                println!("Restarting...");
                FreeRtos::delay_ms(1000);
                unsafe { esp_idf_svc::sys::esp_restart() };
            }
            FreeRtos::delay_ms(10000);
            println!("Error magic_number");
            // break;
        }
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    println!("Starting OTA route!");
    let periph = Peripherals::take().unwrap();
    let mut cnc_access_point =
        cycloid::wifi_module::WiFiModule::new_access_point(periph.modem, ("CNC", "OpeOpenoMi"))
            .unwrap();
    let reset_thread = std::thread::spawn(|| factory_reset_handler(10));

    app_main();

    loop {
        FreeRtos::delay_ms(1000);
    }
}

fn app_main() {
    loop {
        println!("CNC is working Darnapheseos2");
        FreeRtos::delay_ms(1000);
    }
}
