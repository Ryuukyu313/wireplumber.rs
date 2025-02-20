// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Iterator};
use glib::{translate::*};

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Properties(Shared<ffi::WpProperties>);

    match fn {
        ref => |ptr| ffi::wp_properties_ref(ptr),
        unref => |ptr| ffi::wp_properties_unref(ptr),
        type_ => || ffi::wp_properties_get_type(),
    }
}

impl Properties {
    #[doc(alias = "wp_properties_new_empty")]
    #[doc(alias = "new_empty")]
    pub fn new() -> Properties {
        unsafe {
            from_glib_full(ffi::wp_properties_new_empty())
        }
    }

    #[doc(alias = "wp_properties_new_string")]
    pub fn new_string(str: &str) -> Properties {
        unsafe {
            from_glib_full(ffi::wp_properties_new_string(str.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_properties_add")]
    pub fn add(&self, props: &Properties) -> i32 {
        unsafe {
            ffi::wp_properties_add(self.to_glib_none().0, props.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_add_keys_array")]
    #[doc(alias = "add_keys_array")]
    pub fn add_keys(&self, props: &Properties, keys: &[&str]) -> i32 {
        unsafe {
            ffi::wp_properties_add_keys_array(self.to_glib_none().0, props.to_glib_none().0, keys.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_copy")]
#[must_use]
    pub fn copy(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_properties_copy(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_properties_ensure_unique_owner")]
#[must_use]
    pub fn ensure_unique_owner(self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_properties_ensure_unique_owner(self.into_glib_ptr()))
        }
    }

    #[doc(alias = "wp_properties_get")]
    pub fn get(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_get(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
    #[doc(alias = "wp_properties_get_count")]
    #[doc(alias = "get_count")]
    pub fn count(&self) -> u32 {
        unsafe {
            ffi::wp_properties_get_count(self.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_matches")]
    pub fn matches(&self, other: &Properties) -> bool {
        unsafe {
            from_glib(ffi::wp_properties_matches(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_properties_new_iterator")]
    pub fn new_iterator(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_properties_new_iterator(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_properties_set")]
    pub fn set(&self, key: &str, value: Option<&str>) -> i32 {
        unsafe {
            ffi::wp_properties_set(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_sort")]
    pub fn sort(&self) {
        unsafe {
            ffi::wp_properties_sort(self.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_properties_update")]
    pub fn update(&self, props: &Properties) -> i32 {
        unsafe {
            ffi::wp_properties_update(self.to_glib_none().0, props.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_update_keys_array")]
    #[doc(alias = "update_keys_array")]
    pub fn update_keys(&self, props: &Properties, keys: &[&str]) -> i32 {
        unsafe {
            ffi::wp_properties_update_keys_array(self.to_glib_none().0, props.to_glib_none().0, keys.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_properties_iterator_item_get_key")]
    pub fn iterator_item_get_key(item: &glib::Value) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_iterator_item_get_key(item.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_properties_iterator_item_get_value")]
    pub fn iterator_item_get_value(item: &glib::Value) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_iterator_item_get_value(item.to_glib_none().0))
        }
    }
}
