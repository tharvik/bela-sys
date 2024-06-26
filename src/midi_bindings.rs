/* automatically generated by rust-bindgen 0.69.4 */

pub type Midi = *mut ::std::os::raw::c_void;
extern "C" {
    #[doc = " C API for the Midi class. Only a subset of\n methods is currently implemented"]
    pub fn Midi_availableMessages(midi: *mut Midi) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Midi_getMessage(
        midi: *mut Midi,
        buf: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " Creates a new BelaMidi object and listens to port.\n\n @param port the port to listen on\n\n @return a pointer to the object, or NULL if creation failed"]
    pub fn Midi_new(port: *const ::std::os::raw::c_char) -> *mut Midi;
}
extern "C" {
    pub fn Midi_delete(midi: *mut Midi);
}
