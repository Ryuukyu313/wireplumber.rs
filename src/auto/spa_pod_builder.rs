// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::SpaPod;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpaPodBuilder(Shared<ffi::WpSpaPodBuilder>);

    match fn {
        ref => |ptr| ffi::wp_spa_pod_builder_ref(ptr),
        unref => |ptr| ffi::wp_spa_pod_builder_unref(ptr),
        type_ => || ffi::wp_spa_pod_builder_get_type(),
    }
}

impl SpaPodBuilder {
    #[doc(alias = "wp_spa_pod_builder_new_array")]
    pub fn new_array() -> SpaPodBuilder {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_new_array())
        }
    }

    #[doc(alias = "wp_spa_pod_builder_new_choice")]
    pub fn new_choice(choice_type: &str) -> SpaPodBuilder {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_new_choice(choice_type.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_builder_new_object")]
    pub fn new_object(type_name: &str, id_name: &str) -> SpaPodBuilder {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_new_object(type_name.to_glib_none().0, id_name.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_pod_builder_new_sequence")]
    pub fn new_sequence(unit: u32) -> SpaPodBuilder {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_new_sequence(unit))
        }
    }

    #[doc(alias = "wp_spa_pod_builder_new_struct")]
    pub fn new_struct() -> SpaPodBuilder {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_new_struct())
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_boolean")]
    pub fn add_boolean(&self, value: bool) {
        unsafe {
            ffi::wp_spa_pod_builder_add_boolean(self.to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_control")]
    pub fn add_control(&self, offset: u32, ctl_type: &str) {
        unsafe {
            ffi::wp_spa_pod_builder_add_control(self.to_glib_none().0, offset, ctl_type.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_double")]
    pub fn add_double(&self, value: f64) {
        unsafe {
            ffi::wp_spa_pod_builder_add_double(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_fd")]
    pub fn add_fd(&self, value: i64) {
        unsafe {
            ffi::wp_spa_pod_builder_add_fd(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_float")]
    pub fn add_float(&self, value: f32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_float(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_fraction")]
    pub fn add_fraction(&self, num: u32, denom: u32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_fraction(self.to_glib_none().0, num, denom);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_id")]
    pub fn add_id(&self, value: u32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_id(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_int")]
    pub fn add_int(&self, value: i32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_int(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_long")]
    pub fn add_long(&self, value: i64) {
        unsafe {
            ffi::wp_spa_pod_builder_add_long(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_none")]
    pub fn add_none(&self) {
        unsafe {
            ffi::wp_spa_pod_builder_add_none(self.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_pod")]
    pub fn add_pod(&self, pod: &SpaPod) {
        unsafe {
            ffi::wp_spa_pod_builder_add_pod(self.to_glib_none().0, pod.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_property")]
    pub fn add_property(&self, key: &str) {
        unsafe {
            ffi::wp_spa_pod_builder_add_property(self.to_glib_none().0, key.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_property_id")]
    pub fn add_property_id(&self, id: u32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_property_id(self.to_glib_none().0, id);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_rectangle")]
    pub fn add_rectangle(&self, width: u32, height: u32) {
        unsafe {
            ffi::wp_spa_pod_builder_add_rectangle(self.to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_add_string")]
    pub fn add_string(&self, value: &str) {
        unsafe {
            ffi::wp_spa_pod_builder_add_string(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_spa_pod_builder_end")]
    pub fn end(&self) -> Option<SpaPod> {
        unsafe {
            from_glib_full(ffi::wp_spa_pod_builder_end(self.to_glib_none().0))
        }
    }
}
