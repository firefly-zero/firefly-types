/// Wi-Fi connection status.
#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    /// Failed to connect to (or was disconnected from) an Access Point.
    Disconnected(DisconnectReason),
    /// Wifi peripheral is started but not doing anything.
    Started,
    /// Wifi peripheral is not started (or was stopped).
    Stopped,
    /// Connected to the Access Point, obtaining IP address.
    Initializing,
    /// IP address is obtained, ready to go.
    Connected,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            252 => Self::Started,
            253 => Self::Stopped,
            254 => Self::Initializing,
            255 => Self::Connected,
            _ => Self::Disconnected(DisconnectReason::from(value)),
        }
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        match value {
            Status::Disconnected(r) => r.into(),
            Status::Started => 252,
            Status::Stopped => 253,
            Status::Initializing => 254,
            Status::Connected => 255,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DisconnectReason {
    /// Unspecified reason
    Unspecified,
    /// Authentication expired
    AuthExpire,
    /// Deauthentication due to leaving
    AuthLeave,
    /// Disassociated due to inactivity
    DisassocDueToInactivity,
    /// Too many associated stations
    AssocToomany,
    /// Class 2 frame received from nonauthenticated STA
    Class2FrameFromNonauthSta,
    /// Class 3 frame received from nonassociated STA
    Class3FrameFromNonassocSta,
    /// Deassociated due to leaving
    AssocLeave,
    /// Association but not authenticated
    AssocNotAuthed,
    /// Disassociated due to poor power capability
    DisassocPwrcapBad,
    /// Disassociated due to unsupported channel
    DisassocSupchanBad,
    /// Disassociated due to BSS transition
    BssTransitionDisassoc,
    /// Invalid Information Element (IE)
    IeInvalid,
    /// MIC failure
    MicFailure,
    /// 4-way handshake timeout
    FourWayHandshakeTimeout,
    /// Group key update timeout
    GroupKeyUpdateTimeout,
    /// IE differs in 4-way handshake
    IeIn4wayDiffers,
    /// Invalid group cipher
    GroupCipherInvalid,
    /// Invalid pairwise cipher
    PairwiseCipherInvalid,
    /// Invalid AKMP
    AkmpInvalid,
    /// Unsupported RSN IE version
    UnsuppRsnIeVersion,
    /// Invalid RSN IE capabilities
    InvalidRsnIeCap,
    /// 802.1X authentication failed
    AuthFailed,
    /// Cipher suite rejected
    CipherSuiteRejected,
    /// TDLS peer unreachable
    TdlsPeerUnreachable,
    /// TDLS unspecified
    TdlsUnspecified,
    /// SSP requested disassociation
    SspRequestedDisassoc,
    /// No SSP roaming agreement
    NoSspRoamingAgreement,
    /// Bad cipher or AKM
    BadCipherOrAkm,
    /// Not authorized in this location
    NotAuthorizedThisLocation,
    /// Service change precludes TS
    ServiceChangePercludesTs,
    /// Unspecified Quality of Service reason
    UnspecifiedQos,
    /// Not enough bandwidth
    NotEnoughBandwidth,
    /// Missing ACKs
    MissingAcks,
    /// Exceeded TXOP
    ExceededTxop,
    /// Station leaving
    StaLeaving,
    /// End of Block Ack (BA)
    EndBa,
    /// Unknown Block Ack (BA)
    UnknownBa,
    /// Timeout
    Timeout,
    /// Peer initiated disassociation
    PeerInitiated,
    /// AP initiated disassociation
    ApInitiated,
    /// Invalid FT action frame count
    InvalidFtActionFrameCount,
    /// Invalid PMKID
    InvalidPmkid,
    /// Invalid MDE
    InvalidMde,
    /// Invalid FTE
    InvalidFte,
    /// Transmission link establishment failed
    TransmissionLinkEstablishFailed,
    /// Alternative channel occupied
    AlterativeChannelOccupied,

    /// Beacon timeout
    BeaconTimeout,
    /// No AP found
    NoApFound,
    /// Authentication failed
    AuthFail,
    /// Association failed
    AssocFail,
    /// Handshake timeout
    HandshakeTimeout,
    /// Connection failed
    ConnectionFail,
    /// AP TSF reset
    ApTsfReset,
    /// Roaming
    Roaming,
    /// Association comeback time too long
    AssocComebackTimeTooLong,
    /// SA query timeout
    SaQueryTimeout,
    /// No AP found with compatible security
    NoApFoundWCompatibleSecurity,
    /// No AP found in auth mode threshold
    NoApFoundInAuthmodeThreshold,
    /// No AP found in RSSI threshold
    NoApFoundInRssiThreshold,
}

impl DisconnectReason {
    /// Human-readable (but technical) message.
    ///
    /// The first letter is lowercase unless the first word is an abbreviation.
    /// No punctuation at the end.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified reason",
            Self::AuthExpire => "authentication expired",
            Self::AuthLeave => "deauthentication due to leaving",
            Self::DisassocDueToInactivity => "disassociated due to inactivity",
            Self::AssocToomany => "too many associated stations",
            Self::Class2FrameFromNonauthSta => "class 2 frame received from nonauthenticated STA",
            Self::Class3FrameFromNonassocSta => "class 3 frame received from nonassociated STA",
            Self::AssocLeave => "deassociated due to leaving",
            Self::AssocNotAuthed => "association but not authenticated",
            Self::DisassocPwrcapBad => "disassociated due to poor power capability",
            Self::DisassocSupchanBad => "disassociated due to unsupported channel",
            Self::BssTransitionDisassoc => "disassociated due to BSS transition",
            Self::IeInvalid => "invalid Information Element (IE)",
            Self::MicFailure => "MIC failure",
            Self::FourWayHandshakeTimeout => "4-way handshake timeout",
            Self::GroupKeyUpdateTimeout => "group key update timeout",
            Self::IeIn4wayDiffers => "IE differs in 4-way handshake",
            Self::GroupCipherInvalid => "invalid group cipher",
            Self::PairwiseCipherInvalid => "invalid pairwise cipher",
            Self::AkmpInvalid => "invalid AKMP",
            Self::UnsuppRsnIeVersion => "unsupported RSN IE version",
            Self::InvalidRsnIeCap => "invalid RSN IE capabilities",
            Self::AuthFailed => "802.1X authentication failed",
            Self::CipherSuiteRejected => "cipher suite rejected",
            Self::TdlsPeerUnreachable => "TDLS peer unreachable",
            Self::TdlsUnspecified => "TDLS unspecified",
            Self::SspRequestedDisassoc => "SSP requested disassociation",
            Self::NoSspRoamingAgreement => "no SSP roaming agreement",
            Self::BadCipherOrAkm => "bad cipher or AKM",
            Self::NotAuthorizedThisLocation => "not authorized in this location",
            Self::ServiceChangePercludesTs => "service change precludes TS",
            Self::UnspecifiedQos => "unspecified Quality of Service reason",
            Self::NotEnoughBandwidth => "not enough bandwidth",
            Self::MissingAcks => "missing ACKs",
            Self::ExceededTxop => "exceeded TXOP",
            Self::StaLeaving => "station leaving",
            Self::EndBa => "end of Block Ack (BA)",
            Self::UnknownBa => "unknown Block Ack (BA)",
            Self::Timeout => "timeout",
            Self::PeerInitiated => "peer initiated disassociation",
            Self::ApInitiated => "AP initiated disassociation",
            Self::InvalidFtActionFrameCount => "invalid FT action frame count",
            Self::InvalidPmkid => "invalid PMKID",
            Self::InvalidMde => "invalid MDE",
            Self::InvalidFte => "invalid FTE",
            Self::TransmissionLinkEstablishFailed => "transmission link establishment failed",
            Self::AlterativeChannelOccupied => "alternative channel occupied",

            Self::BeaconTimeout => "beacon timeout",
            Self::NoApFound => "no AP found",
            Self::AuthFail => "authentication failed",
            Self::AssocFail => "association failed",
            Self::HandshakeTimeout => "handshake timeout",
            Self::ConnectionFail => "connection failed",
            Self::ApTsfReset => "AP TSF reset",
            Self::Roaming => "roaming",
            Self::AssocComebackTimeTooLong => "association comeback time too long",
            Self::SaQueryTimeout => "SA query timeout",
            Self::NoApFoundWCompatibleSecurity => "no AP found with compatible security",
            Self::NoApFoundInAuthmodeThreshold => "no AP found in auth mode threshold",
            Self::NoApFoundInRssiThreshold => "no AP found in RSSI threshold",
        }
    }
}

impl From<u8> for DisconnectReason {
    fn from(value: u8) -> Self {
        // https://github.com/espressif/esp-idf/blob/master/components/esp_wifi/include/esp_wifi_types_generic.h
        match value {
            // 1 => Self::Unspecified,
            2 => Self::AuthExpire,
            3 => Self::AuthLeave,
            4 => Self::DisassocDueToInactivity,
            5 => Self::AssocToomany,
            6 => Self::Class2FrameFromNonauthSta,
            7 => Self::Class3FrameFromNonassocSta,
            8 => Self::AssocLeave,
            9 => Self::AssocNotAuthed,
            10 => Self::DisassocPwrcapBad,
            11 => Self::DisassocSupchanBad,
            12 => Self::BssTransitionDisassoc,
            13 => Self::IeInvalid,
            14 => Self::MicFailure,
            15 => Self::FourWayHandshakeTimeout,
            16 => Self::GroupKeyUpdateTimeout,
            17 => Self::IeIn4wayDiffers,
            18 => Self::GroupCipherInvalid,
            19 => Self::PairwiseCipherInvalid,
            20 => Self::AkmpInvalid,
            21 => Self::UnsuppRsnIeVersion,
            22 => Self::InvalidRsnIeCap,
            23 => Self::AuthFailed,
            24 => Self::CipherSuiteRejected,
            25 => Self::TdlsPeerUnreachable,
            26 => Self::TdlsUnspecified,
            27 => Self::SspRequestedDisassoc,
            28 => Self::NoSspRoamingAgreement,
            29 => Self::BadCipherOrAkm,
            30 => Self::NotAuthorizedThisLocation,
            31 => Self::ServiceChangePercludesTs,
            32 => Self::UnspecifiedQos,
            33 => Self::NotEnoughBandwidth,
            34 => Self::MissingAcks,
            35 => Self::ExceededTxop,
            36 => Self::StaLeaving,
            37 => Self::EndBa,
            38 => Self::UnknownBa,
            39 => Self::Timeout,
            46 => Self::PeerInitiated,
            47 => Self::ApInitiated,
            48 => Self::InvalidFtActionFrameCount,
            49 => Self::InvalidPmkid,
            50 => Self::InvalidMde,
            51 => Self::InvalidFte,
            67 => Self::TransmissionLinkEstablishFailed,
            68 => Self::AlterativeChannelOccupied,
            200 => Self::BeaconTimeout,
            201 => Self::NoApFound,
            202 => Self::AuthFail,
            203 => Self::AssocFail,
            204 => Self::HandshakeTimeout,
            205 => Self::ConnectionFail,
            206 => Self::ApTsfReset,
            207 => Self::Roaming,
            208 => Self::AssocComebackTimeTooLong,
            209 => Self::SaQueryTimeout,
            210 => Self::NoApFoundWCompatibleSecurity,
            211 => Self::NoApFoundInAuthmodeThreshold,
            212 => Self::NoApFoundInRssiThreshold,
            _ => Self::Unspecified,
        }
    }
}

impl From<DisconnectReason> for u8 {
    fn from(value: DisconnectReason) -> Self {
        match value {
            DisconnectReason::Unspecified => 1,
            DisconnectReason::AuthExpire => 2,
            DisconnectReason::AuthLeave => 3,
            DisconnectReason::DisassocDueToInactivity => 4,
            DisconnectReason::AssocToomany => 5,
            DisconnectReason::Class2FrameFromNonauthSta => 6,
            DisconnectReason::Class3FrameFromNonassocSta => 7,
            DisconnectReason::AssocLeave => 8,
            DisconnectReason::AssocNotAuthed => 9,
            DisconnectReason::DisassocPwrcapBad => 10,
            DisconnectReason::DisassocSupchanBad => 11,
            DisconnectReason::BssTransitionDisassoc => 12,
            DisconnectReason::IeInvalid => 13,
            DisconnectReason::MicFailure => 14,
            DisconnectReason::FourWayHandshakeTimeout => 15,
            DisconnectReason::GroupKeyUpdateTimeout => 16,
            DisconnectReason::IeIn4wayDiffers => 17,
            DisconnectReason::GroupCipherInvalid => 18,
            DisconnectReason::PairwiseCipherInvalid => 19,
            DisconnectReason::AkmpInvalid => 20,
            DisconnectReason::UnsuppRsnIeVersion => 21,
            DisconnectReason::InvalidRsnIeCap => 22,
            DisconnectReason::AuthFailed => 23,
            DisconnectReason::CipherSuiteRejected => 24,
            DisconnectReason::TdlsPeerUnreachable => 25,
            DisconnectReason::TdlsUnspecified => 26,
            DisconnectReason::SspRequestedDisassoc => 27,
            DisconnectReason::NoSspRoamingAgreement => 28,
            DisconnectReason::BadCipherOrAkm => 29,
            DisconnectReason::NotAuthorizedThisLocation => 30,
            DisconnectReason::ServiceChangePercludesTs => 31,
            DisconnectReason::UnspecifiedQos => 32,
            DisconnectReason::NotEnoughBandwidth => 33,
            DisconnectReason::MissingAcks => 34,
            DisconnectReason::ExceededTxop => 35,
            DisconnectReason::StaLeaving => 36,
            DisconnectReason::EndBa => 37,
            DisconnectReason::UnknownBa => 38,
            DisconnectReason::Timeout => 39,
            DisconnectReason::PeerInitiated => 46,
            DisconnectReason::ApInitiated => 47,
            DisconnectReason::InvalidFtActionFrameCount => 48,
            DisconnectReason::InvalidPmkid => 49,
            DisconnectReason::InvalidMde => 50,
            DisconnectReason::InvalidFte => 51,
            DisconnectReason::TransmissionLinkEstablishFailed => 67,
            DisconnectReason::AlterativeChannelOccupied => 68,
            DisconnectReason::BeaconTimeout => 200,
            DisconnectReason::NoApFound => 201,
            DisconnectReason::AuthFail => 202,
            DisconnectReason::AssocFail => 203,
            DisconnectReason::HandshakeTimeout => 204,
            DisconnectReason::ConnectionFail => 205,
            DisconnectReason::ApTsfReset => 206,
            DisconnectReason::Roaming => 207,
            DisconnectReason::AssocComebackTimeTooLong => 208,
            DisconnectReason::SaQueryTimeout => 209,
            DisconnectReason::NoApFoundWCompatibleSecurity => 210,
            DisconnectReason::NoApFoundInAuthmodeThreshold => 211,
            DisconnectReason::NoApFoundInRssiThreshold => 212,
        }
    }
}
