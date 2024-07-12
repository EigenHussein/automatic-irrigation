use control_port::{
    controller::{Condition, Func, Link},
    functions::TimeOfDayNormalized,
    sensors::*,
    Controller,
};
use esp_idf_svc::hal::delay::{self, FreeRtos};

fn main() {
    println!("Staring new One !");
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // shtc3_sensor::init_shtc3();

    // let mut poll = Polling::SensorPolling::default();
    // poll.add_sensor(shtc3_sensor::SHTC3Humidity)
    //     .add_sensor(TimeOfDayNormalized)
    //     .set_timer(1000)
    //     .start_polling();

    let mut simple_controller = Controller::ControlPorts::new(2);
    {
        let p1 = simple_controller.add_control_port(21);
        p1.lock().unwrap().add_link(
            Link::default()
                .add_condition(&Condition::new(
                    TimeOfDayNormalized.into(),
                    control_port::controller::Operations::Smaller(0.0),
                    Func::Constant(10.0),
                ))
                .add_condition(&Condition::new(
                    TimeOfDayNormalized.into(),
                    control_port::controller::Operations::Greater(0.0),
                    Func::Constant(40.0),
                ))
                .clone(),
        );
    }

    println!(
        "Serialization: {}",
        serde_json::to_string(&simple_controller).unwrap()
    );
    loop {
        println!("EfEh");
        FreeRtos::delay_ms(1000);
    }
    // Load Configuration

    // Init Sensors

    // Start Server

    // Start Control Ports

    // Poll for Conditions Checks

    // Poll for Server Updates
}

// fn main2() {
//     esp_idf_svc::sys::link_patches();
//     esp_idf_svc::log::EspLogger::initialize_default();
//     shtc3_sensor::init_shtc3();

//     let mut poll = Polling::SensorPolling::default();
//     poll.add_sensor(shtc3_sensor::SHTC3Humidity)
//         .add_sensor(TimeOfDayNormalized)
//         .set_timer(1000)
//         .start_polling();

//     let mut Fan_Controller = control_port::controller::ControlPort::new();
//     Fan_Controller.add_link(
//         Link::default()
//             .add_condition(&Condition::new(
//                 shtc3_sensor::SHTC3Humidity.into(),
//                 control_port::controller::Operations::Greater(0.0),
//                 Func::Constant(56.0),
//             ))
//             .add_condition(&Condition::new(
//                 control_port::functions::TimeOfDayNormalized.into(),
//                 control_port::controller::Operations::Greater(0.0),
//                 Func::Constant(0.0005785),
//             ))
//             .clone(),
//     );
//     println!("{}", serde_json::to_string(&Fan_Controller).unwrap());
//     loop {
//         let tmp = shtc3_sensor::SHTC3Temperature.get_value();
//         let hum = shtc3_sensor::SHTC3Humidity.get_value();
//         println!(
//             "{:?}",
//             (
//                 tmp,
//                 hum,
//                 control_port::functions::TimeOfDayNormalized.get_value(),
//                 Fan_Controller.eval_links()
//             )
//         );
//         FreeRtos::delay_ms(1000);
//     }
// }
