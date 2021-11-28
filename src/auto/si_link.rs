// Generated by gir (https://github.com/gtk-rs/gir @ unstable-2021-11-21)
// from /nix/store/1gli05cgwhqdc8papdgi4y98l2xzssmq-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/5njndanmnlsxkgl5yyrmkfs8a4909cxa-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Object;
use crate::SessionItem;
use crate::SiLinkable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WpSiLink")]
    pub struct SiLink(Interface<ffi::WpSiLink, ffi::WpSiLinkInterface>) @requires SessionItem, Object;

    match fn {
        type_ => || ffi::wp_si_link_get_type(),
    }
}

impl SiLink {
        pub const NONE: Option<&'static SiLink> = None;
    
}

pub trait SiLinkExt: 'static {
    #[doc(alias = "wp_si_link_get_in_item")]
    #[doc(alias = "get_in_item")]
    fn in_item(&self) -> Option<SiLinkable>;

    #[doc(alias = "wp_si_link_get_out_item")]
    #[doc(alias = "get_out_item")]
    fn out_item(&self) -> Option<SiLinkable>;

    #[doc(alias = "wp_si_link_get_registration_info")]
    #[doc(alias = "get_registration_info")]
    fn registration_info(&self) -> Option<glib::Variant>;

    #[doc(alias = "link-properties-changed")]
    fn connect_link_properties_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SiLink>> SiLinkExt for O {
    fn in_item(&self) -> Option<SiLinkable> {
        unsafe {
            from_glib_none(ffi::wp_si_link_get_in_item(self.as_ref().to_glib_none().0))
        }
    }

    fn out_item(&self) -> Option<SiLinkable> {
        unsafe {
            from_glib_none(ffi::wp_si_link_get_out_item(self.as_ref().to_glib_none().0))
        }
    }

    fn registration_info(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::wp_si_link_get_registration_info(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_link_properties_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn link_properties_changed_trampoline<P: IsA<SiLink>, F: Fn(&P) + 'static>(this: *mut ffi::WpSiLink, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SiLink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"link-properties-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(link_properties_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SiLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SiLink")
    }
}
