use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::Encode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manual {
    pub pages: Vec<Page>,
}

impl Encode<'_> for Manual {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    /// The page name as appears in the Table of Contents.
    pub title: String,
    /// Show the page only when the given badge is unlocked.
    pub badge: Option<u8>,
    /// Show the page only when the given score is reached on the given board.
    pub score: Option<(u8, i16)>,
    /// Encoded color scheme.
    ///
    /// It's encoded the same way as in [Settings][crate::Settings].
    pub theme: Option<u32>,
    pub content: Vec<Block>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Block {
    /// The biggest header after the page title.
    H2(String),
    /// The smallest header.
    H3(String),
    /// Paragraph.
    P(Paragraph),
    /// Ordered (numbered) list item.
    Oli(Paragraph),
    /// Unrdered list item.
    Uli(Paragraph),
    /// Link to another page.
    A(u8),
    /// Block image, centered.
    Img(String),
    /// Block quote.
    Quote(Paragraph),
    /// QR code.
    Qr(String),
}

pub type Paragraph = Vec<Inline>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inline {
    pub kind: InlineKind,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InlineKind {
    /// Plain text.
    Plain,
    /// Bold text.
    Bold,
    /// Italic text.
    Italic,
    /// Inline image.
    Image,
    /// Inline icon image.
    Icon,
    /// Line break.
    Br,
}
