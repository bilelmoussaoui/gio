// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use AskPasswordFlags;
use MountOperationResult;
use PasswordSave;

glib_wrapper! {
    pub struct MountOperation(Object<gio_sys::GMountOperation, gio_sys::GMountOperationClass, MountOperationClass>);

    match fn {
        get_type => || gio_sys::g_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub fn new() -> MountOperation {
        unsafe { from_glib_full(gio_sys::g_mount_operation_new()) }
    }
}

impl Default for MountOperation {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MOUNT_OPERATION: Option<&MountOperation> = None;

pub trait MountOperationExt: 'static {
    fn get_anonymous(&self) -> bool;

    fn get_choice(&self) -> i32;

    fn get_domain(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_is_tcrypt_hidden_volume(&self) -> bool;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_is_tcrypt_system_volume(&self) -> bool;

    fn get_password(&self) -> Option<GString>;

    fn get_password_save(&self) -> PasswordSave;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_pim(&self) -> u32;

    fn get_username(&self) -> Option<GString>;

    fn reply(&self, result: MountOperationResult);

    fn set_anonymous(&self, anonymous: bool);

    fn set_choice(&self, choice: i32);

    fn set_domain(&self, domain: &str);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_is_tcrypt_hidden_volume(&self, hidden_volume: bool);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_is_tcrypt_system_volume(&self, system_volume: bool);

    fn set_password(&self, password: &str);

    fn set_password_save(&self, save: PasswordSave);

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_pim(&self, pim: u32);

    fn set_username(&self, username: &str);

    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_is_tcrypt_hidden_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_is_tcrypt_system_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_save_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_pim_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn get_anonymous(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_mount_operation_get_anonymous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_choice(&self) -> i32 {
        unsafe { gio_sys::g_mount_operation_get_choice(self.as_ref().to_glib_none().0) }
    }

    fn get_domain(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_mount_operation_get_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_is_tcrypt_hidden_volume(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_mount_operation_get_is_tcrypt_hidden_volume(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_is_tcrypt_system_volume(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_mount_operation_get_is_tcrypt_system_volume(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_password(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_mount_operation_get_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_password_save(&self) -> PasswordSave {
        unsafe {
            from_glib(gio_sys::g_mount_operation_get_password_save(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn get_pim(&self) -> u32 {
        unsafe { gio_sys::g_mount_operation_get_pim(self.as_ref().to_glib_none().0) }
    }

    fn get_username(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_mount_operation_get_username(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reply(&self, result: MountOperationResult) {
        unsafe {
            gio_sys::g_mount_operation_reply(self.as_ref().to_glib_none().0, result.to_glib());
        }
    }

    fn set_anonymous(&self, anonymous: bool) {
        unsafe {
            gio_sys::g_mount_operation_set_anonymous(
                self.as_ref().to_glib_none().0,
                anonymous.to_glib(),
            );
        }
    }

    fn set_choice(&self, choice: i32) {
        unsafe {
            gio_sys::g_mount_operation_set_choice(self.as_ref().to_glib_none().0, choice);
        }
    }

    fn set_domain(&self, domain: &str) {
        unsafe {
            gio_sys::g_mount_operation_set_domain(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_is_tcrypt_hidden_volume(&self, hidden_volume: bool) {
        unsafe {
            gio_sys::g_mount_operation_set_is_tcrypt_hidden_volume(
                self.as_ref().to_glib_none().0,
                hidden_volume.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_is_tcrypt_system_volume(&self, system_volume: bool) {
        unsafe {
            gio_sys::g_mount_operation_set_is_tcrypt_system_volume(
                self.as_ref().to_glib_none().0,
                system_volume.to_glib(),
            );
        }
    }

    fn set_password(&self, password: &str) {
        unsafe {
            gio_sys::g_mount_operation_set_password(
                self.as_ref().to_glib_none().0,
                password.to_glib_none().0,
            );
        }
    }

    fn set_password_save(&self, save: PasswordSave) {
        unsafe {
            gio_sys::g_mount_operation_set_password_save(
                self.as_ref().to_glib_none().0,
                save.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn set_pim(&self, pim: u32) {
        unsafe {
            gio_sys::g_mount_operation_set_pim(self.as_ref().to_glib_none().0, pim);
        }
    }

    fn set_username(&self, username: &str) {
        unsafe {
            gio_sys::g_mount_operation_set_username(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
            );
        }
    }

    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn aborted_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"aborted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    aborted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn ask_password_trampoline<
            P,
            F: Fn(&P, &str, &str, &str, AskPasswordFlags) + 'static,
        >(
            this: *mut gio_sys::GMountOperation,
            message: *mut libc::c_char,
            default_user: *mut libc::c_char,
            default_domain: *mut libc::c_char,
            flags: gio_sys::GAskPasswordFlags,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                &GString::from_glib_borrow(message),
                &GString::from_glib_borrow(default_user),
                &GString::from_glib_borrow(default_domain),
                from_glib(flags),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ask-password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ask_password_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reply_trampoline<P, F: Fn(&P, MountOperationResult) + 'static>(
            this: *mut gio_sys::GMountOperation,
            result: gio_sys::GMountOperationResult,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(result),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reply\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    reply_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype processes: *.Array TypeId { ns_id: 2, id: 3 }
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_unmount_progress_trampoline<
            P,
            F: Fn(&P, &str, i64, i64) + 'static,
        >(
            this: *mut gio_sys::GMountOperation,
            message: *mut libc::c_char,
            time_left: i64,
            bytes_left: i64,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(
                &MountOperation::from_glib_borrow(this).unsafe_cast_ref(),
                &GString::from_glib_borrow(message),
                time_left,
                bytes_left,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-unmount-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_unmount_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anonymous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anonymous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_anonymous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_choice_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::choice\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_choice_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_domain_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_domain_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_is_tcrypt_hidden_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_tcrypt_hidden_volume_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-tcrypt-hidden-volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_tcrypt_hidden_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_is_tcrypt_system_volume_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_tcrypt_system_volume_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-tcrypt-system-volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_tcrypt_system_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_password_save_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_save_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::password-save\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_save_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    fn connect_property_pim_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pim_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pim\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pim_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GMountOperation,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MountOperation>,
        {
            let f: &F = &*(f as *const F);
            f(&MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::username\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_username_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MountOperation")
    }
}
