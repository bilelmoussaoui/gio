// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ db49619)
// DO NOT EDIT

use IOStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use InputStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use OutputStream;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SimpleIOStream(Object<ffi::GSimpleIOStream>): IOStream;

    match fn {
        get_type => || ffi::g_simple_io_stream_get_type(),
    }
}

impl SimpleIOStream {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn new<P: IsA<InputStream>, Q: IsA<OutputStream>>(input_stream: &P, output_stream: &Q) -> SimpleIOStream {
        unsafe {
            IOStream::from_glib_full(ffi::g_simple_io_stream_new(input_stream.to_glib_none().0, output_stream.to_glib_none().0)).downcast_unchecked()
        }
    }
}
