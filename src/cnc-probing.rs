use std::{
    io::{Read, Write},
    net::TcpListener,
};

use esp_idf_svc::hal::{
    delay,
    gpio::{Level, Pull},
    peripherals::Peripherals,
};


fn main(){
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let periph = Peripherals::take().unwrap();

    


}

fn main2() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    //
    let periph = Peripherals::take().unwrap();
    let mut probe = esp_idf_svc::hal::gpio::PinDriver::input(periph.pins.gpio27).unwrap();
    probe.set_pull(Pull::Down).unwrap();

    let mut wifi =
        cycloid::wifi_module::WiFiModule::new_client(periph.modem, ("Curl", "@OpenGL64@")).unwrap();

    while !wifi.mWiFi.is_connected().unwrap() {
        println!("Waiting for connection");
        delay::FreeRtos::delay_ms(100);
    }
    let mut tcp_server = TcpListener::bind("0.0.0.0:1610").unwrap();

    for _client in tcp_server.incoming() {
        if let Ok(mut client) = _client {
            loop {
                let mut magic_number = 0u32;
                let val: &mut [u8] = unsafe {
                    std::slice::from_raw_parts_mut(&mut magic_number as *mut u32 as _, 4)
                };
                if let Ok(_) = client.read_exact(val) {
                    if magic_number == 0x8f900e9du32 {
                        let level = probe.get_level();
                        println!("Level: {:?}", level);
                        if level == Level::High {
                            client.write(1u32.to_le_bytes().as_slice());
                        } else {
                            client.write(0u32.to_le_bytes().as_slice());
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
}
