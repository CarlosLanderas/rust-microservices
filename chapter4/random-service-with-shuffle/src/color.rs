use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Serializer};
use std::string::ToString;

pub const WHITE: Color = Color { red: 0xFF, green: 0xFF, blue: 0xFF};
pub const BLACK: Color = Color { red: 0x00, green: 0x00, blue: 0x00};

#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          &WHITE => f.write_str("white"),
          &BLACK => f.write_str("black"),
          color => {
              write!(f, "#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
          }
      }
  }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}



