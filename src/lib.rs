#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]
#![doc = include_str!("../README.md")]

mod client;
mod error;

pub use crate::client::SrpClient;
pub use crate::error::CognitoSrpError;
