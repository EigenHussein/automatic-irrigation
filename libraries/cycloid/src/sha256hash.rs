/// ## Description
/// `SHA256` checksum utility
pub struct Sha256Context {
    context: esp_idf_svc::sys::mbedtls_sha256_context,
}

pub trait TLSHash {
    fn new() -> Result<Self, ()>
    where
        Self: Sized;
    fn update(&mut self, input: &[u8]) -> Result<(), ()>;
    fn finish(&mut self) -> Result<[u8; 32], ()>;
}

impl TLSHash for Sha256Context {
    /// ## Descripion
    /// Start a new context
    fn new() -> Result<Self, ()> {
        let mut context = esp_idf_svc::sys::mbedtls_sha256_context::default();
        unsafe {
            esp_idf_svc::sys::mbedtls_sha256_init(&mut context);
        }
        //mbedtls_sha256_self_test(verbose)
        let ret = unsafe { esp_idf_svc::sys::mbedtls_sha256_starts(&mut context, 0) };

        if ret != 0 {
            Err(())
        } else {
            Ok(Self { context })
        }
    }

    /// ## Description
    /// Update the state of the hash
    fn update(&mut self, input: &[u8]) -> Result<(), ()> {
        let ret = unsafe {
            esp_idf_svc::sys::mbedtls_sha256_update(
                &mut self.context,
                input.as_ptr() as *const u8,
                input.len(),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// ## Description
    /// Finish the hash and return the checksum
    fn finish(&mut self) -> Result<[u8; 32], ()> {
        let mut output = [0u8; 32];
        let ret =
            unsafe { esp_idf_svc::sys::mbedtls_sha256_finish(&mut self.context, output.as_mut_ptr()) };

        if ret == 0 {
            Ok(output)
        } else {
            Err(())
        }
    }
}

impl Drop for Sha256Context {
    fn drop(&mut self) {
        unsafe { esp_idf_svc::sys::mbedtls_sha256_free(&mut self.context) }
    }
}
