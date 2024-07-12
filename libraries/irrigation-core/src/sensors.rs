use esp_idf_svc::{http::headers, io::Write};

use crate::webview::WebModule;

///
/// ## Humidity and Temperature Sensor
/// SHTC3 Sensor
///
pub mod atm_sensor {
    use esp_idf_svc::io::Write;

    use crate::webview::WebModule;

    /// * Humidity and Temperature values
    #[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub struct SensorValue {
        pub h: f32,
        pub t: f32,
    }
    /// ## Description
    /// SHTC3's I2C sensor driver
    pub struct ATMSensorDriver {
        pub i2c_sda_pin: i32,
        pub i2c_scl_pin: i32,
    }

    impl ATMSensorDriver {
        /// ## Description
        /// Initialize SHTC3's library
        /// * Parameters
        /// * `sda` and `scl` pins of I2C
        /// * `frequency`: I2C frequency (Max. 400000)
        /// ## Example
        /// ```
        /// let sensor = ATMSensorDriver::init(21, 22, 400000);
        /// let val = sensor.read_value();
        /// ```
        pub fn init(sda: i32, scl: i32, frequency: i32) -> Self {
            unsafe { esp_idf_svc::sys::my_sensor::shtc3_init(sda, scl, frequency) };
            Self {
                i2c_sda_pin: sda,
                i2c_scl_pin: scl,
            }
        }
        /// ## Description
        /// Read a `SensorValue`
        /// ## Notes
        /// this function is
        pub fn read_value() -> SensorValue {
            let mut tmp = SensorValue::default();
            unsafe {
                esp_idf_svc::sys::my_sensor::shtc3_wake_up(0x70);
                esp_idf_svc::sys::my_sensor::shtc3_read(
                    0x70,
                    esp_idf_svc::sys::my_sensor::T_FIRST_N as _,
                    &mut tmp.t,
                    &mut tmp.h,
                );
                esp_idf_svc::sys::my_sensor::shtc3_sleep(0x70);
            }
            tmp
        }
        pub fn read_val(&self) -> SensorValue {
            return Self::read_value();
        }
    }
    impl WebModule for ATMSensorDriver {
        fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {
            http_server
                .fn_handler("/atm_sensor", esp_idf_svc::http::Method::Get, |res| {
                    res.into_ok_response()
                        .unwrap()
                        .write_all(
                            serde_json::to_string(&Self::read_value())
                                .unwrap()
                                .as_bytes(),
                        )
                        .unwrap_or_default();
                    Ok(())
                })
                .unwrap();
        }
    }
}

static mut SoilSensor_current_value: f32 = 0.0;
pub struct SoilSensor {
    // sensor_value: f32,
}
impl SoilSensor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_value(&mut self) -> f32 {
        unsafe { SoilSensor_current_value = f32::sin(3112.0) };
        return unsafe { SoilSensor_current_value };
    }
    // pub fn get_ptr(&self) -> *const f32 {
    //     return &self.sensor_value;
    // }
}
impl WebModule for SoilSensor {
    fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {
        // let ptr = self.get_ptr();
        let _ = http_server.fn_handler(
            "/get_soild_humidity",
            esp_idf_svc::http::Method::Get,
            move |res| {
                let resp = res.into_ok_response();
                let val = unsafe { SoilSensor_current_value }.to_string();
                if let Ok(mut responsible) = resp {
                    responsible.write(val.as_bytes());
                }
                Ok(())
            },
        );
    }
}
