use crate::{sensors::*, webview::WebModule};

// pub struct AutomationModule<T: WebModule> {
//     pub atm_sensor: T,
// }

// impl<T: WebModule> AutomationModule<T> {
//     pub fn new(atm_sensor: T) -> Self {
//         Self {
//             atm_sensor: atm_sensor,
//         }
//     }
// }

// impl<T: WebModule> AutomationModule<T> {
//     fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {}
// }

pub struct AutomationModule {
    pub atm_sensor: atm_sensor::ATMSensorDriver,
    pub soil_sensor: SoilSensor,
}

impl AutomationModule {
    pub fn new(atm: atm_sensor::ATMSensorDriver, soil: SoilSensor) {
        
    }
}

impl WebModule for AutomationModule {
    fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {}
}
