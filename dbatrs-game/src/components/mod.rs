use bitflags::bitflags;
pub mod common;
pub mod zone;
pub mod item;
pub mod character;
pub mod room;

#[macro_export]
macro_rules! bitflags_with_str {
    (
        $(#[$outer:meta])*
        pub struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:meta])*
                const $Flag:ident = $value:expr;
            )*
        }
    ) => {
        bitflags::bitflags! {
            $(#[$outer])*
            pub struct $BitFlags: $T {
                $(
                    $(#[$inner])*
                    const $Flag = $value;
                )*
            }
        }

        impl $BitFlags {
            /// Convert a string to the corresponding flag.
            /// (Matches the flag name exactly.)
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        stringify!($Flag) => Some($BitFlags::$Flag),
                    )*
                    _ => None,
                }
            }

            /// Convert the set flags into a comma-separated string.
            pub fn to_str(self) -> String {
                let mut v: Vec<&str> = Vec::new();
                $(
                    if self.contains($BitFlags::$Flag) {
                        v.push(stringify!($Flag));
                    }
                )*
                v.join(",")
            }

            /// Build a runtime HashMap mapping flag names to their values.
            pub fn flag_map() -> ::std::collections::HashMap<&'static str, Self> {
                let mut map = ::std::collections::HashMap::new();
                $(
                    map.insert(stringify!($Flag), $BitFlags::$Flag);
                )*
                map
            }
        }
    };
}

#[macro_export]
macro_rules! enum_with_str {
    (
        $(#[$outer:meta])*
        pub enum $Enum:ident : $T:ty {
            $(
                $(#[$inner:meta])*
                $variant:ident = $value:expr $(,)?
            )*
        }
    ) => {
        // Declare the enum with the provided derives and underlying type.
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $Enum {
            $(
                $(#[$inner])*
                $variant = $value,
            )*
        }

        impl $Enum {
            /// Convert from a string (exact match of the variant name) to the enum.
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        stringify!($variant) => Some($Enum::$variant),
                    )*
                    _ => None,
                }
            }

            /// Return the name of the variant as a string.
            pub fn to_str(self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => stringify!($variant),
                    )*
                }
            }

            /// Return the underlying numeric value.
            pub fn value(self) -> $T {
                self as $T
            }

            /// Create the enum variant from its numeric value.
            pub fn from_value(val: $T) -> Option<Self> {
                match val {
                    $(
                        $value => Some($Enum::$variant),
                    )*
                    _ => None,
                }
            }
        }
    };
}