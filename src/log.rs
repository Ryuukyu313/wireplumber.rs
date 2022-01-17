use glib::{Object, Type, GString, GStringBuilder, LogLevelFlags, translate::{IntoGlib, ToGlibPtr, from_glib}, error::ErrorDomain, ObjectExt};
use libspa_sys::spa_log;
use std::{env, ptr, fmt::{self, Write}};

pub struct Log(());

impl Log {
	#[doc(alias = "WP_LOG_LEVEL_TRACE")]
	pub const LEVEL_TRACE: i32 = ffi::WP_LOG_LEVEL_TRACE;

	/*#[doc(alias = "WP_OBJECT_FORMAT")]
	pub const OBJECT_FORMAT: CStr = ffi::WP_OBJECT_FORMAT;*/

	pub fn domain() -> &'static str {
		crate::LibraryErrorEnum::domain().as_str()
	}

	#[doc(alias = "wp_log_level_is_enabled")]
	pub fn level_is_enabled(flags: LogLevelFlags) -> bool {
		unsafe {
			from_glib(ffi::wp_log_level_is_enabled(flags.into_glib()))
		}
	}

	#[doc(alias = "wp_log_set_level")]
	pub fn set_level(level: &str) {
		env::set_var("WIREPLUMBER_DEBUG", level); // XXX: this doesn't seem to work properly otherwise?
		unsafe {
			ffi::wp_log_set_level(level.to_glib_none().0)
		}
	}

	pub fn set_default_level(level: &str) {
		if env::var_os("WIREPLUMBER_DEBUG").is_none() {
			Self::set_level(level)
		}
	}

	#[doc(alias = "wp_spa_log_get_instance")]
	pub fn spa_log() -> *mut spa_log {
		unsafe {
			ffi::wp_spa_log_get_instance()
		}
	}

	#[doc(alias = "wp_log_structured_standard")]
	pub fn log_string<M: Into<GString>>(log_level: LogLevelFlags, context: StructuredLogContext, message: M) {
		unsafe {
			// XXX: so much allocation, it burns...
			let domain = context.domain.to_glib_none();
			let file = context.file.to_glib_none();
			let line = context.line.as_ref().map(ToString::to_string);
			let line = line.to_glib_none();
			let function = context.function.to_glib_none();
			let obj_type = context.object_type.unwrap_or(match context.object {
				Some(obj) => obj.type_(),
				None => Type::INVALID,
			});
			let obj = context.object.as_ref().map(|obj| (*obj).to_glib_none());
			let message = message.into();
			ffi::wp_log_structured_standard(
				domain.0, log_level.into_glib(),
				file.0, line.0, function.0,
				obj_type.into_glib(), obj.as_ref().map(|o| o.0).unwrap_or(ptr::null()) as *mut _,
				b"%s\0".as_ptr() as *const _, message.as_ptr()
			)
		}
	}

	pub fn log_args<O: AsRef<Object>>(log_level: LogLevelFlags, context: StructuredLogContext<O>, args: fmt::Arguments) {
		let mut message = GStringBuilder::default();
		let _ = write!(message, "{}", args);
		Self::log_string(log_level, context.to_object(), message.into_string())
	}

	#[doc(alias = "wp_log_writer_default")]
	pub unsafe fn writer_default(log_level: LogLevelFlags, fields: &[glib::ffi::GLogField], user_data: glib::ffi::gpointer) -> glib::ffi::GLogWriterOutput {
		ffi::wp_log_writer_default(log_level.into_glib(), fields.as_ptr(), fields.len(), user_data)
	}
	// TODO: wp_log_writer_default, wp_log_structured_standard
}

#[derive(Debug, Clone, Default)]
pub struct StructuredLogContext<'a, O = Object> {
	pub domain: Option<&'a str>,
	pub file: Option<&'static str>,
	pub line: Option<u32>,
	pub function: Option<&'a str>,
	pub object: Option<&'a O>,
	pub object_type: Option<Type>,
}

impl<'a, O> StructuredLogContext<'a, O> {
	fn to_object(&self) -> StructuredLogContext<'a, Object> where O: AsRef<Object> {
		StructuredLogContext {
			domain: self.domain,
			file: self.file,
			line: self.line,
			function: self.function,
			object: self.object.map(|o| o.as_ref()),
			object_type: self.object_type,
		}
	}
}

macro_rules! define_log_variant {
	({$dollar:tt} $($name:ident($int_name:ident) = $level:ident,)*) => {
		$(
			#[macro_export]
			macro_rules! $name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::$name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log! { self:$dollar self, domain: $dollar log_domain, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log! { self:$dollar self, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				($dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log! { $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				(domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log! { domain: $dollar log_domain, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
			}

			#[allow(unused_macros)]
			macro_rules! $int_name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$int_name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar($dollar args:tt)*) => {
					$name! { self:$dollar self, domain: $dollar crate::Log::domain(), $dollar($dollar args)* }
				};
				($dollar($dollar args:tt)*) => {
					$name! { domain: $dollar crate::Log::domain(), $dollar($dollar args)* }
				};
			}
		)*
	};
}

define_log_variant! { {$}
	critical(wp_critical) = LEVEL_CRITICAL,
	warning(wp_warning) = LEVEL_WARNING,
	message(wp_message) = LEVEL_MESSAGE,
	info(wp_info) = LEVEL_INFO,
	debug(wp_debug) = LEVEL_DEBUG,
	trace(wp_trace) = LEVEL_TRACE,
}

#[macro_export]
macro_rules! log {
	(self: $self:expr, domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::_log_inner! { $log_domain.into(), $log_level, None, Some($self), $format; $($args)* }
	};
	(self: $self:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::_log_inner! { None, $log_level, None, Some($self), $format; $($args)* }
	};
	(domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::_log_inner! { $log_domain.into(), $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
	($log_level:expr, $format:literal $($args:tt)*) => {
		$crate::_log_inner! { None, $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _log_inner {
	($log_domain:expr, $log_level:expr, $obj_type:expr, $obj:expr, $fmt:expr; $($args:tt)*) => {
		{
			let log_level = $log_level;
			if $crate::Log::level_is_enabled(log_level) {
				let object = $obj;
				let log_context = $crate::StructuredLogContext {
					domain: $log_domain,
					file: Some(file!()),
					line: Some(line!()),
					function: None, // TODO: this is possible: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
					object_type: None, // TODO: consider getting the static type from object
					object: object,
				};
				$crate::Log::log_args(log_level, log_context, format_args!($fmt $($args)*))
			}
		}
	};
}
