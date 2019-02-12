//!# heca-lib
//! heca-lib is a Hebrew <-> Gregorian converter library. It's the backend behind the heca program.
//!
//! # Usage:
//!
//! 1. Add to Cargo.toml:
//!
//!```toml
//!     [dependencies]
//!     heca-lib = "*"
//!```
//!
//! 2. Add the following to your crate root:
//!
//! ```
//! extern crate heca_lib;
//!
//! ```
//! 3. Import the types:
//!
//!```
//!use heca_lib::*;
//!```
//!
//! # Overview:
//!
//! This library converts from Hebrew to Gregorian dates and back. You can get a HebrewDate either from a known Hebrew date or from a Gregorian date:
//!
//! ```
//!
//! extern crate heca_lib;
//!
//! use chrono::Utc;
//! use chrono::offset::TimeZone;
//! use heca_lib::{HebrewDate,HebrewMonth};
//!
//!assert_eq!(HebrewDate::from_gregorian(Utc.ymd(2018,9,10).and_hms(17,59,59)).unwrap(),HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,1).unwrap());
//!
//!```
//!
//!You can then get back a Gregorian date from this Hebrew Date.
//!
//!extern crate heca_lib;
//!
//!use chrono::Utc;
//!use chrono::offset::TimeZone;
//!use heca_lib::{HebrewDate,HebrewMonth};
//!
//!assert_eq!(HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,10).unwrap().to_gregorian(),Utc.ymd(2018, 9,18).and_hms(18,00,00));
//!
//!# Notes:
//!
//!1. This library won't work for years before 3764 (4).
//!2. This library is still unstable and the API may change at any time.
//!3. I tested this library against hebcal for all Rosh Hashanas between 3764 and 9999 (4-6239). I also checked it for all Rosh Chodesh Adars in those years. However, I take no resposibility if you accidently keep Yom Tov on the wrong day!
//!4. While this library _works_, there are still a few inefficienciess that need to be taken care of.

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
pub mod convert;
pub mod holidays;
pub mod types;
#[doc(inline)]
pub use convert::HebrewDate;
#[doc(inline)]
pub use types::{ConversionError, Day, HebrewMonth};
