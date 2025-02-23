use std::fmt;

use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};
use time::Duration;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("hours missing")]
    HoursMissing,
    #[error("minutes missing")]
    MinutesMissing,
    #[error("seconds missing")]
    SecondsMissing,
    #[error("milliseconds missing")]
    MillisecondsMissing,
    #[error("invalid integer")]
    InvalidInteger(#[from] std::num::ParseIntError),
}

#[allow(dead_code)]
pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let whole_secs = value.whole_seconds();
    let hours = whole_secs / 3600;
    let minutes = whole_secs % 3600 / 60;
    let seconds = whole_secs % 3600 % 60;
    let millis = value.subsec_milliseconds();

    serializer.serialize_str(&format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}"))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DurationTimecodeVisitor)
}

struct DurationTimecodeVisitor;

impl Visitor<'_> for DurationTimecodeVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration formatted as 'HH:MM:SS.mmm'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let duration = || -> Result<Duration, Error> {
            let mut hms = v.splitn(3, ':');
            let hours = hms.next().ok_or(Error::HoursMissing)?.parse()?;
            let minutes = hms.next().ok_or(Error::MinutesMissing)?.parse()?;
            let seconds = hms.next().ok_or(Error::SecondsMissing)?;

            let mut sm = seconds.splitn(2, '.');
            let seconds = sm.next().ok_or(Error::SecondsMissing)?.parse()?;
            let millis = sm.next().ok_or(Error::MillisecondsMissing)?.parse()?;

            Ok(Duration::hours(hours)
                + Duration::minutes(minutes)
                + Duration::seconds(seconds)
                + Duration::milliseconds(millis))
        };

        duration().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{Token, assert_tokens};
    use time::Duration;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SimpleDuration {
        #[serde(with = "super")]
        value: Duration,
    }

    #[test]
    fn roundtrip() {
        assert_tokens(
            &SimpleDuration {
                value: Duration::hours(2)
                    + Duration::minutes(15)
                    + Duration::seconds(4)
                    + Duration::milliseconds(310),
            },
            &[
                Token::Struct {
                    name: "SimpleDuration",
                    len: 1,
                },
                Token::Str("value"),
                Token::Str("02:15:04.310"),
                Token::StructEnd,
            ],
        );
    }
}
