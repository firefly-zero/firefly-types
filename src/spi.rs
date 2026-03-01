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

    /// Scan the air for available non-hidden Wi-Fi access points.
    WifiScan,
    /// Connect to an access point using the given SSID and password.
    WifiConnect(&'a str, &'a str),
    WifiStatus,
    /// Disconnect from the currently connected Wi-Fi access point.
    WifiDisconnect,
    /// Connect to the TCP server with the given IP address and port number.
    TcpConnect(u32, u16),
    /// Fetch the state of the currently open TCP connection.
    TcpStatus,
    TcpSend(&'a [u8]),
    // TcpRead,
    /// Close the currently active TCP connection.
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
    /// Respnse for [`Request::NetLocalAddr`].
    NetLocalAddr([u8; 6]),
    /// Confirmation for [`Request::NetAdvertise`].
    NetAdvertised,
    NetIncoming([u8; 6], &'a [u8]),
    NetNoIncoming,
    /// Confirmation for [`Request::NetSend`].
    NetSent,
    /// Respnse for [`Request::NetSendStatus`].
    NetSendStatus(SendStatus),

    Input(Option<(u16, u16)>, u8),

    /// List of SSIDs of top 6 available Wi-Fi Access Points.
    ///
    /// SSID is up to 30 bytes. 6 SSIDs take up to 180 bits.
    /// The SPI packet size is limited to 255 bits
    /// because we use a single byte to transfer the packet size.
    WifiScan([&'a str; 6]),
    WifiStatus(u8),
    WifiConnected,
    WifiDisconnected,

    TcpConnected,
    TcpStatus(u8),
    TcpSent,
    TcpClosed,
    TcpRespStart(u16),
    TcpRespChunk(&'a [u8]),
    TcpRespEnd,
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
