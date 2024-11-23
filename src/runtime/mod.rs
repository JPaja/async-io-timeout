#![allow(dead_code)]

#[cfg(feature = "runtime-tokio")]
mod tokio;
#[cfg(feature = "runtime-tokio")]
pub use tokio::*;

#[cfg(feature = "runtime-embassy")]
mod embassy;
#[cfg(feature = "runtime-embassy")]
pub use embassy::*;


// Compile time check if Sleep, Duration and Instant are Unpin 
const fn test_unpin<'a, T: Unpin>() -> u32 {
    0
}
const SLEEP_UNPIN_CHECK: u32 = test_unpin::<Sleep>();
const DURATION_UNPIN_CHECK: u32 = test_unpin::<Duration>();
const INSTNAT_UNPIN_CHECK: u32 = test_unpin::<Instant>();
