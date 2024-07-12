use esp_idf_svc::io::Write;

use crate::webview::WebModule;



pub struct MainPage;

impl MainPage {
    fn load_functions() -> Vec<(&'static str, &'static str, &'static str)> {
        let source_index =
            include_str!("/home/hussein/Dev/Espressif/automatic-irrigation/webview/index.html");
        let source_style =
            include_str!("/home/hussein/Dev/Espressif/automatic-irrigation/webview/style.css");
        let source_js =
            include_str!("/home/hussein/Dev/Espressif/automatic-irrigation/webview/app.js");

        vec![
            ("/", "text/html", source_index),
            ("/style.css", "text/css", source_style),
            ("/app.js", "text/javascript", source_js),
        ]
    }
}
impl WebModule for MainPage {
    fn add_hundler<'a>(&self, http_server: &mut esp_idf_svc::http::server::EspHttpServer<'a>) {
        let components = Self::load_functions();
        for component in components.iter() {
            http_server
                .fn_handler(component.0, esp_idf_svc::http::Method::Get, |res| {
                    res.into_response(200, None, &[("Content-Type", component.1)])
                        .unwrap()
                        .write_all(component.2.as_bytes())
                        .unwrap_or(());
                    Ok(())
                })
                .unwrap();
        }
    }
}
