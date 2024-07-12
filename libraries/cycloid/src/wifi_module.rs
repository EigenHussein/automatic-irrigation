//Version 1.1

use esp_idf_svc::hal::modem;
use esp_idf_svc::wifi::AuthMethod;

/// ## Description
/// An easy wrapper to use WiFi
pub struct WiFiModule<'a> {
    pub mWiFi: esp_idf_svc::wifi::EspWifi<'a>,
}

impl<'a> WiFiModule<'_> {
    /// ## Description
    /// Start wifi client by connecting to an access point
    /// ## Parameters
    /// * `perph_modem`: consumption of the wifi modem
    /// * `(ssid, pass)`: A tuple of SSID and Password of the access-point
    pub fn new_client(
        perph_modem: modem::Modem,
        (ssid, pass): (&str, &str),
    ) -> Result<WiFiModule<'a>, i32> {
        let ev_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
        let nvs_partition = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();
        let wifi = esp_idf_svc::wifi::EspWifi::new(perph_modem, ev_loop, Some(nvs_partition));
        if let Ok(mut wifi_ok) = wifi {
            if let Err(err_conf) = wifi_ok.set_configuration(
                &esp_idf_svc::wifi::Configuration::Client(esp_idf_svc::wifi::ClientConfiguration {
                    ssid: ssid.into(),
                    password: pass.into(),

                    ..Default::default()
                }),
            ) {
                println!("Error configuring wifi: {}", err_conf);
                return Err(-1);
            }
            if let Err(e_wifi) = wifi_ok.start() {
                println!("Error ({}) starting wifi!", e_wifi);
                return Err(-2);
            }

            if let Err(wifi_connect_error) = wifi_ok.connect() {
                return Err(wifi_connect_error.code());
            }

            let rs = WiFiModule { mWiFi: wifi_ok };
            return Ok(rs);
        }
        Err(-3)
    }

    /// ## Description
    /// Start a wifi access point
    /// ## Parameters
    /// * `perph_modem`: consumption of the wifi modem
    /// * `(ssid, pass)`: A tuple of SSID and Password of the access-point
    pub fn new_access_point(
        perph_modem: modem::Modem,
        (ssid, pass): (&str, &str),
    ) -> Result<WiFiModule<'a>, i32> {
        let ev_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
        let nvs_partition = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();
        let wifi = esp_idf_svc::wifi::EspWifi::new(perph_modem, ev_loop, Some(nvs_partition));

        if let Ok(mut wifi_ok) = wifi {
            if let Err(err_conf) =
                wifi_ok.set_configuration(&esp_idf_svc::wifi::Configuration::AccessPoint(
                    esp_idf_svc::wifi::AccessPointConfiguration {
                        ssid: ssid.into(),
                        password: pass.into(),
                        auth_method: AuthMethod::WPA2Personal,
                        ..Default::default()
                    },
                ))
            {
                println!("Error configuring wifi: {}", err_conf);
                return Err(-1);
            }

            if let Err(e_wifi) = wifi_ok.start() {
                println!("Error ({}) starting wifi!", e_wifi);
                return Err(-2);
            }
            let rs = WiFiModule { mWiFi: wifi_ok };
            return Ok(rs);
        }
        Err(-3)
    }

    /// ## Description
    /// Start wifi with costumized configuration
    /// ## Parameters
    /// * `perph_modem`: consumption of the wifi modem
    /// * `(ssid, pass)`: A tuple of SSID and Password of the access-point
    pub fn new_custom(
        perph_modem: modem::Modem,
        configuration: esp_idf_svc::wifi::Configuration,
    ) -> Result<WiFiModule<'a>, i32> {
        let ev_loop = esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap();
        let nvs_partition = esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();
        let wifi = esp_idf_svc::wifi::EspWifi::new(perph_modem, ev_loop, Some(nvs_partition));

        if let Ok(mut wifi_ok) = wifi {
            if let Err(err_conf) = wifi_ok.set_configuration(&configuration) {
                println!("Error configuring wifi: {}", err_conf);
                return Err(-1);
            }

            if let Err(e_wifi) = wifi_ok.start() {
                println!("Error ({}) starting wifi!", e_wifi);
                return Err(-2);
            }
            let rs = WiFiModule { mWiFi: wifi_ok };
            return Ok(rs);
        }
        Err(-3)
    }
}
