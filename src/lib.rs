pub mod device;
pub mod display;
pub mod keyboard;
pub mod mouse;

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;
