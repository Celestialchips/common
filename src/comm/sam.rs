use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, str::FromStr};

#[cfg(feature = "rusqlite")]
use rusqlite::{ToSql, types::{ToSqlOutput, Value as SqlValue, ValueRef as SqlValueRef, FromSql, FromSqlResult, FromSqlError}};

/// Every Unit needed to be passed around in communication, mainly for sensor readings.

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MaxSize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    /// Current, in amperes.
    Amps,
    /// Pressure, in pounds per square inch.
    Psi,
    /// Temperature, in Kelvin
    Kelvin,
    /// Force, in pounds.
    Pounds,
    /// Electrical Potential, in volts
    Volts,
}

/// Represents all possible channel types that may be used in a `NodeMapping`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MaxSize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    /// Pressure transducer, formerly known as CurrentLoop, which measures the pressure of fluid.
    CurrentLoop,
    /// The voltage present on a pin connected to a valve
    ValveVoltage,
    /// The current flowing through a pin connected to a valve.
    ValveCurrent,
    /// The voltage on the power rail of the board.
    RailVoltage,
    /// The current flowing through the power rail of the board.
    RailCurrent,
    /// The signal from a load cell, carried by a differential pair.
    DifferentialSignal,
    /// The channel of a resistance thermometer, measuring temperature.
    Rtd,
    /// The channel of a thermocouple, measuring temperature.
    Tc,
}



