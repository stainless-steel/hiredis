extern crate hiredis_sys as raw;
extern crate libc;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::from($message)));
);

macro_rules! success(
    ($raw:expr) => (unsafe {
        if (*$raw).err != ::raw::REDIS_OK {
            return Err(::Error {
                message: Some(c_str_to_string!((*$raw).errstr.as_ptr() as *const _)),
            });
        }
    });
);

macro_rules! str_to_c_str(
    ($string:expr) => (
        match ::std::ffi::CString::new($string) {
            Ok(string) => string.as_ptr(),
            Err(_) => raise!("failed to process a string"),
        }
    );
);

macro_rules! c_str_to_string(
    ($string:expr) => (
        String::from_utf8_lossy(::std::ffi::CStr::from_ptr($string).to_bytes()).into_owned()
    );
);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

mod context;
mod error;

pub use context::Context;
pub use error::Error;

/// Connect to a Redis server.
#[inline]
pub fn connect(address: &str, port: usize) -> Result<Context> {
    Context::new(address, port)
}
