#![feature(unchecked_math)] // allow unchecked math
#![allow(warnings)]

extern crate parity_wasm;

#[cfg(test)]
#[macro_use]
extern crate serde;

#[cfg(test)]
extern crate serde_json;

use core::ops::Deref;
use core::fmt::Formatter;
use core::fmt::Debug;

pub mod types;
mod hex;


macro_rules! impl_from {
    ($debug: ty) => {
        impl From<$debug> for StringErr {
            fn from(e: $debug) -> StringErr {
                StringErr(format!("{:?}", e))
            }
        }
    };
}

impl_from!(String);
impl_from!(parity_wasm::SerializationError);

// Error handling utils
pub struct StringErr(pub String);

impl StringErr {
    fn new<T: Deref<Target=str>>(t: T) -> Self {
        StringErr(t.to_string())
    }
}

impl Debug for StringErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self)
    }
}

impl Deref for StringErr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
