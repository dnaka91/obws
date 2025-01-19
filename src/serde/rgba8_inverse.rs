use std::fmt;

use rgb::RGBA8;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("value is too large for an u32: {0}")]
    ValueTooLarge(#[source] std::num::TryFromIntError),
}

pub fn serialize<S>(value: &RGBA8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let abgr = u32::from_be_bytes([value.a, value.b, value.g, value.r]);
    serializer.serialize_u32(abgr)
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<RGBA8, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u32(Rgba8InverseVisitor)
}

struct Rgba8InverseVisitor;

impl Visitor<'_> for Rgba8InverseVisitor {
    type Value = RGBA8;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RGBA color encoded as u32 integer in reverse order")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u32::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueTooLarge(e)))
            .and_then(|v| self.visit_u32(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v = v.to_be_bytes();
        Ok(RGBA8 {
            r: v[3],
            g: v[2],
            b: v[1],
            a: v[0],
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u32::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueTooLarge(e)))
            .and_then(|v| self.visit_u32(v))
    }
}

#[cfg(test)]
mod tests {
    use rgb::RGBA8;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens, assert_tokens, Token};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SimpleDuration {
        #[serde(with = "super")]
        value: RGBA8,
    }

    #[test]
    fn roundtrip() {
        assert_tokens(
            &SimpleDuration {
                value: RGBA8::new(1, 2, 3, 4),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::U32(0x0403_0201),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleDuration {
                value: RGBA8::new(1, 2, 3, 4),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(0x0403_0201),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleDuration {
                value: RGBA8::new(1, 2, 3, 4),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(0x0403_0201),
                Token::StructEnd,
            ],
        );
    }
}
