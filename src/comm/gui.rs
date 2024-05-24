use crate::ToPrettyString;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use std::fmt::Formatter;
use super::ChannelType;

#[cfg(feature = "rusqlite")]
use rusqlite::{ToSql, types::{FromSqlError, FromSqlResult, ToSqlOutput, FromSql, Value as SqlValue, ValueRef as SqlValueRef}};
use rusqlite::types::Value;


/// The state of commanded state of a valve.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MaxSize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValveState {
    /// Undetermined state, whether because the valve is unmapped or has not been commanded yet.
    Undetermined,
    /// Valve Disconnected
    Disconnected,
    /// Valve Open
    Open,
    /// Valve Closed
    Closed,
    /// Valve Fault
    Fault,
}

impl fmt::Display for ValveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Undetermined => "undetermined",
            Self::Disconnected => "cisconnected",
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Fault => "fault",
        })
    }
}

impl ToPrettyString for ValveState {
    /// Converts the valve state into a colored string ready to be displayed on the interface.
    fn to_pretty_string(&self) -> String {
        match self {
            Self::Undetermined => "",
            Self::Disconnected => "",
            Self::Open => "",
            Self::Closed => "",
            Self::Fault => "",
        }.to_owned()
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for ValveState {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

/// Stores the estimated actual valve state as well as the software-commanded state.
#[derive(Clone, Debug, Deserialize, Eq, Hash, MaxSize, PartialEq, Serialize)]
pub struct CompositeValveState {
    /// Commanded state of the valve, according to software.
    pub commanded: ValveState,
    /// Actual state of the valve, determined using voltage and current measurements.
    pub actual: ValveState,
}
