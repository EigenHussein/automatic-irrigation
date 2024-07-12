use std::{fmt::Write, io::Read, net::TcpListener};

use esp_idf_svc::{
    hal::{
        delay,
        gpio::{self, Level, Pull},
        peripherals::Peripherals,
        units::Hertz,
    },
    sys::eNotifyAction_eNoAction,
};
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    //
    let periph = Peripherals::take().unwrap();

    let uart1_tx = unsafe { esp_idf_svc::hal::gpio::AnyIOPin::new(25) };
    let uart1_rx = unsafe { esp_idf_svc::hal::gpio::AnyIOPin::new(33) };
    let uart2_tx = unsafe { esp_idf_svc::hal::gpio::AnyIOPin::new(26) };
    let uart2_rx = unsafe { esp_idf_svc::hal::gpio::AnyIOPin::new(27) };

    let mut uart1 = esp_idf_svc::hal::uart::UartDriver::new(
        periph.uart1,
        uart1_tx,
        uart1_rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &esp_idf_svc::hal::uart::config::Config::default().baudrate(Hertz { 0: 115_200 }),
    )
    .unwrap();

    let mut uart2 = esp_idf_svc::hal::uart::UartDriver::new(
        periph.uart2,
        uart2_tx,
        uart2_rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &esp_idf_svc::hal::uart::config::Config::default().baudrate(Hertz { 0: 115_200 }),
    )
    .unwrap();

    let mut wifi =
        cycloid::wifi_module::WiFiModule::new_client(periph.modem, ("Curl", "@OpenGL64@")).unwrap();

    while !wifi.mWiFi.is_connected().unwrap() {
        println!("Waiting for connection");
        delay::FreeRtos::delay_ms(100);
    }
    let mut tcp_server = TcpListener::bind("0.0.0.0:1610").unwrap();

    // loop {
    //     uart1.write_str("Hello");
    //     let mut mybuf = [0u8; 5];
    //     println!("{:?}", uart2.read(&mut mybuf, 100));
    //     println!("{:?}", mybuf);
    //     delay::FreeRtos::delay_ms(1000);
    // }
    let mut msg_buf = [0u8; 256];
    for _client in tcp_server.incoming() {
        let mut len = 0u32;
        let buf: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(&mut len as *mut u32 as *mut u8, 4) };
        if let Ok(mut client) = _client {
            loop {
                if let Ok(recv) = client.read(buf) {
                    if len > 256 {
                        //client.flush();
                        std::io::Write::flush(&mut client);
                        continue;
                    }
                    client.read_exact(&mut msg_buf[0..recv]);
                    uart1.write(&msg_buf[0..recv]);
                    println!("The Message sent is :{:?}", msg_buf);
                } else {
                    break;
                }
            }
        }
    }
}
