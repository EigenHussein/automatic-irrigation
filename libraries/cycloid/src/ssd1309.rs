fn sd1309_funcs() {
    unsafe {
        let mut ssd1306_ctx = esp_idf_svc::sys::my_sensor::SSD1306_t {
            ..Default::default()
        };
        esp_idf_svc::sys::my_sensor::i2c_master_init(&mut ssd1306_ctx, 5, 4, 0);
        esp_idf_svc::sys::my_sensor::ssd1306_init(&mut ssd1306_ctx, 128, 64);
        esp_idf_svc::sys::my_sensor::ssd1306_clear_screen(&mut ssd1306_ctx, true);
        esp_idf_svc::sys::my_sensor::ssd1306_contrast(&mut ssd1306_ctx, 0xff);
    }
}
