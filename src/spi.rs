use crate::encode::Encode;
use serde::{Deserialize, Serialize};

/// Request that the main chip sends to the IO chip.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Request<'a> {
    /// Start listening for messages.
    NetStart,
    /// Stop accepting new messages and connections.
    NetStop,
    /// Get MAC address of this device's IO chip.
    NetLocalAddr,
    /// Broadcast advertisement message.
    NetAdvertise,
    /// Read an incoming message (if any) from the IO chip.
    NetRecv,
    /// Send an outgoing message to the IO chip.
    NetSend([u8; 6], &'a [u8]),
    /// Get the latest touchpad and buttons inputs.
    ReadInput,
}

impl<'a> Encode<'a> for Request<'a> {}

/// Response that the IO chip sends back to the main chip.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Response<'a> {
    NetError(u32),
    NetStarted,
    NetStopped,
    NetLocalAddr([u8; 6]),
    NetAdvertised,
    NetIncoming([u8; 6], &'a [u8]),
    NetNoIncoming,
    NetSent,
    Input(Option<(i16, i16)>, u8),
}

impl<'a> Encode<'a> for Response<'a> {}
