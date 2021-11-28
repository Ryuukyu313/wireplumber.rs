// Generated by gir (https://github.com/gtk-rs/gir @ unstable-2021-11-21)
// from /nix/store/1gli05cgwhqdc8papdgi4y98l2xzssmq-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/5njndanmnlsxkgl5yyrmkfs8a4909cxa-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::GlobalProxy;
use crate::Metadata;
use crate::Object;
use crate::Properties;
use crate::Proxy;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WpImplMetadata")]
    pub struct ImplMetadata(Object<ffi::WpImplMetadata, ffi::WpImplMetadataClass>) @extends Metadata, GlobalProxy, Proxy, Object;

    match fn {
        type_ => || ffi::wp_impl_metadata_get_type(),
    }
}

impl ImplMetadata {
    #[doc(alias = "wp_impl_metadata_new")]
    pub fn new(core: &Core) -> ImplMetadata {
        unsafe {
            from_glib_full(ffi::wp_impl_metadata_new(core.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_4_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_3")))]
    #[doc(alias = "wp_impl_metadata_new_full")]
    pub fn new_full(core: &Core, name: Option<&str>, properties: Option<&Properties>) -> ImplMetadata {
        unsafe {
            from_glib_full(ffi::wp_impl_metadata_new_full(core.to_glib_none().0, name.to_glib_none().0, properties.to_glib_full()))
        }
    }

    pub fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "name").unwrap().get().unwrap()
    }

    pub fn properties(&self) -> Option<Properties> {
        glib::ObjectExt::property(self, "properties").unwrap().get().unwrap()
    }
}

impl fmt::Display for ImplMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ImplMetadata")
    }
}
