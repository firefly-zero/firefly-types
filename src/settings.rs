use crate::encode::Encode;
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};

/// System settings. Stored in `sys/config`.
///
/// Since we don't have a realiable vesioning for the system config,
/// some of the settings are added "just in case" and might be not used yet
/// or maybe even won't be ever used.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    /// How much XP the player earned over all games.
    pub xp: u32,

    /// How many badges the player earned over all games.
    pub badges: u32,

    /// A two-letter ASCII ISO 639 Set 1 language code.
    pub lang: [u8; 2],

    /// The device name. Randomly generated when creating vFS.
    pub name: String,

    /// The full timezone name as in the IANA database.
    pub timezone: String,

    /// If true, rotate the image on the screen 180 degrees.
    pub rotate_screen: bool,

    /// Brightness of the screen backlight.
    pub screen_brightness: u8,

    /// Brightness of the LEDs.
    pub leds_brightness: u8,

    /// How loud the speakers should play sounds.
    pub speakers_volume: u8,

    /// How loud the headphones should play sounds.
    pub headphones_volume: u8,

    /// The preferred font size (text character height) in pixels.
    pub font_size: u8,

    /// Color scheme to use.
    pub theme: u32,

    /// Automatically lock the screen after N minutes.
    ///
    /// If zero, never locks automatically.
    /// The screen is never locked when in multiplayer.
    pub auto_lock: u8,

    /// If enabled, apps are advised to skip flashy animations.
    pub reduce_flashing: bool,

    /// If enabled, collect and send anonymous telemetry.
    pub telemetry: bool,

    /// Emulate gamepad when connecting Firefly Zero to a PC via USB.
    pub gamepad_mode: bool,

    /// Increase contrast of colors in the default color palette.
    pub contrast: bool,

    /// Let the system apps show easter eggs, holiday effects, and weird jokes.
    pub easter_eggs: bool,

    /// Any feature new fields will be encoded in this field, for backward compatibility.
    pub extra_flags: u32,
}

impl Encode<'_> for Settings {}

impl Default for Settings {
    fn default() -> Self {
        /// * Primary:      Black       (0).
        /// * Secondary:    Light Gray  (D).
        /// * Accent:       Green       (6).
        /// * Background:   White       (C).
        /// * Index:        default     (00).
        const DEFAULT_THEME: u32 = 0x_0D_6C_00;

        Self {
            xp: 0,
            badges: 0,
            lang: [b'e', b'n'],
            name: "firefly-zero".to_string(),
            timezone: "Europe/Amsterdam".to_string(),
            rotate_screen: false,
            screen_brightness: 255,
            leds_brightness: 255,
            speakers_volume: 64,
            headphones_volume: 64,
            font_size: 9,
            theme: DEFAULT_THEME,
            auto_lock: 5,
            reduce_flashing: false,
            telemetry: false,
            gamepad_mode: false,
            contrast: false,
            easter_eggs: false,
            extra_flags: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BatteryInfo {
    pub min_voltage: u16,
    pub max_voltage: u16,
}

impl Encode<'_> for BatteryInfo {}
