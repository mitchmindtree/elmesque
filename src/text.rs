
use color::{black, Color};
use std::path::PathBuf;


/// Drawable Text.
#[derive(Clone, Debug)]
pub struct Text {
    pub sequence: Vec<TextUnit>,
    pub position: Position,
}


#[derive(Clone, Debug)]
pub struct TextUnit {
    pub string: String,
    pub style: Style,
}

/// Styles for lines on text. This allows you to add an underline, an overline, or strike out text.
#[derive(Copy, Clone, Debug)]
pub enum Line {
    Under,
    Over,
    Through,
}

/// Text position relative to center point
#[derive(Copy, Clone, Debug)]
pub enum Position {
    Center,
    ToLeft,
    ToRight
}


/// Represents all the ways you can style `Text`. If the `type_face` list is empty or the `height`
/// is `None`, the users will fall back on their default settings. The following `Style` is black,
/// 16 pixel tall, underlined, and Times New Roman (assuming that typeface is available on the
/// user's computer):
///
///   Style {
///       type_face: Some("Times New Roman"),
///       height: Some(16),
///       color: black(),
///       bold: false,
///       italic: false,
///       line: Some(Line::Under),
///   }
///
#[derive(Clone, Debug)]
pub struct Style {
    pub typeface: Option<PathBuf>,
    pub height: Option<f64>,
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
    pub line: Option<Line>,
    pub monospace: bool,
}

impl Style {
    pub fn default() -> Style {
        Style {
            typeface: None,
            height: None,
            color: black(),
            bold: false,
            italic: false,
            line: None,
            monospace: false,
        }
    }
}


impl Text {

    /// Convert a string into text which can be styled and displayed.
    pub fn from_string(string: String) -> Text {
        Text {
            sequence: vec![TextUnit { string: string, style: Style::default(), }],
            position: Position::Center
        }
    }

    /// Text with nothing in it.
    pub fn empty() -> Text {
        Text::from_string("".to_string())
    }

    /// Put two chunks of text together.
    #[inline]
    pub fn append(mut self, other: Text) -> Text {
        self.sequence.extend(other.sequence.into_iter());
        self
    }

    /// Put many chunks of text together.
    pub fn concat(texts: Vec<Text>) -> Text {
        let position = texts.get(0).map(|t| t.position).unwrap_or(Position::Center);
        Text {
            sequence: texts.into_iter()
                .flat_map(|Text { sequence, position }| sequence.into_iter())
                .collect(),
            position: position
        }
    }

    /// Put many chunks of text together with a separator.
    pub fn join(separator: Text, texts: Vec<Text>) -> Text {
        texts.into_iter().fold(Text::empty(), |texts, text| {
            texts.append(text).append(separator.clone())
        })
    }

    /// Set the style of some text. For example, if you design a `Style` called `foorter_style` that is
    /// specifically for the bottom of your page, you could apply it to text like this:
    ///
    ///   style(footer_style, from_string("the old prince / 2007"))
    ///
    #[inline]
    pub fn style(self, style: Style) -> Text {
        let string = String::from_utf8(self.sequence.into_iter().flat_map(|unit| {
            unit.string.into_bytes().into_iter()
        }).collect()).unwrap();
        Text {
            sequence: vec![TextUnit { string: string, style: style }],
            ..self
        }
    }

    /// Provide a path of a typeface to be used for some text.
    #[inline]
    pub fn typeface(mut self, path: PathBuf) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.typeface = Some(path.clone());
        }
        self
    }

    /// Switch to a monospace typeface. Good for code snippets.
    ///
    ///   monospace(from_string("(0..3).fold(0, |a, b| a + b)"))
    ///
    #[inline]
    pub fn monospace(mut self) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.monospace = true;
        }
        self
    }

    /// Set the height of some text in pixels.
    #[inline]
    pub fn height(mut self, h: f64) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.height = Some(h);
        }
        self
    }

    /// Set the color of some text.
    #[inline]
    pub fn color(mut self, color: Color) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.color = color;
        }
        self
    }

    /// Make the text bold.
    #[inline]
    pub fn bold(mut self) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.bold = true;
        }
        self
    }

    /// Make the text italic.
    #[inline]
    pub fn italic(mut self) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.italic = true;
        }
        self
    }

    /// Put lines on text.
    #[inline]
    pub fn line(mut self, line: Line) -> Text {
        for unit in self.sequence.iter_mut() {
            unit.style.line = Some(line);
        }
        self
    }

    /// Change the text position relative to it's center point
    #[inline]
    pub fn position(mut self, position: Position) -> Text {
        self.position = position;
        self
    }
}

