#![no_std]

//! https://www.usb.org/sites/default/files/DWG_Smart-Card_CCID_Rev110.pdf
//! https://www.usb.org/sites/default/files/DWG_Smart-Card_USB-ICC_ICCD_rev10.pdf

pub mod constants;
pub mod class;
// writer
pub mod der;
// parser
pub mod derp;
pub mod pipe;
pub mod types;

// pub mod piv;

pub use class::Ccid;

logging::add!(logger);