use chrono::Duration;
use serde::{ser, Serializer};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("duration of {} days is too big to be serialized as nanoseconds", .0.num_days())]
    DurationTooBig(Duration),
}

pub fn duration_nanos<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value.num_nanoseconds() {
        Some(nanos) => serializer.serialize_i64(nanos),
        None => Err(ser::Error::custom(Error::DurationTooBig(*value))),
    }
}
