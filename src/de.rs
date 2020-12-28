//! Custom deserializers that are used in both the [`events`](crate::events) and
//! [`responses`](crate::responses) modules.

use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::marker::PhantomData;

use anyhow::{Context, Result};
use chrono::Duration;
use serde::de::{Deserializer, Error, Visitor};

pub fn duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptDurationVisitor)
}

struct OptDurationVisitor;

impl<'de> Visitor<'de> for OptDurationVisitor {
    type Value = Option<Duration>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an optional duration formatted as 'HH:MM:SS.mmm'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let duration = || -> Result<Duration> {
            let mut hms = v.splitn(3, ':');
            let hours = hms.next().context("hours missing")?.parse()?;
            let minutes = hms.next().context("minutes missing")?.parse()?;
            let seconds = hms.next().context("seconds missing")?;

            let mut sm = seconds.splitn(2, '.');
            let seconds = sm.next().context("seconds missing")?.parse()?;
            let millis = sm.next().context("milliseconds missing")?.parse()?;

            Ok(Duration::hours(hours)
                + Duration::minutes(minutes)
                + Duration::seconds(seconds)
                + Duration::milliseconds(millis))
        };

        duration().map(Some).map_err(Error::custom)
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
        deserializer.deserialize_str(Self)
    }
}

pub fn duration_millis_opt<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(OptDurationMillisVisitor)
}

struct OptDurationMillisVisitor;

impl<'de> Visitor<'de> for OptDurationMillisVisitor {
    type Value = Option<Duration>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in milliseconds where -1 means a fixed duration")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(if v < 0 {
            None
        } else {
            Some(Duration::milliseconds(v))
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
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
        deserializer.deserialize_i64(Self)
    }
}

pub fn duration_millis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(DurationMillisVisitor)
}

struct DurationMillisVisitor;

impl<'de> Visitor<'de> for DurationMillisVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in milliseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Duration::milliseconds(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
            Err(e) => Err(Error::custom(e)),
        }
    }
}

pub fn duration_nanos<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(DurationNanosVisitor)
}

struct DurationNanosVisitor;

impl<'de> Visitor<'de> for DurationNanosVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a duration in nanoseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Duration::nanoseconds(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i64::try_from(v) {
            Ok(value) => self.visit_i64(value),
            Err(e) => Err(Error::custom(e)),
        }
    }
}

pub fn bitflags_u8<'de, D, T, TE>(deserializer: D) -> Result<T, D::Error>
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
        E: Error,
    {
        u8::try_from(v)
            .map_err(|_| Error::custom("value doesn't fit into an u8 integer"))
            .and_then(|v| self.visit_u8(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::try_from(v).map_err(Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        u8::try_from(v)
            .map_err(|_| Error::custom("value doesn't fit into an u8 integer"))
            .and_then(|v| self.visit_u8(v))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[test]
    fn deser_duration() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct SimpleDuration {
            #[serde(deserialize_with = "duration")]
            value: Option<Duration>,
        };

        let input = json! {{ "value": "02:15:04.310" }};
        let expect = SimpleDuration {
            value: Some(
                Duration::hours(2)
                    + Duration::minutes(15)
                    + Duration::seconds(4)
                    + Duration::milliseconds(310),
            ),
        };
        assert_eq!(expect, serde_json::from_value(input).unwrap());
    }
}
