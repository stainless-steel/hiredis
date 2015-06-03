use libc::c_int;
use raw;
use std::marker::PhantomData;

use Result;

/// A context.
pub struct Context {
    raw: *mut raw::redisContext,
    phantom: PhantomData<raw::redisContext>,
}

impl Context {
    /// Create a context by establishing connection to a server.
    pub fn new(address: &str, port: usize) -> Result<Context> {
        let context = Context {
            raw: unsafe {
                let raw = raw::redisConnect(str_to_c_str!(address), port as c_int);
                if raw.is_null() {
                    raise!("cannot create a Hiredis context");
                }
                raw
            },
            phantom: PhantomData,
        };
        success!(context.raw);
        Ok(context)
    }
}

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::redisFree(self.raw) };
    }
}
