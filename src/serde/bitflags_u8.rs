use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("value is too large for an u8: {0}")]
    ValueTooLarge(#[source] std::num::TryFromIntError),
    #[error("conversion from integer failed: {0}")]
    IntConversionFailed(String),
}

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<u8> + Copy,
{
    serializer.serialize_u8((*value).into())
}

pub fn deserialize<'de, D, T, TE>(deserializer: D) -> Result<T, D::Error>
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
            .map_err(|e| de::Error::custom(Error::ValueTooLarge(e)))
            .and_then(|v| self.visit_u8(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::try_from(v).map_err(|e| de::Error::custom(Error::IntConversionFailed(e.to_string())))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u8::try_from(v)
            .map_err(|e| de::Error::custom(Error::ValueTooLarge(e)))
            .and_then(|v| self.visit_u8(v))
    }
}

#[cfg(test)]
mod tests {
    use bitflags::bitflags;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};

    bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Flags: u8 {
            const ONE = 1;
            const TWO = 2;
        }
    }

    impl From<Flags> for u8 {
        fn from(value: Flags) -> Self {
            value.bits()
        }
    }

    impl TryFrom<u8> for Flags {
        type Error = crate::Error;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            Self::from_bits(value).ok_or(Self::Error::UnknownFlags(value))
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SimpleFlags {
        #[serde(with = "super")]
        value: Flags,
    }

    #[test]
    fn roundtrip() {
        assert_tokens(
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
}
