// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Converter;
use FileInfo;
use ZlibCompressorFormat;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ZlibCompressor(Object<ffi::GZlibCompressor, ffi::GZlibCompressorClass>): Converter;

    match fn {
        get_type => || ffi::g_zlib_compressor_get_type(),
    }
}

impl ZlibCompressor {
    pub fn new(format: ZlibCompressorFormat, level: i32) -> ZlibCompressor {
        unsafe {
            from_glib_full(ffi::g_zlib_compressor_new(format.to_glib(), level))
        }
    }
}

pub trait ZlibCompressorExt {
    fn get_file_info(&self) -> Option<FileInfo>;

    fn set_file_info<'a, P: Into<Option<&'a FileInfo>>>(&self, file_info: P);

    fn get_property_format(&self) -> ZlibCompressorFormat;

    fn get_property_level(&self) -> i32;

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ZlibCompressor> + IsA<glib::object::Object>> ZlibCompressorExt for O {
    fn get_file_info(&self) -> Option<FileInfo> {
        unsafe {
            from_glib_none(ffi::g_zlib_compressor_get_file_info(self.to_glib_none().0))
        }
    }

    fn set_file_info<'a, P: Into<Option<&'a FileInfo>>>(&self, file_info: P) {
        let file_info = file_info.into();
        let file_info = file_info.to_glib_none();
        unsafe {
            ffi::g_zlib_compressor_set_file_info(self.to_glib_none().0, file_info.0);
        }
    }

    fn get_property_format(&self) -> ZlibCompressorFormat {
        unsafe {
            let mut value = Value::from_type(<ZlibCompressorFormat as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "format".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_level(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "level".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file-info",
                transmute(notify_file_info_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_file_info_trampoline<P>(this: *mut ffi::GZlibCompressor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ZlibCompressor> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ZlibCompressor::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for ZlibCompressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZlibCompressor")
    }
}
