#[macro_export]
macro_rules! unique_features {
    () => {};
    ($first:tt $(,$rest:tt)*,) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
       unique_features!($($rest),*);
    }
}
