use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, DeserializeOwned, Deserializer, Visitor},
    ser::{self, Serialize, Serializer},
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed deserializing JSON string")]
    InvalidJson(#[source] serde_json::Error),
}

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let json = serde_json::to_string(value).map_err(ser::Error::custom)?;
    serializer.serialize_str(&json)
}

#[allow(dead_code)]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    deserializer.deserialize_str(JsonStringVisitor(PhantomData))
}

struct JsonStringVisitor<T>(PhantomData<T>);

impl<T> Visitor<'_> for JsonStringVisitor<T>
where
    T: DeserializeOwned,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("string value that contains JSON")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::from_str(v).map_err(|e| de::Error::custom(Error::InvalidJson(e)))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner {
        value: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SimpleStruct {
        #[serde(with = "super")]
        inner: Inner,
    }

    #[test]
    fn roundtrip() {
        assert_tokens(
            &SimpleStruct {
                inner: Inner { value: 5 },
            },
            &[
                Token::Struct {
                    name: "SimpleStruct",
                    len: 1,
                },
                Token::Str("inner"),
                Token::Str(r#"{"value":5}"#),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn deser() {
        assert_de_tokens_error::<SimpleStruct>(
            &[
                Token::Struct {
                    name: "SimpleStruct",
                    len: 1,
                },
                Token::Str("inner"),
                Token::Str(""),
                Token::StructEnd,
            ],
            "failed deserializing JSON string",
        );
    }
}
