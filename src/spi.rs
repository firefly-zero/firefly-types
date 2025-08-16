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
    /// Get send status of the previous message for the peer.
    NetSendStatus([u8; 6]),
    /// Get the latest touchpad and buttons inputs.
    ReadInput,
}

impl<'a> Encode<'a> for Request<'a> {}

/// Response that the IO chip sends back to the main chip.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Response<'a> {
    Error(&'a str),
    NetStarted,
    NetStopped,
    NetLocalAddr([u8; 6]),
    NetAdvertised,
    NetIncoming([u8; 6], &'a [u8]),
    NetNoIncoming,
    NetSent,
    NetSendStatus(SendStatus),
    Input(Option<(u16, u16)>, u8),
}

impl<'a> Encode<'a> for Response<'a> {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SendStatus {
    /// Trying to send the message. The value is the number of attempts so far.
    Sending(u8),
    /// Message is delivered. The value is the number of attempts that it took.
    Delivered(u8),
    /// Message delivery failed.
    Failed,
    /// No messages were sent to the peer.
    Empty,
}
