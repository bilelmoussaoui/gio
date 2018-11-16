// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use Socket;
use SocketAddress;
use SocketFamily;
use SocketType;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SocketConnection(Object<ffi::GSocketConnection, ffi::GSocketConnectionClass>): IOStream;

    match fn {
        get_type => || ffi::g_socket_connection_get_type(),
    }
}

impl SocketConnection {
    pub fn factory_lookup_type(family: SocketFamily, type_: SocketType, protocol_id: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_socket_connection_factory_lookup_type(family.to_glib(), type_.to_glib(), protocol_id))
        }
    }

    pub fn factory_register_type(g_type: glib::types::Type, family: SocketFamily, type_: SocketType, protocol: i32) {
        unsafe {
            ffi::g_socket_connection_factory_register_type(g_type.to_glib(), family.to_glib(), type_.to_glib(), protocol);
        }
    }
}

pub trait SocketConnectionExt: Sized {
    fn connect<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>>(&self, address: &P, cancellable: Q) -> Result<(), Error>;

    fn connect_async<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, address: &P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketAddress> + Clone + 'static>(&self, address: &P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn get_local_address(&self) -> Result<SocketAddress, Error>;

    fn get_remote_address(&self) -> Result<SocketAddress, Error>;

    fn get_socket(&self) -> Option<Socket>;

    fn is_connected(&self) -> bool;
}

impl<O: IsA<SocketConnection> + IsA<glib::object::Object> + Clone + 'static> SocketConnectionExt for O {
    fn connect<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>>(&self, address: &P, cancellable: Q) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_connection_connect(self.to_glib_none().0, address.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_async<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, address: &P, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_async_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_connection_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            ffi::g_socket_connection_connect_async(self.to_glib_none().0, address.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketAddress> + Clone + 'static>(&self, address: &P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use fragile::Fragile;

        let address = address.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_async(
                 &address,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn get_local_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_connection_get_local_address(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_remote_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_connection_get_remote_address(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_socket(&self) -> Option<Socket> {
        unsafe {
            from_glib_none(ffi::g_socket_connection_get_socket(self.to_glib_none().0))
        }
    }

    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_connection_is_connected(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for SocketConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketConnection")
    }
}
