use std::fmt::{self, Display};

use serde::{Serialize, ser::SerializeStruct};
use uuid::Uuid;

macro_rules! item_id {
    ($ident:ident, $name:literal, $name_field:literal, $uuid_field:literal) => {
        #[doc = concat!("Identifier of the", $name, ".")]
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub enum $ident<'a> {
            #[doc = concat!("Name of the ", $name, ".")]
            Name(&'a str),
            #[doc = concat!("UUID of the ", $name, ".")]
            Uuid(Uuid),
        }

        impl $ident<'_> {
            /// If the identifier is a name, returns the associated value.
            ///
            /// Will return [`None`] if this identifier is not a name.
            #[must_use]
            pub fn as_name(&self) -> Option<&str> {
                match *self {
                    Self::Name(name) => Some(name),
                    Self::Uuid(_) => None,
                }
            }

            /// If the identifier is a UUID, returns the associated value.
            ///
            /// Will return [`None`] if this identifier is not a UUID.
            #[must_use]
            pub fn as_uuid(&self) -> Option<Uuid> {
                match *self {
                    Self::Name(_) => None,
                    Self::Uuid(uuid) => Some(uuid),
                }
            }
        }

        impl Default for $ident<'_> {
            fn default() -> Self {
                Self::Name("")
            }
        }

        impl Display for $ident<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Name(name) => name.fmt(f),
                    Self::Uuid(uuid) => uuid.fmt(f),
                }
            }
        }

        impl PartialEq<str> for $ident<'_> {
            fn eq(&self, other: &str) -> bool {
                match *self {
                    Self::Name(name) => name == other,
                    Self::Uuid(_) => false,
                }
            }
        }

        impl PartialEq<Uuid> for $ident<'_> {
            fn eq(&self, other: &Uuid) -> bool {
                match *self {
                    Self::Name(_) => false,
                    Self::Uuid(uuid) => uuid == *other,
                }
            }
        }

        impl PartialEq<$ident<'_>> for String {
            fn eq(&self, other: &$ident<'_>) -> bool {
                other == self.as_str()
            }
        }

        impl PartialEq<$ident<'_>> for &str {
            fn eq(&self, other: &$ident<'_>) -> bool {
                other == *self
            }
        }

        impl PartialEq<$ident<'_>> for Uuid {
            fn eq(&self, other: &$ident<'_>) -> bool {
                other == self
            }
        }

        impl<'a> From<&'a str> for $ident<'a> {
            fn from(value: &'a str) -> Self {
                Self::Name(value)
            }
        }

        impl From<Uuid> for $ident<'_> {
            fn from(value: Uuid) -> Self {
                Self::Uuid(value)
            }
        }

        impl Serialize for $ident<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut state = serializer.serialize_struct(stringify!($ident), 1)?;
                match *self {
                    Self::Name(name) => {
                        state.serialize_field($name_field, name)?;
                    }
                    Self::Uuid(uuid) => {
                        state.serialize_field($uuid_field, &uuid)?;
                    }
                }
                state.end()
            }
        }
    };
}

item_id!(InputId, "input", "inputName", "inputUuid");
item_id!(SceneId, "scene", "sceneName", "sceneUuid");
item_id!(SourceId, "source", "sourceName", "sourceUuid");
item_id!(
    TransitionId,
    "transition",
    "transitionName",
    "transitionUuid"
);

item_id!(
    DestinationSceneId,
    "destination scene",
    "destinationSceneName",
    "destinationSceneUuid"
);

macro_rules! convert {
    ($source:ident, $target:ident) => {
        impl<'a> From<$source<'a>> for $target<'a> {
            fn from(value: $source<'a>) -> Self {
                match value {
                    $source::Name(name) => Self::Name(name),
                    $source::Uuid(uuid) => Self::Uuid(uuid),
                }
            }
        }

        impl<'a> From<$target<'a>> for $source<'a> {
            fn from(value: $target<'a>) -> Self {
                match value {
                    $target::Name(name) => Self::Name(name),
                    $target::Uuid(uuid) => Self::Uuid(uuid),
                }
            }
        }
    };
}

convert!(SceneId, DestinationSceneId);

impl<'a> InputId<'a> {
    /// Convert the input identifier into a source identifier.
    ///
    /// This is a one-way operation, as there is no way of telling whether a source ID is an actual
    /// input.
    #[must_use]
    pub fn as_source(self) -> SourceId<'a> {
        match self {
            Self::Name(name) => SourceId::Name(name),
            Self::Uuid(uuid) => SourceId::Uuid(uuid),
        }
    }
}

impl<'a> From<InputId<'a>> for SourceId<'a> {
    fn from(value: InputId<'a>) -> Self {
        value.as_source()
    }
}
