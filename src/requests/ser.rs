use rgb::RGBA8;
use serde::ser::{self, Serialize, Serializer};
use time::Duration;

pub fn duration_millis<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let millis = i64::try_from(value.whole_milliseconds()).map_err(ser::Error::custom)?;
    serializer.serialize_i64(millis)
}

pub fn duration_millis_opt<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => {
            let millis = i64::try_from(v.whole_milliseconds()).map_err(ser::Error::custom)?;
            serializer.serialize_some(&millis)
        }
        None => serializer.serialize_none(),
    }
}

pub fn bitflags_u8<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<u8> + Copy,
{
    serializer.serialize_u8((*value).into())
}

pub fn rgba8_inverse<S>(value: &RGBA8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let abgr =
        (value.a as u32) << 24 | (value.b as u32) << 16 | (value.g as u32) << 8 | (value.r as u32);
    serializer.serialize_u32(abgr)
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
    use serde_test::{assert_ser_tokens, Token};

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
    fn ser_bitflags_u8() {
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
            #[serde(serialize_with = "bitflags_u8")]
            value: Flags,
        }

        assert_ser_tokens(
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
    }

    #[test]
    fn ser_rgba8_inverse() {
        #[derive(Serialize)]
        struct SimpleDuration {
            #[serde(serialize_with = "rgba8_inverse")]
            value: RGBA8,
        }

        assert_ser_tokens(
            &SimpleDuration {
                value: RGBA8::new(1, 2, 3, 4),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::U32(0x04030201),
                Token::StructEnd,
            ],
        );
    }
}
