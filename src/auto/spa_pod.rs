// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Iterator};
use glib::{translate::*};
use std::{mem,ptr};

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct SpaPod(Shared<ffi::WpSpaPod>);

    match fn {
        ref => |ptr| ffi::wp_spa_pod_ref(ptr),
        unref => |ptr| ffi::wp_spa_pod_unref(ptr),
        type_ => || ffi::wp_spa_pod_get_type(),
    }
}

impl SpaPod {
    #[doc(alias = "wp_spa_pod_new_boolean")]
    pub fn new_boolean(value: bool) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_boolean(value.into_glib()))
        }
    }

    #[doc(alias = "wp_spa_pod_new_double")]
    pub fn new_double(value: f64) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_double(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_fd")]
    pub fn new_fd(value: i64) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_fd(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_float")]
    pub fn new_float(value: f32) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_float(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_fraction")]
    pub fn new_fraction(num: u32, denom: u32) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_fraction(num, denom))
        }
    }

    #[doc(alias = "wp_spa_pod_new_id")]
    pub fn new_id(value: u32) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_id(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_int")]
    pub fn new_int(value: i32) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_int(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_long")]
    pub fn new_long(value: i64) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_long(value))
        }
    }

    #[doc(alias = "wp_spa_pod_new_none")]
    pub fn new_none() -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_none())
        }
    }

    #[doc(alias = "wp_spa_pod_new_rectangle")]
    pub fn new_rectangle(width: u32, height: u32) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_rectangle(width, height))
        }
    }

    #[doc(alias = "wp_spa_pod_new_string")]
    pub fn new_string(value: &str) -> SpaPod {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_string(value.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_copy")]
#[must_use]
    pub fn copy(&self) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_copy(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_ensure_unique_owner")]
#[must_use]
    pub fn ensure_unique_owner(self) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_ensure_unique_owner(self.into_glib_ptr()))
        }
    }

    #[doc(alias = "wp_spa_pod_equal")]
     fn equal(&self, pod: &SpaPod) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_equal(self.to_glib_none().0, pod.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_filter")]
#[must_use]
    pub fn filter(&self, filter: &SpaPod) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_filter(self.to_glib_none().0, filter.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_fixate")]
    pub fn fixate(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_fixate(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_get_array_child")]
    #[doc(alias = "get_array_child")]
#[must_use]
    pub fn array_child(&self) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_get_array_child(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_get_boolean")]
    #[doc(alias = "get_boolean")]
    pub fn boolean(&self) -> Option<bool> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_boolean(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(from_glib(value.assume_init())) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_choice_child")]
    #[doc(alias = "get_choice_child")]
#[must_use]
    pub fn choice_child(&self) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_get_choice_child(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_get_control")]
    #[doc(alias = "get_control")]
    pub fn control(&self) -> Option<(u32, glib::GString, SpaPod)> {
        unsafe {
            let mut offset = mem::MaybeUninit::uninit();
            let mut ctl_type = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::wp_spa_pod_get_control(self.to_glib_none().0, offset.as_mut_ptr(), &mut ctl_type, &mut value));
            if ret { Some((offset.assume_init(), from_glib_none(ctl_type), from_glib_full(value))) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_double")]
    #[doc(alias = "get_double")]
    pub fn double(&self) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_double(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_fd")]
    #[doc(alias = "get_fd")]
    pub fn fd(&self) -> Option<i64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_fd(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_float")]
    #[doc(alias = "get_float")]
    pub fn float(&self) -> Option<f32> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_float(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_fraction")]
    #[doc(alias = "get_fraction")]
    pub fn fraction(&self) -> Option<(u32, u32)> {
        unsafe {
            let mut num = mem::MaybeUninit::uninit();
            let mut denom = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_fraction(self.to_glib_none().0, num.as_mut_ptr(), denom.as_mut_ptr()));
            if ret { Some((num.assume_init(), denom.assume_init())) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> Option<u32> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_id(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_int")]
    #[doc(alias = "get_int")]
    pub fn int(&self) -> Option<i32> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_int(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_long")]
    #[doc(alias = "get_long")]
    pub fn long(&self) -> Option<i64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_long(self.to_glib_none().0, value.as_mut_ptr()));
            if ret { Some(value.assume_init()) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_property")]
    #[doc(alias = "get_property")]
    pub fn property(&self) -> Option<(glib::GString, SpaPod)> {
        unsafe {
            let mut key = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::wp_spa_pod_get_property(self.to_glib_none().0, &mut key, &mut value));
            if ret { Some((from_glib_none(key), from_glib_none(value))) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_rectangle")]
    #[doc(alias = "get_rectangle")]
    pub fn rectangle(&self) -> Option<(u32, u32)> {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_pod_get_rectangle(self.to_glib_none().0, width.as_mut_ptr(), height.as_mut_ptr()));
            if ret { Some((width.assume_init(), height.assume_init())) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = ptr::null();
            let ret = from_glib(ffi::wp_spa_pod_get_string(self.to_glib_none().0, &mut value));
            if ret { Some(from_glib_none(value)) } else { None }
        }
    }

    #[doc(alias = "wp_spa_pod_is_array")]
    pub fn is_array(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_array(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_boolean")]
    pub fn is_boolean(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_boolean(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_bytes")]
    pub fn is_bytes(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_bytes(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_choice")]
    pub fn is_choice(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_choice(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_control")]
    pub fn is_control(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_control(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_double")]
    pub fn is_double(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_double(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_fd")]
    pub fn is_fd(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_fd(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_float")]
    pub fn is_float(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_float(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_fraction")]
    pub fn is_fraction(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_fraction(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_id")]
    pub fn is_id(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_id(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_int")]
    pub fn is_int(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_int(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_long")]
    pub fn is_long(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_long(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_none")]
    pub fn is_none(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_none(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_object")]
    pub fn is_object(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_object(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_pointer")]
    pub fn is_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_pointer(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_property")]
    pub fn is_property(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_property(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_rectangle")]
    pub fn is_rectangle(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_rectangle(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_sequence")]
    pub fn is_sequence(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_sequence(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_string")]
    pub fn is_string(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_string(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_struct")]
    pub fn is_struct(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_struct(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_is_unique_owner")]
    pub fn is_unique_owner(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_is_unique_owner(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_new_iterator")]
    pub fn new_iterator(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_new_iterator(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_set_boolean")]
    pub fn set_boolean(&self, value: bool) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_boolean(self.to_glib_none().0, value.into_glib()))
        }
    }

    #[doc(alias = "wp_spa_pod_set_double")]
    pub fn set_double(&self, value: f64) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_double(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_fd")]
    pub fn set_fd(&self, value: i64) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_fd(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_float")]
    pub fn set_float(&self, value: f32) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_float(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_fraction")]
    pub fn set_fraction(&self, num: u32, denom: u32) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_fraction(self.to_glib_none().0, num, denom))
        }
    }

    #[doc(alias = "wp_spa_pod_set_id")]
    pub fn set_id(&self, value: u32) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_id(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_int")]
    pub fn set_int(&self, value: i32) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_int(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_long")]
    pub fn set_long(&self, value: i64) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_long(self.to_glib_none().0, value))
        }
    }

    #[doc(alias = "wp_spa_pod_set_pod")]
    pub fn set_pod(&self, pod: &SpaPod) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_pod(self.to_glib_none().0, pod.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_set_rectangle")]
    pub fn set_rectangle(&self, width: u32, height: u32) -> bool {
        unsafe {
            from_glib(ffi::wp_spa_pod_set_rectangle(self.to_glib_none().0, width, height))
        }
    }
}

impl PartialEq for SpaPod {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for SpaPod {}
