//! Custom deserializers that are used in both the [`events`](crate::events) and
//! [`responses`](crate::responses) modules.

use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::marker::PhantomData;

use chrono::Duration;
use serde::de::{self, Deserializer, Visitor};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("hours missing")]
    HoursMissing,
    #[error("minutes missing")]
    MinutesMissing,
    #[error("seconds missing")]
    SecondsMissing,
    #[error("milliseconds missing")]
    MillisecondsMissing,
    #[error("invalid integer")]
    InvalidInteger(#[from] std::num::ParseIntError),
    #[error("value {1} is too large for an i64: {0}")]
    ValueTooLargeI64(#[source] std::num::TryFromIntError, u64),
    #[error("value doesn't fit into an u8 integer: {0}")]
    ValueDoesntFitU8(#[source] std::num::TryFromIntError),
    #[error("conversion from u8 failed: {0}")]
    ConversionFailed(String),
}

pub fn duration_opt<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(DurationOptVisitor)
}

struct DurationOptVisitor;

impl<'de> Visitor<'de> for DurationOptVisitor {
    type Value = Option<Duration>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an optional duration formatted as 'HH:MM:SS.mmm'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let duration = || -> Result<Duration, Error> {
            let mut hms = v.splitn(3, ':');
            let hours = hms.next().ok_or(Error::HoursMissing)?.parse()?;
            let minutes = hms.next().ok_or(Error::MinutesMissing)?.parse()?;
            let seconds = hms.next().ok_or(Error::SecondsMissing)?;

            let mut sm = seconds.splitn(2, '.');
            let seconds = sm.next().ok_or(Error::SecondsMissing)?.parse()?;
            let millis = sm.next().ok_or(Error::MillisecondsMissing)?.parse()?;

            Ok(Duration::hours(hours)
                + Duration::minutes(minutes)
                + Duration::seconds(seconds)
                + Duration::milliseconds(millis))
        };

        duration().map(Some).map_err(de::Error::custom)
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
        deserializer.deserialize_str(Self)
    }
}

pub fn duration_millis_opt<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(DurationMillisOptVisitor)
}

struct DurationMillisOptVisitor;

impl<'de> Visitor<'de> for DurationMillisOptVisitor {
    type Value = Option<Duration>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in milliseconds where -1 means a fixed duration")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(if v < 0 {
            None
        } else {
            Some(Duration::milliseconds(v))
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
            Err(e) => Err(de::Error::custom(Error::ValueTooLargeI64(e, v))),
        }
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
        deserializer.deserialize_i64(Self)
    }
}

pub fn duration_millis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
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
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
            Err(e) => Err(de::Error::custom(Error::ValueTooLargeI64(e, v))),
        }
    }
}

pub fn duration_nanos<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(DurationNanosVisitor)
}

struct DurationNanosVisitor;

impl<'de> Visitor<'de> for DurationNanosVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in nanoseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration::nanoseconds(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
            Err(e) => Err(de::Error::custom(Error::ValueTooLargeI64(e, v))),
        }
    }
}

pub fn bitflags_u8<'de, D, T, TE>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<u8, Error = TE>,
    TE: Display,
{
    deserializer.deserialize_u8(BitflagsU8Visitor { flags: PhantomData })
}

struct BitflagsU8Visitor<T, TE> {
    flags: PhantomData<(T, TE)>,
}

impl<'de, T, TE> Visitor<'de> for BitflagsU8Visitor<T, TE>
where
    T: TryFrom<u8, Error = TE>,
    TE: Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bitflags encoded as u8 integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u8::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueDoesntFitU8(e)))
            .and_then(|v| self.visit_u8(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::try_from(v).map_err(|e| de::Error::custom(Error::ConversionFailed(e.to_string())))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u8::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueDoesntFitU8(e)))
            .and_then(|v| self.visit_u8(v))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use bitflags::bitflags;
    use serde::Deserialize;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use super::*;

    #[test]
    fn deser_duration_opt() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration_opt")]
            value: Option<Duration>,
        }

        assert_de_tokens(
            &SimpleDuration {
                value: Some(
                    Duration::hours(2)
                        + Duration::minutes(15)
                        + Duration::seconds(4)
                        + Duration::milliseconds(310),
                ),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::Str("02:15:04.310"),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
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

        assert_de_tokens(
            &SimpleDuration {
                value: Some(
                    Duration::hours(2)
                        + Duration::minutes(15)
                        + Duration::seconds(4)
                        + Duration::milliseconds(310),
                ),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::Some,
                Token::Str("02:15:04.310"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deser_duration_millis_opt() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration_millis_opt")]
            value: Option<Duration>,
        }

        assert_de_tokens(
            &SimpleDuration {
                value: Some(Duration::milliseconds(150)),
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

        assert_de_tokens(
            &SimpleDuration { value: None },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(-1),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleDuration {
                value: Some(Duration::milliseconds(150)),
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
            "value 18446744073709551615 is too large for an i64: \
            out of range integral type conversion attempted",
        );

        assert_de_tokens(
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

        assert_de_tokens(
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
    }

    #[test]
    fn deser_duration_millis() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration_millis")]
            value: Duration,
        }

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
                Token::I64(150),
                Token::StructEnd,
            ],
        );

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
            "value 18446744073709551615 is too large for an i64: \
            out of range integral type conversion attempted",
        );
    }

    #[test]
    fn deser_duration_nanos() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration_nanos")]
            value: Duration,
        }

        assert_de_tokens(
            &SimpleDuration {
                value: Duration::nanoseconds(150),
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

        assert_de_tokens(
            &SimpleDuration {
                value: Duration::nanoseconds(150),
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
            "value 18446744073709551615 is too large for an i64: \
            out of range integral type conversion attempted",
        );
    }

    #[test]
    fn deser_bitflags_u8() {
        bitflags! {
            struct Flags: u8 {
                const ONE = 1;
                const TWO = 2;
            }
        }

        impl TryFrom<u8> for Flags {
            type Error = anyhow::Error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::from_bits(value).context("unknown flags found")
            }
        }

        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleFlags {
            #[serde(deserialize_with = "bitflags_u8")]
            value: Flags,
        }

        assert_de_tokens(
            &SimpleFlags {
                value: Flags::ONE | Flags::TWO,
            },
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(3),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleFlags>(
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(i64::MAX),
                Token::StructEnd,
            ],
            "value doesn't fit into an u8 integer: out of range integral type conversion attempted",
        );

        assert_de_tokens(
            &SimpleFlags {
                value: Flags::ONE | Flags::TWO,
            },
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::U8(3),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleFlags>(
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::U8(100),
                Token::StructEnd,
            ],
            "conversion from u8 failed: unknown flags found",
        );

        assert_de_tokens(
            &SimpleFlags {
                value: Flags::ONE | Flags::TWO,
            },
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(3),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleFlags>(
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(u64::MAX),
                Token::StructEnd,
            ],
            "value doesn't fit into an u8 integer: out of range integral type conversion attempted",
        );
    }
}
