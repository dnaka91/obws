//! Custom deserializers that are used in both the [`events`](crate::events) and
//! [`responses`](crate::responses) modules.

use std::{convert::TryFrom, fmt};

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
}

pub fn duration_timecode<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DurationTimecodeVisitor)
}

struct DurationTimecodeVisitor;

impl<'de> Visitor<'de> for DurationTimecodeVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration formatted as 'HH:MM:SS.mmm'")
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

        duration().map_err(de::Error::custom)
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use super::*;

    #[test]
    fn deser_duration_timecode() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration_timecode")]
            value: Duration,
        }

        assert_de_tokens(
            &SimpleDuration {
                value: Duration::hours(2)
                    + Duration::minutes(15)
                    + Duration::seconds(4)
                    + Duration::milliseconds(310),
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
}
