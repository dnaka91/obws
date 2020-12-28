use std::fmt;
use std::marker::PhantomData;

use std::iter::FromIterator;

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
