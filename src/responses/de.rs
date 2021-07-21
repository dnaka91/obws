use std::{convert::TryFrom, fmt, iter::FromIterator, marker::PhantomData};

use rgb::RGBA8;
use serde::de::{Deserializer, Error, Visitor};

pub fn string_comma_list<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<String>,
{
    deserializer.deserialize_str(StringListVisitor {
        sep: ',',
        container: PhantomData,
    })
}

struct StringListVisitor<T> {
    sep: char,
    container: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for StringListVisitor<T>
where
    T: FromIterator<String>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a string containing values separated by '{}'",
            self.sep
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.split(self.sep).map(|s| s.to_owned()).collect())
    }
}

pub fn rgba8_inverse_opt<'de, D>(deserializer: D) -> Result<Option<RGBA8>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(Rgba8InverseOptVisitor)
}

struct Rgba8InverseOptVisitor;

impl<'de> Visitor<'de> for Rgba8InverseOptVisitor {
    type Value = Option<RGBA8>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a RGBA color value encoded as integer in inverse order (ABGR)")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match u32::try_from(v) {
            Ok(v) => self.visit_u32(v),
            Err(e) => Err(Error::custom(e)),
        }
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Some(RGBA8::new(
            (v & 0xff) as u8,
            (v >> 8 & 0xff) as u8,
            (v >> 16 & 0xff) as u8,
            (v >> 24 & 0xff) as u8,
        )))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match u32::try_from(v) {
            Ok(v) => self.visit_u32(v),
            Err(e) => Err(Error::custom(e)),
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(Self)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use super::*;

    #[test]
    fn deser_string_comma_list() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleList {
            #[serde(deserialize_with = "string_comma_list")]
            value: Vec<String>,
        }

        assert_de_tokens(
            &SimpleList {
                value: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            },
            &[
                Token::Struct {
                    name: "SimpleList",
                    len: 1,
                },
                Token::Str("value"),
                Token::Str("a,b,c"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deser_rgba8_inverse_opt() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleColor {
            #[serde(deserialize_with = "rgba8_inverse_opt")]
            value: Option<RGBA8>,
        }

        assert_de_tokens(
            &SimpleColor {
                value: Some(RGBA8::new(1, 2, 3, 4)),
            },
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(0x04030201),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleColor>(
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::I64(i64::MIN),
                Token::StructEnd,
            ],
            "out of range integral type conversion attempted",
        );

        assert_de_tokens(
            &SimpleColor {
                value: Some(RGBA8::new(1, 2, 3, 4)),
            },
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::U32(0x04030201),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleColor {
                value: Some(RGBA8::new(1, 2, 3, 4)),
            },
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(0x04030201),
                Token::StructEnd,
            ],
        );

        assert_de_tokens_error::<SimpleColor>(
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::U64(u64::MAX),
                Token::StructEnd,
            ],
            "out of range integral type conversion attempted",
        );

        assert_de_tokens(
            &SimpleColor { value: None },
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::None,
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &SimpleColor {
                value: Some(RGBA8::new(1, 2, 3, 4)),
            },
            &[
                Token::Struct {
                    name: "SimpleColor",
                    len: 1,
                },
                Token::Str("value"),
                Token::Some,
                Token::U32(0x04030201),
                Token::StructEnd,
            ],
        );
    }
}
