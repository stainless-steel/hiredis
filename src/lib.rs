extern crate hiredis_sys as raw;
extern crate libc;

use libc::{c_char, c_int, size_t};
use std::convert::{From, Into};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::fmt::{self, Display, Formatter};
use std::{mem, slice};

macro_rules! raise(
    ($message:expr) => (return Err(Error::from($message)));
);

macro_rules! success(
    ($context:expr) => (unsafe {
        if (*$context.raw).err != raw::REDIS_OK {
            return Err(Error {
                kind: ErrorKind::from((*$context.raw).err as isize),
                message: c_str_to_string!((*$context.raw).errstr.as_ptr() as *const _),
            });
        }
    });
);

macro_rules! str_to_c_str(
    ($string:expr) => (
        match CString::new($string) {
            Ok(string) => string.as_ptr(),
            Err(_) => raise!("failed to process a string"),
        }
    );
);

macro_rules! c_str_to_string(
    ($string:expr, $size:expr) => ({
        let slice: &CStr = mem::transmute(slice::from_raw_parts($string as *const c_char,
                                                                $size as usize + 1));
        String::from_utf8_lossy(slice.to_bytes()).into_owned()
    });
    ($string:expr) => ({
        String::from_utf8_lossy(CStr::from_ptr($string).to_bytes()).into_owned()
    });
);

macro_rules! c_str_to_vec_u8(
    ($string:expr, $size:expr) => ({
        let slice: &[u8] = mem::transmute(slice::from_raw_parts($string as *const c_char,
                                                                $size as usize));
        Vec::from(slice)
    });
);

/// A trait for command arguments.
pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

/// A context.
pub struct Context {
    raw: *mut raw::redisContext,
    phantom: PhantomData<raw::redisContext>,
}

/// An error.
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

/// An error kind.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    InputOutput = raw::REDIS_ERR_IO as isize,
    EndOfFile = raw::REDIS_ERR_EOF as isize,
    Protocol = raw::REDIS_ERR_PROTOCOL as isize,
    OutOfMemory = raw::REDIS_ERR_OOM as isize,
    Other = raw::REDIS_ERR_OTHER as isize,
}

/// A reply of a command.
pub enum Reply {
    Status(String),
    Integer(i64),
    Bulk(Vec<u8>),
    Array,
    Nil,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

impl<'l> AsBytes for &'l str {
    #[inline]
    fn as_bytes(&self) -> &[u8] { (*self).as_bytes() }
}

impl<'l> AsBytes for &'l [u8] {
    #[inline]
    fn as_bytes(&self) -> &[u8] { self }
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
        success!(context);
        Ok(context)
    }

    /// Issue a command.
    pub fn command<T: AsBytes>(&mut self, arguments: &[T]) -> Result<Reply> {
        let argc = arguments.len();
        let mut argv: Vec<*const c_char> = Vec::with_capacity(argc);
        let mut argvlen = Vec::with_capacity(argc);
        for argument in arguments.iter() {
            let data = argument.as_bytes();
            argv.push(data.as_ptr() as *const _ as *const _);
            argvlen.push(data.len() as size_t);
        }

        let raw = unsafe {
            raw::redisCommandArgv(self.raw, argc as c_int, argv.as_ptr() as *const *const _,
                                  argvlen.as_ptr()) as *mut raw::redisReply
        };

        success!(self);
        debug_assert!(!raw.is_null());

        unsafe {
            let reply = match (*raw).kind {
                raw::REDIS_REPLY_STATUS => {
                    Reply::Status(c_str_to_string!((*raw).string, (*raw).len))
                },
                raw::REDIS_REPLY_INTEGER => {
                    Reply::Integer((*raw).integer as i64)
                },
                raw::REDIS_REPLY_NIL => {
                    Reply::Nil
                }
                raw::REDIS_REPLY_STRING => {
                    Reply::Bulk(c_str_to_vec_u8!((*raw).string, (*raw).len))
                },
                raw::REDIS_REPLY_ARRAY => {
                    Reply::Array
                },
                raw::REDIS_REPLY_ERROR => {
                    let message = c_str_to_string!((*raw).string, (*raw).len);
                    raw::freeReplyObject(raw as *mut _);
                    raise!(message);
                },
                _ => {
                    raw::freeReplyObject(raw as *mut _);
                    raise!("failed to identify a reply");
                },
            };
            raw::freeReplyObject(raw as *mut _);

            Ok(reply)
        }
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
        Error { kind: ErrorKind::Other, message: message.into() }
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.message, formatter)
    }
}

impl From<isize> for ErrorKind {
    #[inline]
    fn from(code: isize) -> ErrorKind {
        use ErrorKind::*;
        match code as c_int {
            raw::REDIS_ERR_IO => InputOutput,
            raw::REDIS_ERR_EOF => EndOfFile,
            raw::REDIS_ERR_PROTOCOL => Protocol,
            raw::REDIS_ERR_OOM => OutOfMemory,
            _ => Other,
        }
    }
}

/// Connect to a Redis server.
#[inline]
pub fn connect(address: &str, port: usize) -> Result<Context> {
    Context::new(address, port)
}
