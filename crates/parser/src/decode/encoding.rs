use alloc::borrow::Cow;
use core::str::from_utf8;

use crate::decode::Decode;
use crate::decode::Decoder;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::types::Slice;

const BOM_BE: &[u8] = &[0xFE, 0xFF];
const BOM_LE: &[u8] = &[0xFF, 0xFE];

// =============================================================================
// String Encoding
// =============================================================================

/// Valid types of string encoding.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Encoding {
  /// ISO-8859-1 encoding.
  Latin1 = 0x00,
  /// UTF-16 encoding with BOM.
  Utf16 = 0x01,
  /// UTF-16 encoding (BE).
  Utf16BE = 0x02,
  /// UTF-8 encoding.
  Utf8 = 0x03,
}

impl Encoding {
  pub(crate) fn decode<'a>(self, decoder: &mut Decoder<'a>) -> Result<Cow<'a, str>> {
    match self {
      Encoding::Latin1 => Ok(decode_latin1(decoder.until_nul())),
      Encoding::Utf16 => decode_utf16_bom(decoder.until_nul2()),
      Encoding::Utf16BE => decode_utf16_be(decoder.until_nul2()),
      Encoding::Utf8 => decode_utf8(decoder.until_nul()),
    }
  }
}

impl Decode<'_> for Encoding {
  fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
    let this: Self = match u8::decode(decoder)? {
      0x00 => Self::Latin1,
      0x01 => Self::Utf16,
      0x02 => Self::Utf16BE,
      0x03 => Self::Utf8,
      _ => return Err(Error::new(ErrorKind::InvalidFrameData)),
    };

    // Change the internal text format if another encoding is encountered.
    decoder.set_format(this);

    Ok(this)
  }
}

copy_into_owned!(Encoding);

fn decode_latin1(slice: &Slice) -> Cow<'_, str> {
  // Note: Only the ASCII subset of ISO-8859-1 has the same representation in
  //       UTF-8 and can be borrowed. Bytes 0x80-0xFF map to the Unicode code
  //       points U+0080-U+00FF, which are two bytes wide in UTF-8.
  match from_utf8(slice.as_ref()) {
    Ok(text) if text.is_ascii() => Cow::Borrowed(text),
    // TODO: Should probably return an error for control characters.
    _ => Cow::Owned(slice.iter().copied().map(char::from).collect()),
  }
}

fn decode_utf16_bom(slice: &Slice) -> Result<Cow<'_, str>> {
  // Note: Empty strings are commonly written without a BOM.
  if slice.is_empty() {
    return Ok(Cow::Borrowed(""));
  }

  match slice.as_ref().get(..2) {
    Some(BOM_BE) => decode_utf16_be(&slice[2..]),
    Some(BOM_LE) => decode_utf16_le(&slice[2..]),
    _ => Err(Error::new(ErrorKind::InvalidFrameData)),
  }
}

fn decode_utf16_be(slice: &Slice) -> Result<Cow<'static, str>> {
  decode_utf16(slice, u16::from_be_bytes)
}

fn decode_utf16_le(slice: &Slice) -> Result<Cow<'static, str>> {
  decode_utf16(slice, u16::from_le_bytes)
}

fn decode_utf16<F>(slice: &Slice, convert: F) -> Result<Cow<'static, str>>
where
  F: Fn([u8; 2]) -> u16,
{
  // TODO: Would be nice to use array_chunks::<2> here
  //
  // https://github.com/rust-lang/rust/issues/74985
  let iter = slice
    .as_ref()
    .chunks_exact(2)
    .map(|chunk| convert(chunk.try_into().unwrap()));

  let mut output: String = String::with_capacity(slice.len() >> 1);

  // TODO: Use collect() when applicable
  //
  // https://github.com/rust-lang/rust/issues/48994
  for ch in char::decode_utf16(iter) {
    match ch {
      Ok(ch) => {
        output.push(ch);
      }
      Err(error) => {
        return Err(Error::new_std(ErrorKind::InvalidFrameData, error));
      }
    }
  }

  Ok(Cow::Owned(output))
}

fn decode_utf8(slice: &Slice) -> Result<Cow<'_, str>> {
  from_utf8(slice.as_ref())
    .map(Cow::Borrowed)
    .map_err(|error| Error::new_std(ErrorKind::InvalidFrameData, error))
}
