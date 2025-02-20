// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use bitflags::{bitflags};
use glib::{translate::*,value::FromValue,value::ToValue,StaticType,Type};

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
    #[doc(alias = "WpDbusFeatures")]
    pub struct DbusFeatures: u32 {
        #[doc(alias = "WP_DBUS_FEATURE_ENABLED")]
        const ENABLED = ffi::WP_DBUS_FEATURE_ENABLED as _;
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl IntoGlib for DbusFeatures {
    type GlibType = ffi::WpDbusFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpDbusFeatures {
        self.bits()
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl FromGlib<ffi::WpDbusFeatures> for DbusFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpDbusFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl StaticType for DbusFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_dbus_features_get_type()) }
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl glib::HasParamSpec for DbusFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl glib::value::ValueType for DbusFeatures {
    type Type = Self;
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
unsafe impl<'a> FromValue<'a> for DbusFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl ToValue for DbusFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl From<DbusFeatures> for glib::Value {
    #[inline]
    fn from(v: DbusFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpInitFlags")]
    pub struct InitFlags: u32 {
        #[doc(alias = "WP_INIT_PIPEWIRE")]
        const PIPEWIRE = ffi::WP_INIT_PIPEWIRE as _;
        #[doc(alias = "WP_INIT_SPA_TYPES")]
        const SPA_TYPES = ffi::WP_INIT_SPA_TYPES as _;
        #[doc(alias = "WP_INIT_SET_PW_LOG")]
        const SET_PW_LOG = ffi::WP_INIT_SET_PW_LOG as _;
        #[doc(alias = "WP_INIT_SET_GLIB_LOG")]
        const SET_GLIB_LOG = ffi::WP_INIT_SET_GLIB_LOG as _;
        #[doc(alias = "WP_INIT_ALL")]
        const ALL = ffi::WP_INIT_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for InitFlags {
    type GlibType = ffi::WpInitFlags;

    #[inline]
    fn into_glib(self) -> ffi::WpInitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInitFlags> for InitFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::WpInitFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InitFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_init_flags_get_type()) }
    }
}

impl glib::HasParamSpec for InitFlags {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for InitFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InitFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InitFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InitFlags> for glib::Value {
    #[inline]
    fn from(v: InitFlags) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpInterestMatch")]
    pub struct InterestMatch: u32 {
        #[doc(alias = "WP_INTEREST_MATCH_NONE")]
        const NONE = ffi::WP_INTEREST_MATCH_NONE as _;
        #[doc(alias = "WP_INTEREST_MATCH_GTYPE")]
        const GTYPE = ffi::WP_INTEREST_MATCH_GTYPE as _;
        #[doc(alias = "WP_INTEREST_MATCH_PW_GLOBAL_PROPERTIES")]
        const PW_GLOBAL_PROPERTIES = ffi::WP_INTEREST_MATCH_PW_GLOBAL_PROPERTIES as _;
        #[doc(alias = "WP_INTEREST_MATCH_PW_PROPERTIES")]
        const PW_PROPERTIES = ffi::WP_INTEREST_MATCH_PW_PROPERTIES as _;
        #[doc(alias = "WP_INTEREST_MATCH_G_PROPERTIES")]
        const G_PROPERTIES = ffi::WP_INTEREST_MATCH_G_PROPERTIES as _;
    }
}

#[doc(hidden)]
impl IntoGlib for InterestMatch {
    type GlibType = ffi::WpInterestMatch;

    #[inline]
    fn into_glib(self) -> ffi::WpInterestMatch {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInterestMatch> for InterestMatch {
    #[inline]
    unsafe fn from_glib(value: ffi::WpInterestMatch) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InterestMatch {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_interest_match_get_type()) }
    }
}

impl glib::HasParamSpec for InterestMatch {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for InterestMatch {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterestMatch {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InterestMatch {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InterestMatch> for glib::Value {
    #[inline]
    fn from(v: InterestMatch) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpInterestMatchFlags")]
    pub struct InterestMatchFlags: u32 {
        #[doc(alias = "WP_INTEREST_MATCH_FLAGS_NONE")]
        const NONE = ffi::WP_INTEREST_MATCH_FLAGS_NONE as _;
        #[doc(alias = "WP_INTEREST_MATCH_FLAGS_CHECK_ALL")]
        const CHECK_ALL = ffi::WP_INTEREST_MATCH_FLAGS_CHECK_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for InterestMatchFlags {
    type GlibType = ffi::WpInterestMatchFlags;

    #[inline]
    fn into_glib(self) -> ffi::WpInterestMatchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpInterestMatchFlags> for InterestMatchFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::WpInterestMatchFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InterestMatchFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_interest_match_flags_get_type()) }
    }
}

impl glib::HasParamSpec for InterestMatchFlags {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for InterestMatchFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterestMatchFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InterestMatchFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InterestMatchFlags> for glib::Value {
    #[inline]
    fn from(v: InterestMatchFlags) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpLinkFeatures")]
    pub struct LinkFeatures: u32 {
        #[cfg(any(feature = "v0_4_11", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
        #[doc(alias = "WP_LINK_FEATURE_ESTABLISHED")]
        const ESTABLISHED = ffi::WP_LINK_FEATURE_ESTABLISHED as _;

        const PROXY_FEATURES = ProxyFeatures::ALL.bits();
    }
}

#[doc(hidden)]
impl IntoGlib for LinkFeatures {
    type GlibType = ffi::WpLinkFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpLinkFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpLinkFeatures> for LinkFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpLinkFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl glib::HasParamSpec for LinkFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for LinkFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LinkFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for LinkFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<LinkFeatures> for glib::Value {
    #[inline]
    fn from(v: LinkFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpLookupDirs")]
    pub struct LookupDirs: u32 {
        #[doc(alias = "WP_LOOKUP_DIR_ENV_CONFIG")]
        const ENV_CONFIG = ffi::WP_LOOKUP_DIR_ENV_CONFIG as _;
        #[doc(alias = "WP_LOOKUP_DIR_ENV_DATA")]
        const ENV_DATA = ffi::WP_LOOKUP_DIR_ENV_DATA as _;
        #[doc(alias = "WP_LOOKUP_DIR_XDG_CONFIG_HOME")]
        const XDG_CONFIG_HOME = ffi::WP_LOOKUP_DIR_XDG_CONFIG_HOME as _;
        #[doc(alias = "WP_LOOKUP_DIR_ETC")]
        const ETC = ffi::WP_LOOKUP_DIR_ETC as _;
        #[doc(alias = "WP_LOOKUP_DIR_PREFIX_SHARE")]
        const PREFIX_SHARE = ffi::WP_LOOKUP_DIR_PREFIX_SHARE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for LookupDirs {
    type GlibType = ffi::WpLookupDirs;

    #[inline]
    fn into_glib(self) -> ffi::WpLookupDirs {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpLookupDirs> for LookupDirs {
    #[inline]
    unsafe fn from_glib(value: ffi::WpLookupDirs) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for LookupDirs {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_lookup_dirs_get_type()) }
    }
}

impl glib::HasParamSpec for LookupDirs {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for LookupDirs {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LookupDirs {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for LookupDirs {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<LookupDirs> for glib::Value {
    #[inline]
    fn from(v: LookupDirs) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpMetadataFeatures")]
    pub struct MetadataFeatures: u32 {
        #[doc(alias = "WP_METADATA_FEATURE_DATA")]
        const DATA = ffi::WP_METADATA_FEATURE_DATA as _;

        const PROXY_FEATURE_BOUND = ProxyFeatures::PROXY_FEATURE_BOUND.bits();
    }
}

#[doc(hidden)]
impl IntoGlib for MetadataFeatures {
    type GlibType = ffi::WpMetadataFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpMetadataFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpMetadataFeatures> for MetadataFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpMetadataFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for MetadataFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_metadata_features_get_type()) }
    }
}

impl glib::HasParamSpec for MetadataFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for MetadataFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MetadataFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for MetadataFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<MetadataFeatures> for glib::Value {
    #[inline]
    fn from(v: MetadataFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpNodeFeatures")]
    pub struct NodeFeatures: u32 {
        #[doc(alias = "WP_NODE_FEATURE_PORTS")]
        const PORTS = ffi::WP_NODE_FEATURE_PORTS as _;

        const PROXY_FEATURES = ProxyFeatures::ALL.bits();
    }
}

#[doc(hidden)]
impl IntoGlib for NodeFeatures {
    type GlibType = ffi::WpNodeFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpNodeFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpNodeFeatures> for NodeFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpNodeFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for NodeFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_node_features_get_type()) }
    }
}

impl glib::HasParamSpec for NodeFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for NodeFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for NodeFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for NodeFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<NodeFeatures> for glib::Value {
    #[inline]
    fn from(v: NodeFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpPluginFeatures")]
    pub struct PluginFeatures: u32 {
        #[doc(alias = "WP_PLUGIN_FEATURE_ENABLED")]
        const ENABLED = ffi::WP_PLUGIN_FEATURE_ENABLED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for PluginFeatures {
    type GlibType = ffi::WpPluginFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpPluginFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpPluginFeatures> for PluginFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpPluginFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PluginFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_plugin_features_get_type()) }
    }
}

impl glib::HasParamSpec for PluginFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for PluginFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PluginFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PluginFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PluginFeatures> for glib::Value {
    #[inline]
    fn from(v: PluginFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpProxyFeatures")]
    pub struct ProxyFeatures: u32 {
        #[doc(alias = "WP_PROXY_FEATURE_BOUND")]
        const PROXY_FEATURE_BOUND = ffi::WP_PROXY_FEATURE_BOUND as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_INFO")]
        const PIPEWIRE_OBJECT_FEATURE_INFO = ffi::WP_PIPEWIRE_OBJECT_FEATURE_INFO as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG as _;
        #[doc(alias = "WP_PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE")]
        const PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE = ffi::WP_PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE as _;

        #[doc(alias = "WP_PROXY_FEATURE_CUSTOM_START")]
        const PROXY_FEATURE_CUSTOM_START = 1 << 16;
        const PROXY_FEATURE_CUSTOM = 0xff00;
    }
}

#[doc(hidden)]
impl IntoGlib for ProxyFeatures {
    type GlibType = ffi::WpProxyFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpProxyFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpProxyFeatures> for ProxyFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpProxyFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ProxyFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_proxy_features_get_type()) }
    }
}

impl glib::HasParamSpec for ProxyFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for ProxyFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ProxyFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ProxyFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ProxyFeatures> for glib::Value {
    #[inline]
    fn from(v: ProxyFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpSessionItemFeatures")]
    pub struct SessionItemFeatures: u32 {
        #[doc(alias = "WP_SESSION_ITEM_FEATURE_ACTIVE")]
        const ACTIVE = ffi::WP_SESSION_ITEM_FEATURE_ACTIVE as _;
        #[doc(alias = "WP_SESSION_ITEM_FEATURE_EXPORTED")]
        const EXPORTED = ffi::WP_SESSION_ITEM_FEATURE_EXPORTED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for SessionItemFeatures {
    type GlibType = ffi::WpSessionItemFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpSessionItemFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpSessionItemFeatures> for SessionItemFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpSessionItemFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SessionItemFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_session_item_features_get_type()) }
    }
}

impl glib::HasParamSpec for SessionItemFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for SessionItemFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SessionItemFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SessionItemFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SessionItemFeatures> for glib::Value {
    #[inline]
    fn from(v: SessionItemFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "WpSpaDeviceFeatures")]
    pub struct SpaDeviceFeatures: u32 {
        #[doc(alias = "WP_SPA_DEVICE_FEATURE_ENABLED")]
        const ENABLED = ffi::WP_SPA_DEVICE_FEATURE_ENABLED as _;

        const PROXY_FEATURE_BOUND = ProxyFeatures::PROXY_FEATURE_BOUND.bits();
    }
}

#[doc(hidden)]
impl IntoGlib for SpaDeviceFeatures {
    type GlibType = ffi::WpSpaDeviceFeatures;

    #[inline]
    fn into_glib(self) -> ffi::WpSpaDeviceFeatures {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpSpaDeviceFeatures> for SpaDeviceFeatures {
    #[inline]
    unsafe fn from_glib(value: ffi::WpSpaDeviceFeatures) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for SpaDeviceFeatures {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_spa_device_features_get_type()) }
    }
}

impl glib::HasParamSpec for SpaDeviceFeatures {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name| Self::ParamSpec::builder(name)
                }
}

impl glib::value::ValueType for SpaDeviceFeatures {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SpaDeviceFeatures {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for SpaDeviceFeatures {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SpaDeviceFeatures> for glib::Value {
    #[inline]
    fn from(v: SpaDeviceFeatures) -> Self {
        ToValue::to_value(&v)
    }
}

