#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!("bindings.rs");

#[cfg(feature = "midi")]
pub mod midi {
    include!("midi_bindings.rs");
}

#[cfg(feature = "trill")]
pub mod trill {
    include!("trill_bindings.rs");
}
