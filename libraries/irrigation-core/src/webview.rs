use esp_idf_svc::{
    http::server::{Configuration, EspHttpServer},
    io::EspIOError,
};


pub trait WebModule {
    /// ## Description
    /// Add module specific `api` calls
    fn add_hundler<'a>(&self, http_server: &mut EspHttpServer<'a>);
}

///
/// ## Description
/// Web interface management
pub struct WebView<'a> {
    http_server: EspHttpServer<'a>,
}

impl<'a> WebView<'a> {
    /// ## Description
    /// Create a new http server on default ports (80,443) to serve a web interface
    pub fn new() -> Result<Self, EspIOError> {
        let http_server = esp_idf_svc::http::server::EspHttpServer::new(&Configuration {
            ..Default::default()
        });
        if let Ok(valid_http_server) = http_server {
            return Ok(Self {
                http_server: valid_http_server,
            });
        }
        return Err(http_server.err().unwrap());
    }
    /// ## Description
    /// Add a module the interface
    /// the module must add a uri handler to serve its requests
    /// ## Parameters
    /// * `module`: A struct with `WebModule` implemented correctly
    pub fn add_module(&mut self, module: impl WebModule) {
        module.add_hundler(&mut self.http_server);
    }
}
