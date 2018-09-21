#[cfg(not(debug_assertions))]
#[allow(unused)]
macro_rules! TODO {
    ($message:expr; $dummy:expr) => {
        compile_error!(concat!(
            "TODO: Must implement prior to release: ",
            $message
        ));
    };
    ($message:expr;) => {
        compile_error!(concat!(
            "TODO: Must implement prior to release: ",
            $message
        ));
    };
    ($message:expr) => {
        compile_error!(concat!(
            "TODO: Must implement prior to release: ",
            $message
        ));
    };
}

#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! TODO {
    ($message:expr; $dummy:expr) => {{
        $dummy
    }};
    ($message:expr;) => {{
        unimplemented!($message)
    }};
    ($message:expr) => {{
        unimplemented!($message)
    }};
}
