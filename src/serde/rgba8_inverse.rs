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
    let abgr =
        (value.a as u32) << 24 | (value.b as u32) << 16 | (value.g as u32) << 8 | (value.r as u32);
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

impl<'de> Visitor<'de> for Rgba8InverseVisitor {
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
        Ok(RGBA8 {
            r: (v & 0xff) as u8,
            g: (v >> 8 & 0xff) as u8,
            b: (v >> 16 & 0xff) as u8,
            a: (v >> 24 & 0xff) as u8,
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
    use serde_test::{assert_tokens, Token};

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
                Token::U32(0x04030201),
                Token::StructEnd,
            ],
        );
    }
}
