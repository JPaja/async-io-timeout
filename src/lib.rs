cfg_if::cfg_if! {
    if #[cfg(any(feature = "runtime-tokio", feature = "runtime-embassy"))] {
        mod deadline;
        mod runtime;

        pub(crate) use runtime::*;
        pub use deadline::*;
    }
    else
    {
        compile_error!("You must enable either 'runtime-tokio' or 'runtime-embassy' feature.");
    }
}
