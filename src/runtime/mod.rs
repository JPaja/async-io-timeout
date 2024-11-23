#[cfg(feature = "runtime-tokio")]
mod tokio;
#[cfg(feature = "runtime-tokio")]
pub use tokio::*;

#[cfg(feature = "runtime-embassy")]
mod embassy;
#[cfg(feature = "runtime-embassy")]
pub use embassy::*;

mod test;
