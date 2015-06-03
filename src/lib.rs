extern crate hiredis_sys as raw;
extern crate libc;

use libc::{c_char, c_int, size_t};
use std::convert::{From, Into};
use std::marker::PhantomData;

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

/// An argument.
pub struct Argument {
    pointer: *mut c_char,
    size: size_t,
}

/// A context.
pub struct Context {
    raw: *mut raw::redisContext,
    phantom: PhantomData<raw::redisContext>,
}

/// An error.
#[derive(Debug)]
pub struct Error {
    pub message: Option<String>,
}

/// A reply.
pub struct Reply {
    raw: *mut raw::redisReply,
    phantom: PhantomData<raw::redisReply>,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl<'l> From<&'l &'l str> for Argument {
    #[inline]
    fn from(data: &'l &'l str) -> Argument {
        Argument {
            pointer: data.as_ptr() as *mut u8 as *mut _,
            size: data.len() as size_t,
        }
    }
}

impl Context {
    /// Create a context by establishing connection to a server.
    pub fn new(address: &str, port: usize) -> Result<Context> {
        let context = Context {
            raw: unsafe {
                let raw = raw::redisConnect(str_to_c_str!(address), port as c_int);
                if raw.is_null() {
                    raise!("failed to create a context");
                }
                raw
            },
            phantom: PhantomData,
        };
        success!(context.raw);
        Ok(context)
    }

    /// Issue a command.
    pub fn command<'l, T>(&mut self, arguments: &'l [T]) -> Result<Reply>
        where &'l T: Into<Argument>
    {
        let argc = arguments.len();
        let mut argv = Vec::with_capacity(argc);
        let mut argvlen = Vec::with_capacity(argc);
        for argument in arguments.iter() {
            let Argument { pointer, size } = argument.into();
            argv.push(pointer);
            argvlen.push(size);
        }
        let raw = unsafe {
            raw::redisCommandArgv(self.raw, argc as c_int, argv.as_ptr() as *const *const _,
                                  argvlen.as_ptr())
        };
        success!(self.raw);
        debug_assert!(!raw.is_null());
        Ok(Reply { raw: raw as *mut _, phantom: PhantomData })
    }

    /// Reconnect to the server.
    #[inline]
    pub fn reconnect(&mut self) -> Result<()> {
        if unsafe { raw::redisReconnect(self.raw) } != raw::REDIS_OK {
            raise!("failed to reconnect");
        }
        Ok(())
    }
}

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::redisFree(self.raw) };
    }
}

impl<T> From<T> for Error where T: Into<String> {
    #[inline]
    fn from(message: T) -> Error {
        Error {
            message: Some(message.into()),
        }
    }
}

impl Drop for Reply {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::freeReplyObject(self.raw as *mut _) };
    }
}

/// Connect to a Redis server.
#[inline]
pub fn connect(address: &str, port: usize) -> Result<Context> {
    Context::new(address, port)
}
