// // pub trait Sensor {
// //     fn init() -> Self;
// //     fn deinit(&self);
// // }
// pub mod atm_sensor {
//     #[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
//     pub struct SensorValue {
//         pub h: f32,
//         pub t: f32,
//     }
//     pub struct ATMSensorDriver {
//         pub i2c_sda_pin: i32,
//         pub i2c_scl_pin: i32,
//     }

//     impl ATMSensorDriver {
//         pub fn init(sda: i32, scl: i32, frequency: i32) -> Self {
//             unsafe { esp_idf_svc::sys::my_sensor::shtc3_init(sda, scl, frequency) };
//             Self {
//                 i2c_sda_pin: sda,
//                 i2c_scl_pin: scl,
//             }
//         }
//         pub fn read_value() -> SensorValue {
//             let mut tmp = SensorValue::default();
//             unsafe {
//                 esp_idf_svc::sys::my_sensor::shtc3_wake_up(0x70);
//                 esp_idf_svc::sys::my_sensor::shtc3_read(
//                     0x70,
//                     esp_idf_svc::sys::my_sensor::T_FIRST_N as _,
//                     &mut tmp.t,
//                     &mut tmp.h,
//                 );
//                 esp_idf_svc::sys::my_sensor::shtc3_sleep(0x70);
//             }
//             tmp
//         }
//     }
// }

// pub struct SoilSensor {}
