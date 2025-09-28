use std::fmt;

use serde::{de::Visitor, Deserialize, Serialize};
// use unix_ts::Timestamp;
use chrono::{DateTime, TimeDelta, Utc};
/// wrapper for chrono timestamps
#[derive(Debug)]
pub struct UnixStamp {
    pub stamp: DateTime<Utc>,
}
impl UnixStamp {
    pub fn now() -> Self {
        
        Self { stamp: Utc::now() }
    }
    pub fn days_to(&self) -> TimeDelta{
        let now = Utc::now();
        now - self.stamp
    }
}
impl From<i64> for UnixStamp {
    fn from(value: i64) -> Self {
        Self {
            stamp: if let Some(dt) = DateTime::from_timestamp_secs(value) {
                dt
            } else {
                panic!("datetime failed to convert, if this actually goes off, consider refactoring to make an Option and TryFrom")
            },
        }
    }
}
impl Into<i64> for UnixStamp {
    fn into(self) -> i64 {
        self.stamp.timestamp()
    }
}

impl fmt::Display for UnixStamp{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let now = Utc::now();
        let delta = now - self.stamp;
        self.stamp.fmt(f) // forward timestamp
    }
}
impl<'de> Deserialize<'de> for UnixStamp{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            struct UnixStampVistor;
            impl <'de>Visitor<'de> for UnixStampVistor{
                type Value = UnixStamp;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter,"an i64 integer value")
                }
                
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(UnixStamp::from(v))
                }
            }
            deserializer.deserialize_i64(UnixStampVistor)
        }
}
impl Serialize for UnixStamp{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_i64(self.stamp.timestamp()) // i would use into for this but into is STUPID and consumes self.
    }
}
