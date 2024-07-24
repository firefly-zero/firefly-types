//! Serialization for messages send over USB ("serial port").
//!
//! These messages are used for interacting with a running device:
//! getting runtime stats, executing cheats and admin commands, etc.

/// Messages that clients (desktop app, CLI, etc) send into runtime (device or emulator).
pub enum Request {
    /// Call the `cheat` callback with the given two arguments.
    ///
    /// It's up to the app how to handle the passed values,
    /// but the most common practice is to treat the first value as the command
    /// to execute (for exmaple, 42 for "noclip") and the second value as
    /// the command argument (for example, 0 for "disable" and 1 for "enable").
    Cheat(i32, i32),
}

/// Messages that the runtime (device or emulator) sends to connected clients.
pub enum Response {
    /// The value returned by the `cheat` callback.
    Cheat(i32),
    /// Instructions executed by a callback.
    Fuel(Callback, Fuel),
    /// CPU time spent running code vs sleeping.
    CPU(CPU),
    /// Linear memory used by the wasm app.
    Memory(Memory),
}

pub enum Callback {
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
pub struct Fuel {
    /// The least fuel consumed by a single run.
    pub min: u32,

    /// The most fuel consumed by a single run.
    pub max: u32,

    /// The average number of instructions executed per run.
    pub avg: u32,

    /// Standard deviation of individual runs from the average.
    ///
    /// Lower value means more consistent CPU load. Higher values mean
    /// that some runs are fast and some runs are slow.
    pub stdev: f32,

    /// The number of runs of the given callback on the observed interval.
    pub calls: u32,
}

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
}

pub struct CPU {
    /// The time taken running the app.
    ///
    /// Includes executing wasm callbacks, wasm host functions,
    /// rendering frame buffer on the screen, syncing network code, etc.
    /// Basically, everything except when the main thread is sleeping.
    pub busy_ns: u32,

    /// The total duration of the observed interval, in nanoseconds.
    pub total_ns: u32,
}
