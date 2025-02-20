// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,Object,PipewireObject,Properties,Proxy};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "WpImplNode")]
    pub struct ImplNode(Object<ffi::WpImplNode, ffi::WpImplNodeClass>) @extends Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_impl_node_get_type(),
    }
}

impl ImplNode {
    #[doc(alias = "wp_impl_node_new_from_pw_factory")]
    #[doc(alias = "new_from_pw_factory")]
    pub fn from_pw_factory(core: &Core, factory_name: &str, properties: Option<Properties>) -> Option<ImplNode> {
        unsafe {
            from_glib_full(ffi::wp_impl_node_new_from_pw_factory(core.to_glib_none().0, factory_name.to_glib_none().0, properties.into_glib_ptr()))
        }
    }

    //#[doc(alias = "wp_impl_node_new_wrap")]
    //pub fn new_wrap(core: &Core, node: /*Unimplemented*/Option<Basic: Pointer>) -> ImplNode {
    //    unsafe { TODO: call ffi:wp_impl_node_new_wrap() }
    //}

    //#[doc(alias = "pw-impl-node")]
    //pub fn pw_impl_node(&self) -> /*Unimplemented*/Basic: Pointer {
    //    glib::ObjectExt::property(self, "pw-impl-node")
    //}
}
