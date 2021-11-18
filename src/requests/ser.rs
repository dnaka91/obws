use serde::Serializer;
use time::Duration;

pub fn duration_millis<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i128(value.whole_milliseconds())
}
