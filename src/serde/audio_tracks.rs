use std::fmt;

use serde::{
    de::{self, Deserializer, Visitor},
    ser::{SerializeMap, Serializer},
};

#[derive(Debug, thiserror::Error)]
enum Error<'a> {
    #[error("track index `{0}` is out of range")]
    OutOfRange(&'a str),
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<[bool; 6], D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(AudioTracksVisitor)
}

struct AudioTracksVisitor;

impl<'de> Visitor<'de> for AudioTracksVisitor {
    type Value = [bool; 6];

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("audio tracks as key-value pairs")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut map = [false; 6];

        while let Some((key, value)) = access.next_entry::<std::borrow::Cow<'_, str>, bool>()? {
            let index = match key.as_ref() {
                "1" | "2" | "3" | "4" | "5" | "6" => key.as_bytes()[0] - b'0' - 1,
                _ => return Err(de::Error::custom(Error::OutOfRange(key.as_ref()))),
            };

            map[index as usize] = value;
        }

        Ok(map)
    }
}

pub mod option {
    use super::*;

    pub fn serialize<S>(value: &[Option<bool>; 6], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(value.iter().copied().flatten().count()))?;
        for (k, v) in ["1", "2", "3", "4", "5", "6"]
            .into_iter()
            .zip(value)
            .filter_map(|(k, v)| v.map(|v| (k, v)))
        {
            map.serialize_entry(k, &v)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{Token, assert_de_tokens, assert_de_tokens_error, assert_ser_tokens};

    #[derive(Debug, PartialEq, Serialize)]
    struct SimpleTracksSer {
        #[serde(with = "super::option")]
        value: [Option<bool>; 6],
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct SimpleTracksDe {
        #[serde(with = "super")]
        value: [bool; 6],
    }

    #[test]
    fn roundtrip() {
        assert_ser_tokens(
            &SimpleTracksSer {
                value: [Some(true), Some(true), None, None, Some(false), Some(true)],
            },
            &[
                Token::Struct {
                    name: "SimpleTracksSer",
                    len: 1,
                },
                Token::Str("value"),
                Token::Map { len: Some(4) },
                Token::Str("1"),
                Token::Bool(true),
                Token::Str("2"),
                Token::Bool(true),
                Token::Str("5"),
                Token::Bool(false),
                Token::Str("6"),
                Token::Bool(true),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleTracksDe {
                value: [true, true, false, false, false, true],
            },
            &[
                Token::Struct {
                    name: "SimpleTracksDe",
                    len: 1,
                },
                Token::Str("value"),
                Token::Map { len: Some(6) },
                Token::Str("1"),
                Token::Bool(true),
                Token::String("2"),
                Token::Bool(true),
                Token::BorrowedStr("3"),
                Token::Bool(false),
                Token::Str("4"),
                Token::Bool(false),
                Token::Str("5"),
                Token::Bool(false),
                Token::Str("6"),
                Token::Bool(true),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deser() {
        assert_de_tokens(
            &SimpleTracksDe { value: [true; 6] },
            &[
                Token::Struct {
                    name: "SimpleTracksDe",
                    len: 1,
                },
                Token::Str("value"),
                Token::Map { len: Some(6) },
                Token::Str("1"),
                Token::Bool(true),
                Token::Str("2"),
                Token::Bool(true),
                Token::Str("3"),
                Token::Bool(true),
                Token::Str("4"),
                Token::Bool(true),
                Token::Str("5"),
                Token::Bool(true),
                Token::Str("6"),
                Token::Bool(true),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleTracksDe>(
            &[
                Token::Struct {
                    name: "SimpleTracksDe",
                    len: 1,
                },
                Token::Str("value"),
                Token::Map { len: Some(1) },
                Token::Str("10"),
                Token::Bool(true),
                Token::MapEnd,
            ],
            "track index `10` is out of range",
        );
    }
}
