use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;

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

pub fn rgba8_inverse<'de, D>(deserializer: D) -> Result<Option<RGBA8>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u32(Rgba8InverseOptVisitor)
}

struct Rgba8InverseOptVisitor;

impl<'de> Visitor<'de> for Rgba8InverseOptVisitor {
    type Value = Option<RGBA8>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a RGBA color value encoded as integer in inverse order (ABGR)")
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

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match u32::try_from(v) {
            Ok(v) => self.visit_u32(v),
            Err(e) => Err(Error::custom(e)),
        }
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
    use serde_json::json;

    use super::*;

    #[test]
    fn deser_string_comma_list() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleList {
            #[serde(deserialize_with = "string_comma_list")]
            value: Vec<String>,
        }

        let input = json! {{ "value": "a,b,c" }};
        let expect = SimpleList {
            value: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        };
        assert_eq!(expect, serde_json::from_value(input).unwrap());
    }
}
