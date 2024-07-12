use std::{
    io::{Read, Write},
    net::TcpStream,
};

/// ### Description
/// * Recevie a struct `T` using `TcpSteam`
/// #### Parameters
/// * `connection`: A mutable reference to a `TcpStream`
/// #### Notes
/// * `T` type must be `Sized` and implement `Default` trait
/// ### Example
/// ```
/// let received_struct: firmware_info = receive_struct::<firmware_info>(&mut client).unwrap();
/// ```

pub fn receive_struct<T>(connection: &mut TcpStream) -> Result<T, ()>
where
    T: Default + Sized,
{
    let mut tmp = T::default();
    let ptr: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(&mut tmp as *mut T as *mut u8, std::mem::size_of::<T>())
    };
    if let Ok(_) = connection.read_exact(ptr) {
        return Ok(tmp);
    }

    Err(())
}
/// ### Description
/// * Send a struct `T` using `TcpSteam`
/// #### Parameters
/// * `connection`: A mutable reference to a `TcpStream`
/// #### Notes
/// * `T` type must be `Sized`
/// ### Example
/// ```
/// let mut my_struct = struct_type{...};
/// send_struct::<struct_type>(&mut client, &my_struct).unwrap();
/// ```
pub fn send_struct<T>(connection: &mut TcpStream, obj: &T) -> std::io::Result<()>
where
    T: Sized,
{
    let ptr: &[u8] = unsafe {
        std::slice::from_raw_parts(obj as *const T as *const u8, std::mem::size_of::<T>())
    };
    connection.write_all(ptr)
}
