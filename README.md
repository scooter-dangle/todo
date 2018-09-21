TODO
====

Usage

```rust
#[allow(unused)]
#[macro_use]
extern crate todo;

fn my_function() -> Result<(), String> {
    TODO!(
        "`my_function` should return something _really groundbreaking_";
        Ok(())
    )
}

fn main() {
    assert_eq!(my_function().unwrap(), ());
}
```

When compiled with debug assertions, the `Ok(())` dummy value will be used and
your code will type-check and run without having to use `unimplemented!()`
(although if you don't provide the dummy value, it will fall back to using
`unimplemented!()` with your TODO message).

When you build in release mode, though, with debug assertions on, you will get a
compile-time error containing your TODO message.

Is this useful?
---------------
It _was_ motivated by a specific case at work where we had a bunch of structs,
introduced a trait, and then split up the work of implementing that trait for
each of the structs. One of those structs' implementations would call the
implementation of another struct, _but we totally forgot to do that_ since we
didn't provide a dummy trait implementation on the other struct and then forgot
to come back to the first structs' implementation.

That might not be the best pitch for this, but I _think_ it _might_ have helped.

Is this a really small thing?
-----------------------------
Yes

Maybe way too small to justify pulling it in as a crate dependency?
-------------------------------------------------------------------
Maybe. Hmm. Yeah, you might have a point there. If it's too annoyingly small
to pull in as a crate, but you think it might be useful for something you're
doing, you could just copy/paste the macro declaration into your code (no hard
feelings!):

```rust
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
```
