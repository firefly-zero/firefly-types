use crate::encode::Encode;
use serde::{Deserialize, Serialize};

/// Request that the main chip sends to the IO chip.
#[derive(Serialize, Deserialize, Clone, Debug)]
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

    /// Scan the air for available non-hidden wifi access points.
    WifiScan,

    /// Connect to an access point using the given SSID and password.
    ///
    /// Async. Check [`Request::WifiStatus`] to see if the device is actually connected.
    WifiConnect(&'a str, &'a str),

    /// Get the current connection status to an AP.
    ///
    /// Since [`Request::WifiConnect`] is async, make sure to account for races.
    /// For instance, shortly after requesting a connect, the status might
    /// still indicate the status of the previous connection.
    WifiStatus,

    /// Disconnect from the currently connected wifi access point.
    WifiDisconnect,

    /// Connect to the TCP server with the given IP address and port number.
    ///
    /// There can be only one open TCP connection at a time.
    TcpConnect(u32, u16),

    /// Fetch the state of the currently open TCP connection.
    TcpStatus,

    /// Send the given bytes into the currently open TCP connection.
    TcpSend(&'a [u8]),

    /// Read a bytes chunk from the currently open TCP connection.
    TcpRecv,

    /// Close the currently open TCP connection.
    TcpClose,
}

impl<'a> Encode<'a> for Request<'a> {}

/// Response that the IO chip sends back to the main chip.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Response<'a> {
    Error(&'a str),

    /// Confirmation for [`Request::NetStart`].
    NetStarted,
    /// Confirmation for [`Request::NetStop`].
    NetStopped,
    /// Response for [`Request::NetLocalAddr`].
    NetLocalAddr([u8; 6]),
    /// Confirmation for [`Request::NetAdvertise`].
    NetAdvertised,
    NetIncoming([u8; 6], &'a [u8]),
    NetNoIncoming,
    /// Confirmation for [`Request::NetSend`].
    NetSent,
    /// Response for [`Request::NetSendStatus`].
    NetSendStatus(SendStatus),

    /// Response for [`Request::ReadInput`].
    ///
    /// The first item is the touch coordinates on the pad (if any).
    /// The second is serialized bitflags of pressed buttons.
    Input(Option<(u16, u16)>, u8),

    /// List of SSIDs of up to 6 available wifi Access Points.
    ///
    /// Includes 6 of the first detected APs.
    /// So, it's not the top closest APs but the closer AP
    /// the higher its chance to make it to the list.
    ///
    /// SSID is up to 30 bytes. 6 SSIDs take up to 180 bits.
    /// The SPI packet size is limited to 255 bits
    /// because we use a single byte to transfer the packet size.
    WifiScan([&'a str; 6]),

    /// The status of current connection to a wifi AP.
    ///
    /// For firefly-types and firefly-runtime, it's opaque.
    /// It doesn't know which integer corresponds to which state.
    /// The status is encoded in firefly-io and decoded in firefly-installer.
    /// firefly-hal is also aware of status encoding, though, for the purpose
    /// of mocking wifi on the hosted environment (emulator).
    WifiStatus(u8),

    /// Confirmation for [`Request::WifiConnect`].
    ///
    /// The connection request is async, so the response doesn't mean
    /// that the device is actually connected to the AP (yet).
    /// Use [`Request::WifiStatus`] to get the actual connection status.
    WifiConnected,

    /// Confirmation for [`Request::WifiDisconnect`].
    WifiDisconnected,
    /// Confirmation for [`Request::TcpConnect`].
    TcpConnected,
    /// Response for [`Request::TcpStatus`].
    TcpStatus(u8),
    /// Confirmation for [`Request::TcpSend`].
    TcpSent,
    /// Response for [`Request::TcpRecv`].
    TcpChunk(&'a [u8]),
    /// Confirmation for [`Request::TcpClose`].
    TcpClosed,
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
