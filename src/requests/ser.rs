use std::convert::TryFrom;

use rgb::RGBA8;
use serde::ser::{self, Serialize, Serializer};
use time::Duration;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("duration of {} days is too big to be serialized as number", .0.whole_days())]
    DurationTooBig(Duration),
}

pub fn duration_millis_opt<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(duration) => {
            serializer.serialize_some(&to_i64(duration, duration.whole_milliseconds())?)
        }
        None => serializer.serialize_none(),
    }
}

pub fn duration_millis<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(to_i64(value, value.whole_milliseconds())?)
}

pub fn duration_nanos<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(to_i64(value, value.whole_nanoseconds())?)
}

fn to_i64<E>(duration: &Duration, value: i128) -> Result<i64, E>
where
    E: ser::Error,
{
    i64::try_from(value).map_err(|_| E::custom(Error::DurationTooBig(*duration)))
}

pub fn bitflags_u8<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<u8> + Copy,
{
    serializer.serialize_some(&(*value).into())
}

pub fn bitflags_u8_opt<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<u8> + Copy,
{
    match value {
        Some(flags) => bitflags_u8(flags, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn rgba8_inverse<S>(value: &RGBA8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let abgr =
        (value.a as u32) << 24 | (value.b as u32) << 16 | (value.g as u32) << 8 | (value.r as u32);
    serializer.serialize_some(&abgr)
}

pub fn rgba8_inverse_opt<S>(value: &Option<RGBA8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(rgba) => rgba8_inverse(rgba, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn json_string<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let json = serde_json::to_string(value).map_err(ser::Error::custom)?;
    serializer.serialize_str(&json)
}

#[cfg(test)]
mod tests {
    use bitflags::bitflags;
    use serde::Serialize;
    use serde_test::{assert_ser_tokens, assert_ser_tokens_error, Token};

    use super::*;

    #[test]
    fn ser_duration_millis_opt() {
        #[derive(Serialize)]
        struct SimpleDuration {
            #[serde(serialize_with = "duration_millis_opt")]
            value: Option<Duration>,
        }

        assert_ser_tokens(
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

        assert_ser_tokens(
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
    fn ser_duration_millis() {
        #[derive(Serialize)]
        struct SimpleDuration {
            #[serde(serialize_with = "duration_millis")]
            value: Duration,
        }

        assert_ser_tokens(
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
    fn ser_duration_nanos() {
        #[derive(Serialize)]
        struct SimpleDuration {
            #[serde(serialize_with = "duration_nanos")]
            value: Duration,
        }

        assert_ser_tokens(
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

        assert_ser_tokens_error(
            &SimpleDuration {
                value: Duration::days(365_000_000),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
            ],
            "duration of 365000000 days is too big to be serialized as number",
        );
    }

    #[test]
    fn ser_bitflags_u8_opt() {
        bitflags! {
            struct Flags: u8 {
                const ONE = 1;
                const TWO = 2;
            }
        }

        impl From<Flags> for u8 {
            fn from(value: Flags) -> Self {
                value.bits
            }
        }

        #[derive(Serialize)]
        struct SimpleFlags {
            #[serde(serialize_with = "bitflags_u8_opt")]
            value: Option<Flags>,
        }

        assert_ser_tokens(
            &SimpleFlags {
                value: Some(Flags::ONE | Flags::TWO),
            },
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::Some,
                Token::U8(3),
                Token::StructEnd,
            ],
        );

        assert_ser_tokens(
            &SimpleFlags { value: None },
            &[
                Token::Struct {
                    name: "SimpleFlags",
                    len: 1,
                },
                Token::Str("value"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn ser_rgba8_inverse_opt() {
        #[derive(Serialize)]
        struct SimpleDuration {
            #[serde(serialize_with = "rgba8_inverse_opt")]
            value: Option<RGBA8>,
        }

        assert_ser_tokens(
            &SimpleDuration {
                value: Some(RGBA8::new(1, 2, 3, 4)),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::Some,
                Token::U32(0x04030201),
                Token::StructEnd,
            ],
        );

        assert_ser_tokens(
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
}
