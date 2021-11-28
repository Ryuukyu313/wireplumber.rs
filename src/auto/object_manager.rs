// Generated by gir (https://github.com/gtk-rs/gir @ c148542)
// from /nix/store/5njndanmnlsxkgl5yyrmkfs8a4909cxa-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// from /nix/store/h9500ckrbcbv20q4qsj87q3rnp01fmxx-wireplumber.gir/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::Iterator;
use crate::ObjectFeatures;
use crate::ObjectInterest;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ObjectManager(Object<ffi::WpObjectManager, ffi::WpObjectManagerClass>);

    match fn {
        type_ => || ffi::wp_object_manager_get_type(),
    }
}

impl ObjectManager {
    #[doc(alias = "wp_object_manager_new")]
    pub fn new() -> ObjectManager {
        unsafe {
            from_glib_full(ffi::wp_object_manager_new())
        }
    }

    //#[doc(alias = "wp_object_manager_add_interest")]
    //pub fn add_interest(&self, gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:wp_object_manager_add_interest() }
    //}

    #[doc(alias = "wp_object_manager_add_interest_full")]
    pub fn add_interest_full(&self, interest: &ObjectInterest) {
        unsafe {
            ffi::wp_object_manager_add_interest_full(self.to_glib_none().0, interest.to_glib_full());
        }
    }

    #[doc(alias = "wp_object_manager_get_n_objects")]
    #[doc(alias = "get_n_objects")]
    pub fn n_objects(&self) -> u32 {
        unsafe {
            ffi::wp_object_manager_get_n_objects(self.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_object_manager_is_installed")]
    pub fn is_installed(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_object_manager_is_installed(self.to_glib_none().0))
        }
    }

    //#[doc(alias = "wp_object_manager_lookup")]
    //pub fn lookup(&self, gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> /*Ignored*/Option<glib::Object> {
    //    unsafe { TODO: call ffi:wp_object_manager_lookup() }
    //}

    //#[doc(alias = "wp_object_manager_lookup_full")]
    //pub fn lookup_full(&self, interest: &ObjectInterest) -> /*Ignored*/Option<glib::Object> {
    //    unsafe { TODO: call ffi:wp_object_manager_lookup_full() }
    //}

    //#[doc(alias = "wp_object_manager_new_filtered_iterator")]
    //pub fn new_filtered_iterator(&self, gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_object_manager_new_filtered_iterator() }
    //}

    #[doc(alias = "wp_object_manager_new_filtered_iterator_full")]
    pub fn new_filtered_iterator_full(&self, interest: &ObjectInterest) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_object_manager_new_filtered_iterator_full(self.to_glib_none().0, interest.to_glib_full()))
        }
    }

    #[doc(alias = "wp_object_manager_new_iterator")]
    pub fn new_iterator(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_object_manager_new_iterator(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_object_manager_request_object_features")]
    pub fn request_object_features(&self, object_type: glib::types::Type, wanted_features: ObjectFeatures) {
        unsafe {
            ffi::wp_object_manager_request_object_features(self.to_glib_none().0, object_type.into_glib(), wanted_features);
        }
    }

    pub fn core(&self) -> Option<Core> {
        unsafe {
            let mut value = glib::Value::from_type(<Core as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"core\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `core` getter")
        }
    }

    #[doc(alias = "installed")]
    pub fn connect_installed<F: Fn(&ObjectManager) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn installed_trampoline<F: Fn(&ObjectManager) + 'static>(this: *mut ffi::WpObjectManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"installed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(installed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    //#[doc(alias = "object-added")]
    //pub fn connect_object_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored object: GObject.Object
    //}

    //#[doc(alias = "object-removed")]
    //pub fn connect_object_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored object: GObject.Object
    //}

    #[doc(alias = "objects-changed")]
    pub fn connect_objects_changed<F: Fn(&ObjectManager) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn objects_changed_trampoline<F: Fn(&ObjectManager) + 'static>(this: *mut ffi::WpObjectManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"objects-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(objects_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "core")]
    pub fn connect_core_notify<F: Fn(&ObjectManager) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_core_trampoline<F: Fn(&ObjectManager) + 'static>(this: *mut ffi::WpObjectManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::core\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_core_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for ObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ObjectManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ObjectManager")
    }
}
