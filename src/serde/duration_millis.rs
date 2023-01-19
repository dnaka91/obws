use std::fmt;

use serde::{
    de::{self, Deserializer, Visitor},
    ser::{self, Serializer},
};
use time::Duration;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("value is too large for an i64: {0}")]
    ValueTooLargeI64(#[source] std::num::TryFromIntError),
}

pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let millis = i64::try_from(value.whole_milliseconds())
        .map_err(|e| ser::Error::custom(Error::ValueTooLargeI64(e)))?;
    serializer.serialize_i64(millis)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(DurationMillisVisitor)
}

struct DurationMillisVisitor;

impl<'de> Visitor<'de> for DurationMillisVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in milliseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration::milliseconds(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        i64::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueTooLargeI64(e)))
            .and_then(|v| self.visit_i64(v))
    }
}

pub mod option {
    use super::*;

    pub fn serialize<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => {
                let millis = i64::try_from(v.whole_milliseconds())
                    .map_err(|e| ser::Error::custom(Error::ValueTooLargeI64(e)))?;
                serializer.serialize_some(&millis)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(DurationMillisOptVisitor)
    }

    struct DurationMillisOptVisitor;

    impl<'de> Visitor<'de> for DurationMillisOptVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a duration in milliseconds")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_i64(DurationMillisVisitor)
                .map(Some)
        }
    }

    #[cfg(test)]
    mod tests {
        use serde::{Deserialize, Serialize};
        use serde_test::{assert_de_tokens_error, assert_tokens, Token};
        use time::Duration;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct SimpleDuration {
            #[serde(with = "super")]
            value: Option<Duration>,
        }

        #[test]
        fn roundtrip() {
            assert_tokens(
                &SimpleDuration {
                    value: Some(Duration::milliseconds(150)),
                },
                &[
                    Token::Struct {
                        name: "SimpleDuration",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Some,
                    Token::I64(150),
                    Token::StructEnd,
                ],
            );

            assert_tokens(
                &SimpleDuration { value: None },
                &[
                    Token::Struct {
                        name: "SimpleDuration",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::None,
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn deser() {
            assert_de_tokens_error::<SimpleDuration>(
                &[
                    Token::Struct {
                        name: "SimpleDuration",
                        len: 0,
                    },
                    Token::StructEnd,
                ],
                "missing field `value`",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};
    use time::Duration;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SimpleDuration {
        #[serde(with = "super")]
        value: Duration,
    }

    #[test]
    fn roundtrip() {
        assert_tokens(
            &SimpleDuration {
                value: Duration::milliseconds(150),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(150),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deser() {
        assert_de_tokens(
            &SimpleDuration {
                value: Duration::milliseconds(150),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(150),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleDuration>(
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(u64::MAX),
                Token::StructEnd,
            ],
            "value is too large for an i64: out of range integral type conversion attempted",
        );
    }
}
