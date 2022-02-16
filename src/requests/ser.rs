use serde::Serializer;
use time::Duration;

pub fn duration_millis<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i128(value.whole_milliseconds())
}

pub fn duration_millis_opt<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_some(&v.whole_milliseconds()),
        None => serializer.serialize_none(),
    }
}
