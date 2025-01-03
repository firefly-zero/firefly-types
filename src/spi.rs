use crate::encode::Encode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Request<'a> {
    NetStart,
    NetStop,
    NetLocalAddr,
    NetAdvertise,
    NetRecv,
    NetSend(&'a [u8]),
    ReadInput,
}

impl<'a> Encode<'a> for Request<'a> {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Response<'a> {
    NetError(u32),
    NetStarted,
    NetStopped,
    NetLocalAddr([u8; 6]),
    NetAdvertised,
    NetIncoming(&'a [u8]),
    NetSent,
    Input(Option<(i16, i16)>, u8),
}

impl<'a> Encode<'a> for Response<'a> {}
