// crates.io
use serde::{Serializer, de::Error as DeserializeError};
// self
use crate::_prelude::*;

macro_rules! impl_const_str {
	($( $name:tt => $val:expr ),* $(,)?) => {
		$(
			paste::paste! {
				pub type [<Const $name>] = _ConstStr<[<_ $name>]>;
				#[derive(Clone, Debug, Default)]
				pub struct [<_ $name>];
				impl _ConstStrT for [<_ $name>] {
					const VAL: &'static str = $val;
				}
			}
		)*
	};
}
pub(crate) use impl_const_str;

macro_rules! _define_enum {
	// Standard variant without default.
	($name:ident { $($var:ident),* $(,)? }) => {
		#[derive(Clone, Debug, PartialEq, Eq)]
		pub enum $name {
			$($var),*
		}
	};
	// Variant with default implementation.
	($name:ident { $($var:ident),* } with_default $default_var:ident) => {
		#[derive(Clone, Debug, PartialEq, Eq)]
		pub enum $name {
			$($var),*
		}
		impl Default for $name {
			fn default() -> Self {
				Self::$default_var
			}
		}
	};
}
pub(crate) use _define_enum;

macro_rules! _parse_enum_with_default {
	// Pattern: `#[default]` only variant.
	(
		$name:ident {
			#[default]
			$default_var:ident => $default_val:expr $(,)?
		}
		=> $callback:ident
	) => {
		$callback!($name { $default_var } { $default_var => $default_val } with_default $default_var);
	};
	// Pattern: `#[default]` first, then others.
	(
		$name:ident {
			#[default]
			$default_var:ident => $default_val:expr,
			$($var:ident => $val:expr),+ $(,)?
		}
		=> $callback:ident
	) => {
		$callback!($name { $default_var, $($var),+ } { $default_var => $default_val, $($var => $val),+ } with_default $default_var);
	};
	// Pattern: others first, `#[default]` somewhere in middle.
	(
		$name:ident {
			$($before_var:ident => $before_val:expr),+,
			#[default]
			$default_var:ident => $default_val:expr,
			$($after_var:ident => $after_val:expr),+ $(,)?
		}
		=> $callback:ident
	) => {
		$callback!($name { $($before_var),+, $default_var, $($after_var),+ } { $($before_var => $before_val),+, $default_var => $default_val, $($after_var => $after_val),+ } with_default $default_var);
	};
	// Pattern: others first, `#[default]` at the end.
	(
		$name:ident {
			$($before_var:ident => $before_val:expr),+,
			#[default]
			$default_var:ident => $default_val:expr $(,)?
		}
		=> $callback:ident
	) => {
		$callback!($name { $($before_var),+, $default_var } { $($before_var => $before_val),+, $default_var => $default_val } with_default $default_var);
	};
	// Pattern: no default attribute, fallback to normal enum.
	(
		$name:ident {
			$($var:ident => $val:expr),* $(,)?
		}
		=> $callback:ident
	) => {
		$callback!($name { $($var),* } { $($var => $val),* } without_default);
	};
}
pub(crate) use _parse_enum_with_default;

// Helper macro for _parse_enum_with_default callback.
macro_rules! _generate_serializable_enum {
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } with_default $default_var:ident) => {
		crate::util::_define_enum!($name { $($var),* } with_default $default_var);
		crate::util::_impl_enum_as_str!($name { $($var_val => $val),* });
		crate::util::_impl_enum_serialize!($name);
	};
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } without_default) => {
		crate::util::_define_enum!($name { $($var),* });
		crate::util::_impl_enum_as_str!($name { $($var_val => $val),* });
		crate::util::_impl_enum_serialize!($name);
	};
}
pub(crate) use _generate_serializable_enum;

macro_rules! _generate_deserializable_enum {
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } with_default $default_var:ident) => {
		crate::util::_define_enum!($name { $($var),* } with_default $default_var);
		crate::util::_impl_enum_deserialize!($name { $($var_val => $val),* });
	};
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } without_default) => {
		crate::util::_define_enum!($name { $($var),* });
		crate::util::_impl_enum_deserialize!($name { $($var_val => $val),* });
	};
}
pub(crate) use _generate_deserializable_enum;

macro_rules! _generate_serializable_deserializable_enum {
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } with_default $default_var:ident) => {
		crate::util::_define_enum!($name { $($var),* } with_default $default_var);
		crate::util::_impl_enum_as_str!($name { $($var_val => $val),* });
		crate::util::_impl_enum_serialize!($name);
		crate::util::_impl_enum_deserialize!($name { $($var_val => $val),* });
	};
	($name:ident { $($var:ident),* } { $($var_val:ident => $val:expr),* } without_default) => {
		crate::util::_define_enum!($name { $($var),* });
		crate::util::_impl_enum_as_str!($name { $($var_val => $val),* });
		crate::util::_impl_enum_serialize!($name);
		crate::util::_impl_enum_deserialize!($name { $($var_val => $val),* });
	};
}
pub(crate) use _generate_serializable_deserializable_enum;

macro_rules! _impl_enum_as_str {
	($name:ident { $($var:ident => $val:expr),* $(,)? }) => {
		impl $name {
			pub const fn as_str(&self) -> &'static str {
				match self {
					$(Self::$var => $val),*
				}
			}
		}
	};
}
pub(crate) use _impl_enum_as_str;

macro_rules! _impl_enum_serialize {
	($name:ident) => {
		impl serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				serializer.serialize_str(self.as_str())
			}
		}
	};
}
pub(crate) use _impl_enum_serialize;

macro_rules! _impl_enum_deserialize {
	($name:ident { $($var:ident => $val:expr),* $(,)? }) => {
		impl<'de> serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				let s = String::deserialize(deserializer)?;
				match s.as_str() {
					$($val => Ok(Self::$var),)*
					_ => Err(serde::de::Error::custom(format!("unknown variant: {s}"))),
				}
			}
		}
	};
}
pub(crate) use _impl_enum_deserialize;

macro_rules! impl_serializable_enum {
	{
		$(
			$name:ident {
				$($content:tt)*
			}
		)*
	} => {
		$(
			crate::util::_parse_enum_with_default!(
				$name { $($content)* } => _generate_serializable_enum
			);
		)*
	};
}
pub(crate) use impl_serializable_enum;

macro_rules! impl_deserializable_enum {
	{
		$(
			$name:ident {
				$($content:tt)*
			}
		)*
	} => {
		$(
			crate::util::_parse_enum_with_default!(
				$name { $($content)* } => _generate_deserializable_enum
			);
		)*
	};
}
pub(crate) use impl_deserializable_enum;

macro_rules! impl_serializable_deserializable_enum {
	{
		$(
			$name:ident {
				$($content:tt)*
			}
		)*
	} => {
		$(
			crate::util::_parse_enum_with_default!(
				$name { $($content)* } => _generate_serializable_deserializable_enum
			);
		)*
	};
}
pub(crate) use impl_serializable_deserializable_enum;

pub trait _ConstStrT {
	const VAL: &'static str;
}

#[derive(Clone, Debug, Default)]
pub struct _ConstStr<T>(PhantomData<T>)
where
	T: _ConstStrT;
impl<T> Serialize for _ConstStr<T>
where
	T: _ConstStrT,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(T::VAL)
	}
}
impl<'de, T> Deserialize<'de> for _ConstStr<T>
where
	T: _ConstStrT + Default,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;

		if s == T::VAL {
			Ok(Self(PhantomData))
		} else {
			Err(DeserializeError::custom(format!("expected '{}', found '{s}'", T::VAL)))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	impl_serializable_deserializable_enum! {
		TestStatus {
			#[default]
			Active => "active",
			Inactive => "inactive",
			Pending => "pending",
		}
	}

	#[test]
	fn default_value_should_work() {
		let status = TestStatus::default();
		assert_eq!(status, TestStatus::Active);
	}

	#[test]
	fn serialization_should_work() {
		let status = TestStatus::Active;
		let serialized = serde_json::to_string(&status).unwrap();
		assert_eq!(serialized, "\"active\"");

		let status = TestStatus::Inactive;
		let serialized = serde_json::to_string(&status).unwrap();
		assert_eq!(serialized, "\"inactive\"");
	}

	#[test]
	fn deserialization_should_work() {
		let deserialized: TestStatus = serde_json::from_str("\"active\"").unwrap();
		assert_eq!(deserialized, TestStatus::Active);

		let deserialized: TestStatus = serde_json::from_str("\"inactive\"").unwrap();
		assert_eq!(deserialized, TestStatus::Inactive);
	}

	// Test with default in different positions.
	impl_serializable_deserializable_enum! {
		TestStatus2 {
			First => "first",
			#[default]
			Second => "second",
			Third => "third",
		}
	}

	#[test]
	fn default_in_middle_should_work() {
		let status = TestStatus2::default();
		assert_eq!(status, TestStatus2::Second);
	}

	impl_serializable_deserializable_enum! {
		TestStatus3 {
			First => "first",
			Second => "second",
			#[default]
			Third => "third",
		}
	}

	#[test]
	fn default_at_end_should_work() {
		let status = TestStatus3::default();
		assert_eq!(status, TestStatus3::Third);
	}
}
