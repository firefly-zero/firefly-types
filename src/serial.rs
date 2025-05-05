//! Serialization for messages send over USB ("serial port").
//!
//! These messages are used for interacting with a running device:
//! getting runtime stats, executing cheats and admin commands, etc.
//!
//! Unlike in multiplayer (which is peer-to-peer), this is asymmetric communication.
//! Clients (desktop app, CLI, etc) send [`Request`]s
//! and the runtime (device or emulator) sends back [`Response`]s.
use crate::encode::Encode;
use serde::{Deserialize, Serialize};

/// Messages that clients send into the runtime.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Request {
    /// Call the `cheat` callback with the given two arguments.
    ///
    /// It's up to the app how to handle the passed values,
    /// but the most common practice is to treat the first value as the command
    /// to execute (for exmaple, 42 for "noclip") and the second value as
    /// the command argument (for example, 0 for "disable" and 1 for "enable").
    Cheat(i32, i32),

    /// Turn on/off collection and sending of runtime stats.
    Stats(bool),
}

impl Encode<'_> for Request {}

/// Messages that the runtime sends to connected clients.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Response {
    /// The value returned by the `cheat` callback.
    Cheat(i32),
    /// Instructions executed by a callback.
    Fuel(Callback, Fuel),
    /// CPU time spent running code vs sleeping.
    CPU(CPU),
    /// Linear memory used by the wasm app.
    Memory(Memory),
    /// Log record
    Log(alloc::string::String),
}

impl Encode<'_> for Response {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Callback {
    /// The `boot` wasm callback.
    Boot,
    /// The `update` wasm callback.
    Update,
    /// The `render` wasm callback.
    Render,
    /// The `render_line` wasm callback.
    RenderLine,
    /// The `cheat` wasm callback.
    Cheat,
}

/// The fuel consumed (wasm instructions executed) by a callback on the observed interval.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Fuel {
    /// The least fuel consumed by a single run.
    pub min: u32,

    /// The most fuel consumed by a single run.
    pub max: u32,

    /// The average number of instructions executed per run.
    pub mean: u32,

    /// Squared standard deviation of individual runs from the average.
    ///
    /// Lower value means more consistent CPU load. Higher values mean
    /// that some runs are fast and some runs are slow.
    ///
    /// Take square root to get stdev.
    pub var: f32,

    /// The number of runs of the given callback on the observed interval.
    pub calls: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Memory {
    /// The number of linear memory pages allocated for the app.
    ///
    /// One page size is 64 KB, as defined by the WebAssembly core specification.
    pub pages: u16,

    /// The address of the last byte that isn't zero.
    ///
    /// This roughly corresponds to the actual memory used in the app,
    /// assuming that the allocator tries to use lower address values
    /// and that most of data structures in use aren't all zeroes.
    pub last_one: u32,

    /// The number of read operations.
    ///
    /// Currently unused. Reserved for future use.
    pub reads: u32,

    /// The number of write operations.
    ///
    /// Currently unused. Reserved for future use.
    pub writes: u32,

    /// The maximum memory that can be allocated.
    ///
    /// Currently unused. Reserved for future use.
    pub max: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CPU {
    /// The time taken running the app.
    ///
    /// Includes executing wasm callbacks, wasm host functions,
    /// rendering frame buffer on the screen, syncing network code, etc.
    /// Basically, everything except when the main thread is sleeping.
    ///
    /// Lower is better.
    pub busy_ns: u32,

    /// The time over expected limit taken by updates.
    ///
    /// Lower is better. If this value is not zero, the app will be lagging.
    pub lag_ns: u32,

    /// The total duration of the observed interval, in nanoseconds.
    pub total_ns: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_request() {
        let given = Request::Cheat(3, 4);
        let mut buf = vec![0; given.size()];
        let raw = given.encode_buf(&mut buf).unwrap();
        let actual = Request::decode(raw).unwrap();
        assert_eq!(given, actual);
    }

    #[test]
    fn test_roundtrip_response() {
        let given = Response::Cheat(13);
        let mut buf = vec![0; given.size()];
        let raw = given.encode_buf(&mut buf).unwrap();
        let actual = Response::decode(raw).unwrap();
        assert_eq!(given, actual);
    }
}
