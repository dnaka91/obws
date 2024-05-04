use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! item_id {
    ($ident:ident, $name:literal, $name_field:literal, $uuid_field:literal) => {
        #[doc = concat!("Identifier of the", $name, ".")]
        #[derive(
            Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
        )]
        pub struct $ident {
            #[doc = concat!("Name of the", $name, ".")]
            #[serde(rename = $name_field)]
            pub name: String,
            #[doc = concat!("UUID of the", $name, ".")]
            #[serde(rename = $uuid_field)]
            pub uuid: Uuid,
        }

        impl PartialEq<$ident> for String {
            fn eq(&self, other: &$ident) -> bool {
                other == self.as_str()
            }
        }

        impl PartialEq<$ident> for &str {
            fn eq(&self, other: &$ident) -> bool {
                other == *self
            }
        }

        impl PartialEq<$ident> for Uuid {
            fn eq(&self, other: &$ident) -> bool {
                other == self
            }
        }

        impl PartialEq<str> for $ident {
            fn eq(&self, other: &str) -> bool {
                self.name == other
            }
        }

        impl PartialEq<Uuid> for $ident {
            fn eq(&self, other: &Uuid) -> bool {
                self.uuid == *other
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
    CurrentPreviewSceneId,
    "current preview scene",
    "currentPreviewSceneName",
    "currentPreviewSceneUuid"
);
item_id!(
    CurrentProgramSceneId,
    "current program scene",
    "currentProgramSceneName",
    "currentProgramSceneUuid"
);
item_id!(
    CurrentSceneTransitionId,
    "current scene transition",
    "currentSceneTransitionName",
    "currentSceneTransitionUuid"
);

macro_rules! convert {
    ($source:ident, $target:ident) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                Self {
                    name: value.name,
                    uuid: value.uuid,
                }
            }
        }

        impl From<$target> for $source {
            fn from(value: $target) -> Self {
                Self {
                    name: value.name,
                    uuid: value.uuid,
                }
            }
        }
    };
}

convert!(SceneId, CurrentPreviewSceneId);
convert!(SceneId, CurrentProgramSceneId);
convert!(CurrentPreviewSceneId, CurrentProgramSceneId);
convert!(TransitionId, CurrentSceneTransitionId);

macro_rules! request {
    ($ident:ident) => {
        impl From<$ident> for crate::requests::ids::$ident<'_> {
            fn from(value: $ident) -> Self {
                Self::Uuid(value.uuid)
            }
        }

        impl From<&$ident> for crate::requests::ids::$ident<'_> {
            fn from(value: &$ident) -> Self {
                Self::Uuid(value.uuid)
            }
        }

        impl PartialEq<$ident> for crate::requests::ids::$ident<'_> {
            fn eq(&self, other: &$ident) -> bool {
                match *self {
                    Self::Name(name) => name == other.name,
                    Self::Uuid(uuid) => uuid == other.uuid,
                }
            }
        }

        impl PartialEq<crate::requests::ids::$ident<'_>> for $ident {
            fn eq(&self, other: &crate::requests::ids::$ident<'_>) -> bool {
                other == self
            }
        }
    };
}

request!(InputId);
request!(SceneId);
request!(SourceId);
request!(TransitionId);
